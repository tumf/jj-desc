# Tasks: refactor-config-resolution

## Implementation Tasks

### 1. Add ConfigSource enum
- [x] Add `ConfigSource` enum to `src/config.rs`
- [x] Define `Default`, `Environment`, `CommandLine` variants

### 2. Extend Config struct
- [x] Add `model_source: ConfigSource` field
- [x] Add `max_tokens: Option<u32>` field
- [x] Add `temperature: Option<f32>` field
- [x] Set `model_source` appropriately in `from_env()`

### 3. Modify with_provider()
- [x] Replace model only when `model_source == Default`
- [x] Return error if new provider's API key is missing
- [x] Update existing tests

### 4. Modify with_model()
- [x] Set `model_source = CommandLine` when called

### 5. Add CLI options
- [x] Add `--max-tokens` to `GenerateArgs`
- [x] Add `--temperature` to `GenerateArgs`
- [x] Add same options to `BackfillArgs`
- [x] Support environment variables `LLM_MAX_TOKENS`, `LLM_TEMPERATURE`

### 6. Update openai_compat.rs
- [x] Add `max_tokens`, `temperature` to `ChatCompletionRequest`
- [x] Retrieve values from Config and include in request

### 7. Update anthropic.rs
- [x] Add `temperature` to `AnthropicRequest` (max_tokens already exists)
- [x] Retrieve values from Config

### 8. Add tests
- [x] Test that `ConfigSource` tracking works correctly
- [x] Test CLI options
- [x] Test environment variables

### 9. CI verification
- [x] No errors from `cargo clippy -- -D warnings`
- [x] Verify full `cargo test` execution

## Dependencies

- Implementing `refactor-error-types` first would simplify handling of missing API key errors

## Verification Methods

1. Verify provider switching with `--provider openai --model gpt-4o`
2. Verify parameters are reflected with `--max-tokens 500 --temperature 0.7`
3. Verify appropriate errors are shown for invalid configurations
