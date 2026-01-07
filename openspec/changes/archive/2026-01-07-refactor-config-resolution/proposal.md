# Proposal: refactor-config-resolution

## Summary

Refactor the `Config` resolution logic to clarify configuration value precedence and LLM request parameters.

## Why

The current implementation has the following issues:

1. **Implicit model replacement**: When the model matches the default value in `with_provider()`, it is automatically replaced with the new provider's default (`config.rs:76-83`)
   - Example problem: Even if a user intentionally specifies `gpt-4o`, it gets replaced if it matches another provider's default

2. **API key inconsistency**: When switching providers via CLI, if the new provider's API key is missing, the old provider's key remains (`config.rs:73-75`)

3. **Insufficient LLM parameters**: `max_tokens` and `temperature` cannot be configured, leading to unstable output depending on the provider

## What Changes

### 1. Configuration Source Tracking

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSource {
    Default,
    Environment,
    CommandLine,
}

pub struct Config {
    pub provider: Provider,
    pub api_key: String,
    pub model: String,
    pub model_source: ConfigSource,  // Added
    pub base_url: String,
    pub max_tokens: Option<u32>,     // Added
    pub temperature: Option<f32>,    // Added
}
```

### 2. Modify with_provider() Behavior

- Replace with the new provider's default model only when `model_source` is `Default`
- Make the new provider's API key mandatory; return an error if missing

### 3. Add LLM Request Parameters

- Add `--max-tokens` / `LLM_MAX_TOKENS` option
- Add `--temperature` / `LLM_TEMPERATURE` option
- Support both OpenAI-compatible and Anthropic providers

## Expected Benefits

- Configuration precedence becomes clear, reducing unexpected behavior
- Improved LLM output stability
- Source of configuration values becomes traceable during debugging

## Impact Scope

- `src/config.rs`: Extend `Config` struct, modify resolution logic
- `src/cli.rs`: Add new options
- `src/llm/openai_compat.rs`: Support `max_tokens`, `temperature`
- `src/llm/anthropic.rs`: Support `temperature`

## Priority

**Medium** — Improves user experience, but current implementation is functional
