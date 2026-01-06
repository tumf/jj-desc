# Proposal: refactor-error-types

## Why

The current error handling implementation has several issues that affect debugging and user experience:

1. **Error type confusion**: API HTTP status errors are incorrectly mapped to `JjDescError::JjCommand` (`openai_compat.rs:107`, `anthropic.rs:104`)
2. **Semantic ambiguity**: `JjCommand` is intended for `jj` command execution failures, but is also used for API errors, causing log confusion
3. **Silent configuration errors**: `Config::from_env()` silently ignores `LLM_PROVIDER` parsing errors and falls back to `OpenRouter`, preventing users from detecting typos

## What Changes

### 1. Add New Error Variants

```rust
#[derive(Error, Debug)]
pub enum JjDescError {
    // Existing
    #[error("API key environment variable is not set...")]
    MissingApiKey,

    #[error("Invalid provider: {0}...")]
    InvalidProvider(String),

    #[error("No changes found in diff")]
    EmptyDiff,

    #[error("jj command failed: {0}")]
    JjCommand(String),

    // New additions
    #[error("API request failed with status {status}: {body}")]
    ApiStatus { status: u16, body: String },

    #[error("API response error: {0}")]
    ApiResponseError(String),

    // Existing (unchanged)
    #[error("API request failed: {0}")]
    ApiError(#[from] reqwest::Error),
    // ...
}
```

### 2. Fix Provider Parsing Errors

When `LLM_PROVIDER` is set but cannot be parsed, return an `InvalidProvider` error instead of silently falling back.

## Expected Benefits

- Clearer error messages improve debugging efficiency
- Users can detect configuration mistakes early
- Improved log analysis and troubleshooting

## Impact

- `src/error.rs`: Add new error variants
- `src/llm/openai_compat.rs`: Use `ApiStatus` for HTTP errors
- `src/llm/anthropic.rs`: Use `ApiStatus` for HTTP errors
- `src/config.rs`: Return error on provider parsing failure

## Priority

**High** — Directly impacts user experience and debugging efficiency
