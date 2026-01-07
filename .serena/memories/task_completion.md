# Task Completion Checklist for jj-desc

## Before Completing a Task

### 1. Format Code
```bash
cargo fmt
```

### 2. Run Linter
```bash
cargo clippy --all-features -- -D warnings
```

### 3. Run Tests
```bash
cargo test
```

### 4. Check Build
```bash
cargo build
```

## CI/CD Checks
GitHub Actions runs on push/PR to `main`:
- **test**: `cargo test` (Rust 1.85 + stable)
- **lint**: `cargo clippy -- -D warnings`
- **format**: `cargo fmt --check`

## Pre-commit Hooks
If installed (`./scripts/pre-commit-install.sh`):
- On commit: `cargo fmt --check` and `cargo clippy`
- On push: `cargo test`

## Quick Verification Command
```bash
cargo fmt && cargo clippy --all-features -- -D warnings && cargo test
```
