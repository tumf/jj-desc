# タスク: add-commit-style

## 実装タスク

### Phase 1: プロンプト

#### 1.1 CommitStyle enum の追加
- [x] `src/prompt.rs` に `CommitStyle`（`Conventional` / `Scoped`）を定義
- [x] `clap::ValueEnum` と `Default`（`Conventional`）を derive
- [x] `system_prompt()` と `format_requirements()` を実装

#### 1.2 プロンプト生成の変更
- [x] `build_user_prompt(diff, style)` にスタイル引数を追加
- [x] 共通要件とスタイル固有要件を分離
- [x] `SYSTEM_PROMPT` 定数を削除

### Phase 2: CLI と設定（Phase 1 に依存）

#### 2.1 CLI オプション
- [x] `src/cli.rs` に `--style`（`env = "LLM_COMMIT_STYLE"`、デフォルト `conventional`）を追加

#### 2.2 Config
- [x] `Config` に `style` フィールドを追加
- [x] `with_style()` ビルダーメソッドを追加
- [x] `src/commands/mod.rs` で `args.style` を適用

### Phase 3: LLM クライアント（Phase 2 に依存）

- [x] `src/llm/anthropic.rs` で `config.style` からプロンプトを生成
- [x] `src/llm/openai_compat.rs` で `config.style` からプロンプトを生成
- [x] `src/llm/mod.rs` の `test_config()` を更新

### Phase 4: ドキュメント

- [x] `README.md` にオプションと環境変数を追記
- [x] `README.ja.md` にオプションと環境変数を追記

## 検証タスク

- [x] `prompt.rs`: Scoped プロンプトのテスト、デフォルトスタイルのテスト
- [x] `cli.rs`: `--style` のデフォルト値、`scoped` のパース、不正値のエラーのテスト
- [x] `cargo test` がパスすること
- [x] `cargo clippy --all-features -- -D warnings` がパスすること
- [x] `cargo fmt --check` がパスすること
- [ ] 手動確認: `jj-desc --dry-run --style scoped` で Scoped Commits 形式のメッセージが生成されること
- [ ] 手動確認: `LLM_COMMIT_STYLE=scoped jj-desc --dry-run` で同様の結果になること
- [ ] 手動確認: オプション未指定時に Conventional Commits 形式が維持されること
