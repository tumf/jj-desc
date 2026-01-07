# Contributing to jj-desc

Thank you for your interest in contributing to jj-desc! This document provides guidelines and instructions for development.

## Development Setup

### Requirements

- Rust 1.85+ (Edition 2024)
- [jj](https://github.com/martinvonz/jj) installed and available in PATH
- API key for your chosen LLM provider (for testing)

### Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Set Up Pre-commit Hooks

```bash
# Install pre-commit (if not already installed)
pip install pre-commit
# or on macOS
brew install pre-commit

# Install hooks
./scripts/pre-commit-install.sh
```

The pre-commit hooks run the **same checks as CI**:
- **On every commit:**
  - `cargo fmt --check` - Validate code formatting (fails if not formatted)
  - `cargo clippy --all-features -- -D warnings` - Lint with zero warnings
- **On push:**
  - `cargo test --all-features` - Run all tests

To manually run all checks: `pre-commit run --all-files`

## Release Process

This project uses `cargo-release` and `cargo-dist` for releases:

```bash
# Install tools (one-time setup)
cargo install cargo-release git-cliff

# Create a release (example: minor version bump 0.2.0 → 0.3.0)
cargo release minor --execute

# Push tag to trigger GitHub release build
git push --follow-tags
```

See [AGENTS.md](AGENTS.md#release-process) for detailed release instructions.

## Project Structure

```
src/
├── main.rs         # Entry point and logging setup
├── cli.rs          # Command-line argument parsing
├── commands/       # Command implementation
│   └── mod.rs          # Unified command execution
├── config.rs       # Configuration management
├── diff_filter.rs  # Diff filtering and optimization
├── provider.rs     # Provider enumeration
├── error.rs        # Error type definitions
├── jj.rs           # jj command integration
├── llm/
│   ├── mod.rs          # LLM client trait and factory
│   ├── openai_compat.rs # OpenAI-compatible client
│   └── anthropic.rs     # Anthropic Messages API client
└── prompt.rs       # LLM prompt generation
```

## Technology Stack

- **Rust Edition 2024** (requires Rust 1.85+)
- **clap** - CLI argument parsing
- **tokio** - Async runtime
- **async-trait** - Trait support for async methods
- **reqwest** - HTTP client (rustls-tls, no OpenSSL dependency)
- **serde** - JSON serialization
- **thiserror** / **anyhow** - Error handling
- **tracing** - Structured logging

## Code Style

Please follow the coding conventions described in [AGENTS.md](AGENTS.md):

- **Import Order**: Standard library → External crates → Internal modules
- **Naming**: PascalCase for types, snake_case for functions, SCREAMING_SNAKE_CASE for constants
- **Error Handling**: Use `thiserror` for custom errors and `anyhow` for context
- **Documentation**: Module-level comments at file top, `///` doc comments for public APIs

## Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting: `cargo test && cargo clippy --all-features -- -D warnings`
5. Format code: `cargo fmt`
6. Submit a pull request

## License

By contributing to jj-desc, you agree that your contributions will be licensed under the MIT License.
