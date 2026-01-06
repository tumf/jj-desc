# タスク: refactor-diff-result

## 実装タスク

### 1. DiffResult enum の定義
- [ ] `src/jj.rs` に `DiffResult` enum を追加
- [ ] `Content(String)` と `EmptyMerge` バリアントを定義
- [ ] 必要な derive マクロ（Debug, Clone）を追加

### 2. get_diff() の戻り値変更
- [ ] `get_diff()` のシグネチャを `Result<DiffResult, JjDescError>` に変更
- [ ] 空 diff + マージコミットの場合は `Ok(DiffResult::EmptyMerge)` を返す
- [ ] 通常の diff は `Ok(DiffResult::Content(diff))` を返す
- [ ] 空 diff + 非マージの場合は `Err(JjDescError::EmptyDiff)` を返す

### 3. generate.rs の更新
- [ ] `DiffResult` を import
- [ ] `get_diff()` の結果を match で処理
- [ ] `Content(diff)` の場合は LLM に渡す
- [ ] `EmptyMerge` の場合は "Merge commit" を設定
- [ ] 既存の `Err(EmptyDiff)` 分岐での冗長なマージ判定を削除

### 4. backfill.rs の更新
- [ ] `DiffResult` を import
- [ ] `get_diff()` の結果を match で処理
- [ ] `EmptyMerge` の場合は "Merge branches" を設定
- [ ] 既存の `is_merge` 判定ロジックを簡素化

### 5. テストの追加・更新
- [ ] `DiffResult` enum のユニットテスト追加
- [ ] 既存テストが通ることを確認
- [ ] `cargo test` 全体の実行確認

### 6. CI 確認
- [ ] `cargo clippy -- -D warnings` でエラーなし
- [ ] `cargo fmt --check` でフォーマット確認

## 依存関係

なし（独立して実装可能）

## 検証方法

1. ユニットテストの追加・実行
2. 手動テスト: マージコミットに対して `jj-desc generate` を実行
3. dry-run モードで動作確認
