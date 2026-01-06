# 提案: refactor-error-types

## 概要

`JjDescError` のバリアントを整理し、エラーの意味を明確化する。

## 背景・動機

現在の実装には以下の問題がある：

1. **エラー型の混線**: API の HTTP ステータスエラーを `JjDescError::JjCommand` に詰めている（`openai_compat.rs:107`, `anthropic.rs:104`）
2. **意味の曖昧さ**: `JjCommand` は本来 `jj` コマンド実行失敗用だが、API エラーにも使われておりログが混乱する
3. **設定エラーの握りつぶし**: `Config::from_env()` で `LLM_PROVIDER` のパースエラーを無視して `OpenRouter` にフォールバックしている（typo に気づけない）

## 提案内容

### 1. 新しいエラーバリアントの追加

```rust
#[derive(Error, Debug)]
pub enum JjDescError {
    // 既存
    #[error("API key environment variable is not set...")]
    MissingApiKey,

    #[error("Invalid provider: {0}...")]
    InvalidProvider(String),

    #[error("No changes found in diff")]
    EmptyDiff,

    #[error("jj command failed: {0}")]
    JjCommand(String),

    // 新規追加
    #[error("API request failed with status {status}: {body}")]
    ApiStatus { status: u16, body: String },

    #[error("API response error: {0}")]
    ApiResponseError(String),

    // 既存（変更なし）
    #[error("API request failed: {0}")]
    ApiError(#[from] reqwest::Error),
    // ...
}
```

### 2. Provider パースエラーの修正

`LLM_PROVIDER` が設定されているがパース不能な場合、`InvalidProvider` エラーを返す。

## 期待される効果

- エラーメッセージが明確になり、デバッグが容易に
- ユーザーが設定ミスに早期に気づける
- ログ分析が改善される

## 影響範囲

- `src/error.rs`: 新バリアント追加
- `src/llm/openai_compat.rs`: `ApiStatus` を使用
- `src/llm/anthropic.rs`: `ApiStatus` を使用
- `src/config.rs`: Provider パース失敗時にエラー返却

## 優先度

**高** — ユーザー体験とデバッグ効率に直接影響
