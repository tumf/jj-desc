# Code Style and Conventions for jj-desc

## Import Order
```rust
// 1. Standard library
use std::env;

// 2. External crates (alphabetically)
use anyhow::{Context, Result};
use clap::Parser;

// 3. Internal modules
use crate::config::Config;
```

## Naming Conventions
| Element | Convention | Example |
|---------|------------|---------|
| Structs/Enums | PascalCase | `JjDescError`, `Config` |
| Functions | snake_case | `get_diff`, `from_env` |
| Constants | SCREAMING_SNAKE_CASE | `SYSTEM_PROMPT` |
| Modules | snake_case | `openai_compat` |

## Error Handling
Use `thiserror` for custom errors and `anyhow` for context:
```rust
#[derive(Error, Debug)]
pub enum JjDescError {
    #[error("API key not set")]
    MissingApiKey,
    #[error("API request failed: {0}")]
    ApiError(#[from] reqwest::Error),
}

// Add context with anyhow
Config::from_env().context("Failed to load config")?;
```

## Async Patterns
```rust
#[tokio::main]
async fn main() -> Result<()> { ... }

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_description(&self, diff: &str) -> Result<String, JjDescError>;
}

#[instrument(skip(self, diff))]  // tracing span
async fn generate_description(&self, diff: &str) -> Result<String, JjDescError> { ... }
```

## Struct Design
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config { ... }

// Builder pattern
impl Config {
    pub fn with_model(mut self, model: Option<String>) -> Self {
        if let Some(m) = model { self.model = m; }
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiRequest { ... }
```

## Test Patterns
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(expected, actual);
    }

    // Env var tests: use unsafe block with save/restore
    #[test]
    fn test_with_env_vars() {
        unsafe {
            let original = env::var("VAR").ok();
            env::set_var("VAR", "value");
            // ... test ...
            env::remove_var("VAR");
            if let Some(v) = original { env::set_var("VAR", v); }
        }
    }
}
```

## Documentation
- Module-level: `// Brief description` at file top
- Public APIs: `/// Description` doc comments
