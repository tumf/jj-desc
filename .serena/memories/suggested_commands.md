# Suggested Commands for jj-desc

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
```

## Pre-commit Hooks
```bash
./scripts/pre-commit-install.sh         # Install pre-commit hooks
pre-commit run --all-files              # Run all checks manually
```

## Release
```bash
./scripts/release.sh minor    # Minor release (0.3.0 → 0.4.0)
./scripts/release.sh patch    # Patch release (0.3.0 → 0.3.1)
```

## Running the CLI
```bash
jj-desc                       # Generate descriptions for mutable commits
jj-desc -r @                  # Current working copy only
jj-desc --dry-run             # Preview mode
jj-desc -i                    # Interactive mode
jj-desc --provider openai     # Use specific provider
jj-desc --model gpt-4o        # Use specific model
jj-desc -v                    # Verbose logging
```
