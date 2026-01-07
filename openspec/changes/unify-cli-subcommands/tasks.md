# タスク: unify-cli-subcommands

## 実装タスク

- [x] 1. CLI定義の変更 (`src/cli.rs`)
  - `Command` enum と `GenerateArgs`, `BackfillArgs` を削除
  - `Args` 構造体にすべてのオプションを直接定義
  - `-r/--revisions` オプション（デフォルト: `::@ & mutable()`）
  - `-n/--limit` オプション
  - `-i/--interactive` オプション
  - 既存の共通オプション（`--dry-run`, `--provider`, `--model` など）を維持

- [x] 2. コマンド実行ロジックの統合 (`src/commands/mod.rs`)
  - `execute_generate` と `execute_backfill` を単一の `execute` 関数に統合
  - サブコマンド分岐ロジックを削除

- [x] 3. main.rs の簡素化
  - `match args.command` 分岐を削除
  - 直接 `commands::execute(args)` を呼び出す

- [x] 4. テストの更新 (`src/cli.rs` のテスト)
  - サブコマンドのテストを削除
  - 新しいオプション構造に対応したテストを追加
  - `test_generate_subcommand` → 削除
  - `test_backfill_subcommand` → 削除
  - 新規: `test_revisions_option`, `test_limit_option` など

- [x] 5. 仕様の更新 (`openspec/specs/backfill-descriptions/spec.md`)
  - `jj-desc backfill` を `jj-desc` に変更
  - シナリオの更新

- [x] 6. ドキュメント更新
  - README.md のコマンド例を更新
  - `--help` 出力の確認

## 検証

- [x] `cargo test` が全て通過
- [x] `cargo clippy --all-features -- -D warnings` が警告なし
- [x] `jj-desc --help` が正しく表示される
- [x] `jj-desc -r @` が動作する
- [x] `jj-desc` (引数なし) がデフォルト動作する
