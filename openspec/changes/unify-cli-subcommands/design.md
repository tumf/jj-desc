# 設計: unify-cli-subcommands

## 現状の構造

```
jj-desc
├── generate     # GenerateArgs
│   ├── --dry-run
│   ├── --provider
│   ├── --model
│   ├── --max-tokens
│   ├── --temperature
│   └── --revision      # 単一リビジョン（オプション）
│
└── backfill     # BackfillArgs
    ├── --dry-run
    ├── --provider
    ├── --model
    ├── --max-tokens
    ├── --temperature
    ├── --revisions     # revset（デフォルト: ::@ & mutable()）
    ├── --limit
    └── --interactive
```

## 統合後の構造

```
jj-desc
├── --dry-run
├── --provider
├── --model
├── --max-tokens
├── --temperature
├── -r/--revisions     # revset（デフォルト: ::@ & mutable()）
├── -n/--limit
└── -i/--interactive
```

## 設計判断

### 1. リビジョン指定の統一

**決定**: `--revision` と `--revisions` を `-r/--revisions` に統一

**理由**:
- jj では `-r` が revset を受け付ける標準的なパターン
- 単一コミット（例: `@`）も revset として有効
- ユーザーが「単一か複数か」を意識する必要がなくなる

### 2. デフォルト動作

**決定**: デフォルトは `::@ & mutable()` で空説明のコミットのみ処理

**理由**:
- 最も一般的なユースケース（作業中のコミット群の説明埋め）をカバー
- 既存の backfill デフォルト動作と互換
- 説明がすでにあるコミットは自動スキップ

### 3. 単一コミット強制の廃止

**決定**: `generate` の「単一コミット限定」制約を撤廃

**理由**:
- `-r @` で単一コミットを指定できる
- `-n 1` で処理数を1に制限できる
- 特別な制約は不要

### 4. コマンド実装の統合

**決定**: `execute_backfill` をベースに統合、`execute_generate` は削除

**理由**:
- backfill のロジックが generate を包含
- コード重複の削減
- テストの簡素化

## 移行計画

1. CLI定義の変更（`cli.rs`）
2. コマンド実行ロジックの統合（`commands/mod.rs`）
3. テストの更新
4. ドキュメント更新

## 代替案の検討

### 案A: エイリアス方式
`generate` を `backfill -n 1` のエイリアスとして残す
→ 却下: 複雑さが残る

### 案B: 新サブコマンド `describe`
`jj-desc describe` という統合サブコマンドを追加
→ 却下: サブコマンド自体が冗長
