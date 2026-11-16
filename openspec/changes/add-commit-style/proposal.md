# Proposal: add-commit-style

## Why

jj-desc は現在 [Conventional Commits](https://www.conventionalcommits.org/) 形式のみをサポートしており、`SYSTEM_PROMPT` 定数とユーザープロンプトに形式がハードコードされている。

Linux、FreeBSD、Git、Go、nixpkgs など多くのプロジェクトは、型プレフィックスを使わずスコープ（サブシステム・モジュール）を先頭に置く [Scoped Commits](https://scopedcommits.com) 形式を採用している。これらのプロジェクトで jj-desc を使うと、生成された `feat:` / `fix:` 形式のメッセージを毎回手作業で書き直す必要がある。

## What Changes

### 1. コミットスタイル enum の導入

`src/prompt.rs` に `CommitStyle` enum（`Conventional` / `Scoped`）を追加する。

- `Conventional` をデフォルトとする（既存の動作を維持）
- `CommitStyle::system_prompt()` がスタイルごとのシステムプロンプトを返す
- `build_user_prompt(diff, style)` がスタイルを引数として受け取る
- `SYSTEM_PROMPT` 定数を削除する

### 2. CLI オプションと環境変数

- `--style <conventional|scoped>` オプションを追加
- 環境変数 `LLM_COMMIT_STYLE` でも指定可能（CLI が優先）
- 未指定時は `conventional`
- 不正な値はエラーとする

### 3. 設定と LLM クライアントへの伝搬

- `Config` に `style: CommitStyle` フィールドと `with_style()` ビルダーを追加
- `AnthropicClient` と `OpenAICompatClient` は `config.style` からプロンプトを生成する

### 4. ドキュメント

`README.md` と `README.ja.md` にオプション・環境変数を追記する。

## Expected Benefits

- Scoped Commits を採用するプロジェクトでもそのまま利用可能
- 将来的なスタイル追加が enum への variant 追加のみで済む
- デフォルト動作は変わらないため既存ユーザーへの影響なし

## Impact Scope

- `src/prompt.rs`: `CommitStyle` enum、スタイル別プロンプト
- `src/cli.rs`: `--style` オプション
- `src/config.rs`: `style` フィールド、`with_style()`
- `src/commands/mod.rs`: CLI 値を `Config` に適用
- `src/llm/anthropic.rs`, `src/llm/openai_compat.rs`: スタイル別プロンプトの使用
- `src/llm/mod.rs`: `test_config()` の更新
- `README.md`, `README.ja.md`: ドキュメント

## Breaking Changes

なし。`SYSTEM_PROMPT` は crate 内部の定数であり、公開 API ではない。

## Priority

**Medium** — 新機能、既存動作に影響なし
