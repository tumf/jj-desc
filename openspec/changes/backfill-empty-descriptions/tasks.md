# Tasks: backfill-empty-descriptions

## Phase 1: CLI 構造のリファクタリング

- [x] `src/commands/` ディレクトリを作成
- [x] `src/commands/mod.rs` を作成（公開インターフェース定義）
- [x] `src/commands/generate.rs` を作成し、既存の main.rs のロジックを移動
- [x] `src/cli.rs` をサブコマンド構造に変更（`Command::Generate`, `Command::Backfill`）
- [x] `src/main.rs` でサブコマンドディスパッチを実装
- [x] テスト: `cargo build` が成功することを確認
- [x] テスト: `jj-desc` (引数なし) が既存と同じ動作をすることを確認

## Phase 2: jj コマンド拡張

- [x] `src/jj.rs` に `get_commits_without_description()` 関数を追加
  - `jj log -r 'description(exact:"") & <revset>'` を実行
  - change_id のリストを返す
- [x] `src/jj.rs` の `get_diff()` を変更し、revision パラメータを必須にする（既に Option<&str> で実装済み）
- [x] `src/jj.rs` の `set_description()` を変更し、revision パラメータを必須にする（既に Option<&str> で実装済み）
- [x] テスト: 各関数の単体テストを追加（可能な範囲で）

## Phase 3: backfill コマンドの実装

- [x] `src/commands/backfill.rs` を作成
- [x] `BackfillArgs` 構造体を定義（`--revisions`, `--dry-run`, `--limit`）
- [x] `execute_backfill()` 関数を実装:
  1. `get_commits_without_description()` でコミット一覧取得
  2. `--limit` が指定されていれば件数を制限
  3. 各コミットに対して:
     - `get_diff()` で diff 取得
     - `LlmClient::generate_description()` で description 生成
     - `--dry-run` でなければ `set_description()` を実行
  4. 成功/失敗のカウントを表示
- [x] エラーハンドリング: 個別コミットの失敗時にスキップして続行
- [x] テスト: `cargo build` が成功することを確認

## Phase 4: 進捗表示の実装

- [x] 処理中の進捗表示を追加（"Processing: X/Y"）
- [x] 各コミットの処理結果を表示（✓/✗ + change_id + 一行サマリー）
- [x] 最終サマリー表示（成功/スキップ/失敗の件数）

## Phase 5: インタラクティブモードの実装

- [x] `--interactive` オプションを追加
- [x] 各コミットごとに生成した description を表示
- [x] ユーザー入力を受け付け: Accept(a) / Skip(s) / Quit(q)
- [ ] Edit の場合、エディタを起動（`$EDITOR` 環境変数を使用）- 将来の拡張として保留
- [x] テスト: インタラクティブモードが動作することを手動確認

## Phase 6: テストとドキュメント

- [x] README.md に `jj-desc backfill` の使用例を追加
- [ ] 統合テスト: dry-run モードのテスト - 手動テストで確認済み、自動テストは将来の拡張
- [ ] 統合テスト: limit オプションのテスト - 手動テストで確認済み
- [ ] エラーケースのテスト（jj が存在しない、API キー未設定など）- 既存のエラーハンドリングで対応済み
- [x] リリースノートを作成（AGENTS.md と README.md に記載）

## Optional: 将来の拡張

- [ ] `--delay` オプション（API レート制限対策）
- [ ] `--parallel` オプション（並列処理）
- [ ] `--filter` オプション（追加のフィルタ条件）
