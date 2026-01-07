# Design: refactor-config-resolution

## Design Decisions

### Configuration Value Precedence

Configuration values are resolved in the following priority order (higher takes precedence):

1. **CommandLine** — Explicitly specified via CLI arguments
2. **Environment** — Set via environment variables
3. **Default** — Provider-specific default values

### Tracking with ConfigSource

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSource {
    Default,
    Environment,
    CommandLine,
}
```

This enables the following decision-making in `with_provider()`:

```rust
pub fn with_provider(mut self, provider: Option<Provider>) -> Result<Self, JjDescError> {
    if let Some(p) = provider {
        self.provider = p;

        // Always retrieve the new provider's API key (error if missing)
        self.api_key = env::var(p.api_key_env_var())
            .map_err(|_| JjDescError::MissingApiKey)?;

        // Determine model based on source
        if self.model_source == ConfigSource::Default {
            // Only change to new provider's default if it was a default value
            self.model = p.default_model().to_string();
        }
        // Keep existing value for Environment/CommandLine (respect user intent)

        self.base_url = env::var(p.base_url_env_var())
            .unwrap_or_else(|_| p.default_base_url().to_string());
    }
    Ok(self)
}
```

### LLM Parameter Default Values

| Parameter | Default | Rationale |
|-----------|---------|-----------|
| `max_tokens` | `1024` | Sufficient for commit messages |
| `temperature` | `0.3` | Prioritize stable output |

### CLI Option Design

```rust
#[derive(Parser, Debug)]
pub struct GenerateArgs {
    // Existing options...

    /// Maximum tokens for LLM response
    #[arg(long, env = "LLM_MAX_TOKENS")]
    pub max_tokens: Option<u32>,

    /// Temperature for LLM response (0.0-2.0)
    #[arg(long, env = "LLM_TEMPERATURE")]
    pub temperature: Option<f32>,
}
```

### API Request Structure Changes

**OpenAI-compatible:**
```rust
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}
```

**Anthropic:**
```rust
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,  // Required by Anthropic
    system: String,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}
```

## Backward Compatibility

- Existing environment variables continue to work
- All new options are optional
- Default behavior remains equivalent to current implementation
