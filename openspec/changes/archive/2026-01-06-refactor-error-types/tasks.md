# Tasks: refactor-error-types

## Implementation Tasks

### 1. Add New Error Variants
- [x] Add `ApiStatus { status: u16, body: String }` to `src/error.rs`
- [x] Add `ApiResponseError(String)` for response parsing errors
- [x] Update existing tests

### 2. Fix openai_compat.rs
- [x] Use `JjDescError::ApiStatus` for HTTP errors
- [x] Use `ApiResponseError` for response parsing errors
- [x] Remove usage of `JjCommand` for API errors

### 3. Fix anthropic.rs
- [x] Use `JjDescError::ApiStatus` for HTTP errors
- [x] Use `ApiResponseError` for response parsing errors
- [x] Remove usage of `JjCommand` for API errors

### 4. Fix Provider Parsing in config.rs
- [x] Return `InvalidProvider` when `LLM_PROVIDER` exists but cannot be parsed
- [x] Fall back to `OpenRouter` only when environment variable is not set
- [x] Add test cases

### 5. Test Error Messages
- [x] Test `Display` implementation for new variants
- [x] Verify error messages are appropriate

### 6. CI Verification
- [x] No errors from `cargo clippy -- -D warnings`
- [x] Verify all `cargo test` passes

## Dependencies

None (can be implemented independently)

## Verification Method

1. Set invalid `LLM_PROVIDER` and verify appropriate error is shown
2. Test with invalid API key to verify API error messages
3. Add and run unit tests
