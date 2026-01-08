# Proposal: refactor-code-quality

## Why

jj-desc のコードベースに複数のコード品質上の問題が存在する。これらは現在の動作に影響しないが、保守性・可読性・一貫性の観点で改善が必要である。

### 発見された問題

#### 1. コードクローン（DRY 違反）

1. **test_config ヘルパーの重複**: 同一の `test_config()` 関数が3箇所で定義されている
   - `src/llm/mod.rs:31-40`
   - `src/llm/openai_compat.rs:116-125`
   - `src/llm/anthropic.rs:113-121`

2. **HTTP クライアント初期化の重複**: `reqwest::Client` ビルダーコードがほぼ同一
   - `src/llm/openai_compat.rs:44-55`
   - `src/llm/anthropic.rs:47-58`

3. **環境変数の保存・復元パターン**: config.rs テストで6回以上繰り返されるボイラープレート

#### 2. 一貫性の問題

1. **エラーメッセージの大文字・小文字**: `JjDescError` の variant で不一致
   - ほとんどが大文字開始: "API key environment variable..."
   - 1つだけ小文字開始: "jj command failed..."

2. **Option ハンドリングパターン**: `Config` ビルダーメソッドで異なるパターン使用
   - `with_model`: `if let Some(m) = model` を使用
   - `with_max_tokens`: `if max_tokens.is_some()` を使用（冗長に Some で再ラップ）

3. **マジックナンバー**: ハードコードされた値が複数ファイルに散在
   - `max_tokens: 1024`, `temperature: 0.3`
   - `timeout: 30 secs`, `connect_timeout: 5 secs`

## What Changes

### 1. 共通 HTTP クライアントビルダーの抽出

`llm/mod.rs` に共通関数を追加し、`openai_compat.rs` と `anthropic.rs` から呼び出す。

### 2. テストユーティリティモジュールの作成

`test_config` ヘルパーを共通化。将来的には `tests/` ディレクトリへの移動も検討。

### 3. エラーメッセージ一貫性の修正

すべてのエラーメッセージを大文字開始に統一。

### 4. Config ビルダーパターンの統一

`if let Some(x) = opt` パターンに統一し、冗長なラッピングを排除。

### 5. 名前付き定数の導入

`llm/mod.rs` または `config.rs` に定数を定義。

## Expected Benefits

- **保守性向上**: 変更が1箇所で済む
- **可読性向上**: 一貫したパターンでコードが理解しやすい
- **バグ防止**: マジックナンバーの集中管理で不整合を防止

## Impact Scope

- `src/llm/mod.rs`: 共通コード追加
- `src/llm/openai_compat.rs`: HTTP クライアント初期化の簡素化
- `src/llm/anthropic.rs`: HTTP クライアント初期化の簡素化
- `src/error.rs`: エラーメッセージ修正
- `src/config.rs`: ビルダーメソッド統一

## Priority

**Medium** — 機能に影響なし、保守性改善
