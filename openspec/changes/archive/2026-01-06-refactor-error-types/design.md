# Design: refactor-error-types

## Design Decisions

### Error Variant Classification

Reclassify the current `JjDescError` variants as follows:

| Category | Variant | Purpose |
|----------|---------|---------|
| Configuration | `MissingApiKey` | API key not set |
| Configuration | `InvalidProvider` | Invalid provider name |
| jj Errors | `JjCommand` | jj command execution failure |
| jj Errors | `EmptyDiff` | Empty diff (non-merge) |
| API Errors | `ApiError` | Network/connection errors |
| API Errors | `ApiStatus` | HTTP status errors (4xx, 5xx) |
| API Errors | `ApiResponseError` | Response parse/format errors |
| Other | `InvalidUtf8` | Character encoding errors |
| Other | `Io` | I/O errors |
| Other | `JsonError` | JSON parsing errors |

### Provider Parsing Strategy

```rust
// Current implementation (problematic)
let provider = env::var("LLM_PROVIDER")
    .ok()
    .and_then(|s| s.parse().ok())  // Silently ignores parse errors
    .unwrap_or(Provider::OpenRouter);

// After fix
let provider = match env::var("LLM_PROVIDER") {
    Ok(s) => s.parse()?,  // Propagate parse errors
    Err(env::VarError::NotPresent) => Provider::OpenRouter,  // Default only when unset
    Err(env::VarError::NotUnicode(_)) => {
        return Err(JjDescError::InvalidProvider("non-UTF8 value".into()))
    }
};
```

### API Error Discrimination

```rust
// Network errors (reqwest::Error)
// → ApiError (automatic conversion via #[from])

// HTTP status errors (4xx, 5xx)
if !response.status().is_success() {
    return Err(JjDescError::ApiStatus {
        status: response.status().as_u16(),
        body: response.text().await.unwrap_or_default(),
    });
}

// Response format errors (empty choices, etc.)
.ok_or_else(|| JjDescError::ApiResponseError(
    "No choices in API response".to_string()
))?
```

## Backward Compatibility

- No impact on CLI exit codes
- Only error messages are improved
- No breaking changes
