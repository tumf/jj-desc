# Design: backfill-empty-descriptions

## アーキテクチャ

### CLI 構造

```
jj-desc
├── generate (default)  # 既存の単一コミット処理
│   ├── --revision
│   ├── --dry-run
│   └── ...
└── backfill            # 新機能：複数コミット一括処理
    ├── --revisions
    ├── --dry-run
    ├── --limit
    └── --interactive
```

### モジュール設計

```
src/
├── cli.rs              # サブコマンド定義（修正）
├── commands/           # 新規ディレクトリ
│   ├── mod.rs
│   ├── generate.rs     # 既存ロジックを移動
│   └── backfill.rs     # 新規：バックフィルロジック
├── jj.rs               # jj コマンド操作（拡張）
└── ...
```

## 処理フロー

### `jj-desc backfill` の実行フロー

```
1. jj log -r 'description(exact:"") & <revset>' --no-graph -T '{change_id}'
   └─> 空 description のコミット一覧を取得

2. For each commit:
   a. jj diff --revision <change_id>
      └─> diff 取得
   
   b. LLM API 呼び出し
      └─> description 生成
   
   c. --interactive の場合: ユーザー確認
      └─> Accept / Skip / Edit
   
   d. jj describe --revision <change_id> -m '<description>'
      └─> description 設定

3. サマリー表示
   - 成功: X件
   - スキップ: Y件
   - 失敗: Z件
```

## 技術的決定

### 1. jj revset による対象選択

**決定**: `jj log -r 'description(exact:"") & <user-revset>'` を使用

**理由**:
- jj の revset 構文を活用し、柔軟な範囲指定が可能
- `description(exact:"")` で空 description のみを確実にフィルタ
- ユーザーが `mutable()`, `mine()`, `@..main` など自由に指定可能

**代替案**: 
- ❌ すべてのコミットを取得して Rust 側でフィルタ → 非効率

### 2. エラーハンドリング戦略

**決定**: 個別コミットの失敗は記録してスキップ、処理を続行

**理由**:
- 大量コミット処理時に1件の失敗で全体が止まるのを避ける
- 最終的に成功/失敗のサマリーを表示してユーザーに判断を委ねる

**実装**:
```rust
for commit in commits {
    match process_commit(&commit).await {
        Ok(_) => success_count += 1,
        Err(e) => {
            eprintln!("Failed to process {}: {}", commit.id, e);
            failure_count += 1;
        }
    }
}
```

### 3. API レート制限対策

**決定**: オプションで遅延時間を指定可能にする

**提案オプション**: `--delay <milliseconds>` (デフォルト: 0)

**理由**:
- LLM プロバイダーによってはレート制限がある
- ユーザーが必要に応じて調整可能にする

### 4. 進捗表示

**決定**: シンプルなカウンター表示

**実装**:
```
Processing commits: 5/20 (25%)
✓ abc123: Added feature X
✓ def456: Fixed bug Y
✗ ghi789: Failed (API error)
...
```

**代替案**:
- ❌ プログレスバーライブラリ（indicatif など）→ 依存関係増加を避ける

## 後方互換性

### `jj-desc` 単体実行の維持

**要件**: `jj-desc` を引数なしで実行した場合、既存の動作を維持

**実装方針**:
```rust
#[derive(Parser)]
enum Command {
    /// Generate description for a single commit (default)
    #[command(default)]
    Generate(GenerateArgs),
    
    /// Backfill descriptions for multiple commits
    Backfill(BackfillArgs),
}
```

clap の `default` 属性または、引数パース前に検証して自動で `generate` にフォールバックする。

## セキュリティ考慮事項

- diff 内容に機密情報が含まれる可能性 → 既存の `jj-desc` と同じリスク
- 大量 API 呼び出しによるコスト → `--limit`, `--dry-run` で制御可能

## パフォーマンス

- 同期処理（逐次実行）を基本とする
- 将来的な拡張: `--parallel <N>` で並列処理（別提案で対応）
