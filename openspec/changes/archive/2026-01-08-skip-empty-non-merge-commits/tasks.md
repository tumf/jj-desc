# タスクリスト

## 実装タスク

1. [x] **`src/jj.rs` の変更**
   - `EMPTY_NON_MERGE_DESCRIPTION` 定数を削除
   - 関連するテスト (`test_empty_non_merge_description_constant`, `test_description_constants_are_different`) を削除
   - ドキュメントコメントの更新

2. [x] **`src/commands/mod.rs` の変更**
   - `DiffResult::EmptyNonMerge` のインポート削除
   - `EmptyNonMerge` ケースの処理を「スキップ」に変更
   - スキップ時のメッセージ表示（例: `○ Skipped (empty non-merge commit)`）

3. [x] **仕様の更新**
   - `openspec/specs/merge-detection/spec.md` を更新
   - REQ-MERGE-002, REQ-MERGE-003 のシナリオ修正

4. [x] **テスト実行**
   - `cargo test` で全テストがパスすることを確認
   - `cargo clippy` で警告がないことを確認

5. [x] **ドキュメント更新**
   - `README.md` の該当部分を確認・修正（必要に応じて）

## 検証

- [x] 空の非マージコミットがスキップされることを手動で確認
- [x] マージコミットの処理が従来通り動作することを確認
