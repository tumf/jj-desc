<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# jj-desc Development Guide

A CLI tool that generates jj commit descriptions using LLM providers.

**Language:** Rust 2024 Edition | **MSRV:** 1.85+

## Build Commands

```bash
cargo build              # Development build
cargo build --release    # Release build (LTO, stripped)
cargo install --path .   # Install from source
cargo check              # Check without building
```

## Test Commands

```bash
cargo test                              # Run all tests
cargo test test_error_display           # Single test (partial match)
cargo test --exact test_provider_from_str  # Exact match
cargo test config::                     # Tests in module
cargo test -- --nocapture               # Show output
cargo test -- --test-threads=1          # Single-threaded (env var tests)
```

## Lint & Format

```bash
cargo fmt                                      # Format code
cargo fmt --check                              # Check formatting (CI)
cargo clippy --all-features -- -D warnings     # Lint (warnings as errors, CI)
./scripts/pre-commit-install.sh                # Install pre-commit hooks (recommended)
pre-commit run --all-files                     # Run all pre-commit checks manually
```

**Pre-commit hooks:** Install hooks to automatically run CI checks before commit/push:
- Prevents CI failures by catching issues locally
- Same checks as GitHub Actions CI
- Use `git commit --no-verify` to skip (not recommended)

## Code Style

### Import Order

```rust
// 1. Standard library
use std::env;

// 2. External crates (alphabetically)
use anyhow::{Context, Result};
use clap::Parser;

// 3. Internal modules
use crate::config::Config;
```

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Structs/Enums | PascalCase | `JjDescError`, `Config` |
| Functions | snake_case | `get_diff`, `from_env` |
| Constants | SCREAMING_SNAKE_CASE | `SYSTEM_PROMPT` |
| Modules | snake_case | `openai_compat` |

### Error Handling

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

### Async Patterns

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

### Struct Design

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

### Test Patterns

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

### Documentation

- Module-level: `// Brief description` at file top
- Public APIs: `/// Description` doc comments

## Project Structure

```
src/
├── main.rs       # Entry point, CLI orchestration
├── cli.rs        # Command-line args (clap)
├── config.rs     # Configuration, env vars
├── error.rs      # Custom errors (thiserror)
├── jj.rs         # jj VCS interaction
├── llm/          # LLM client implementations
│   ├── mod.rs
│   ├── anthropic.rs
│   └── openai_compat.rs
├── prompt.rs     # Prompt templates
└── provider.rs   # Provider enum/traits
```

## CI/CD

GitHub Actions on push/PR to `main`:
- **test**: `cargo test` (Rust 1.85 + stable)
- **lint**: `cargo clippy -- -D warnings`
- **format**: `cargo fmt --check`

## Release Process

This project uses **cargo-release** for version management and **cargo-dist** for binary distribution.

### Prerequisites

```bash
cargo install cargo-release
cargo install git-cliff  # For CHANGELOG generation
```

### Creating a Release

**Patch release (0.2.0 → 0.2.1):**
```bash
cargo release patch --execute
```

**Minor release (0.2.0 → 0.3.0):**
```bash
cargo release minor --execute
```

**Major release (0.2.0 → 1.0.0):**
```bash
cargo release major --execute
```

### What Happens

1. `cargo-release` bumps version in `Cargo.toml`
2. `git-cliff` generates/updates `CHANGELOG.md`
3. Creates a git commit: `chore: release v0.x.x`
4. Creates a git tag: `v0.x.x`
5. You manually push: `git push --follow-tags`
6. GitHub Actions (`release.yml`) builds binaries for all platforms
7. GitHub Release is created with:
   - Release notes from CHANGELOG
   - Pre-built binaries (Linux, macOS, Windows)
   - Install scripts (shell, PowerShell)
   - Homebrew formula

### Dry Run (Recommended First)

```bash
# Preview changes without executing
cargo release minor
```

### Manual Alternative

If `cargo-release` doesn't work as expected:

```bash
# 1. Update version manually
vim Cargo.toml  # Change version field

# 2. Generate CHANGELOG
git cliff -o CHANGELOG.md --tag v0.x.x

# 3. Commit and tag
git add Cargo.toml CHANGELOG.md
git commit -m "chore: release v0.x.x"
git tag v0.x.x

# 4. Push
git push --follow-tags
```

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| clap | CLI parsing (derive macros) |
| tokio | Async runtime |
| reqwest | HTTP client (rustls-tls) |
| serde | JSON serialization |
| thiserror/anyhow | Error handling |
| tracing | Structured logging |
| rstest | Test framework (dev) |
