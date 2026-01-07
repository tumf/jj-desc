# タスク: refactor-diff-result

## 実装タスク

### 1. DiffResult enum の定義
- [x] `src/jj.rs` に `DiffResult` enum を追加
- [x] `Content(String)` と `EmptyMerge` バリアントを定義
- [x] 必要な derive マクロ（Debug, Clone）を追加

### 2. get_diff() の戻り値変更
- [x] `get_diff()` のシグネチャを `Result<DiffResult, JjDescError>` に変更
- [x] 空 diff + マージコミットの場合は `Ok(DiffResult::EmptyMerge)` を返す
- [x] 通常の diff は `Ok(DiffResult::Content(diff))` を返す
- [x] 空 diff + 非マージの場合は `Err(JjDescError::EmptyDiff)` を返す

### 3. generate.rs の更新
- [x] `DiffResult` を import
- [x] `get_diff()` の結果を match で処理
- [x] `Content(diff)` の場合は LLM に渡す
- [x] `EmptyMerge` の場合は "Merge commit" を設定
- [x] 既存の `Err(EmptyDiff)` 分岐での冗長なマージ判定を削除

### 4. backfill.rs の更新
- [x] `DiffResult` を import
- [x] `get_diff()` の結果を match で処理
- [x] `EmptyMerge` の場合は "Merge branches" を設定
- [x] 既存の `is_merge` 判定ロジックを簡素化

### 5. テストの追加・更新
- [x] `DiffResult` enum のユニットテスト追加
- [x] 既存テストが通ることを確認
- [x] `cargo test` 全体の実行確認

### 6. CI 確認
- [x] `cargo clippy -- -D warnings` でエラーなし
- [x] `cargo fmt --check` でフォーマット確認

## 依存関係

なし（独立して実装可能）

## 検証方法

1. ユニットテストの追加・実行
2. 手動テスト: マージコミットに対して `jj-desc generate` を実行
3. dry-run モードで動作確認
