# Development Guide

## Quick Start

### Install Pre-commit Hooks (Recommended)

To prevent CI failures, install pre-commit hooks that run the **same checks as CI**:

```bash
# Install pre-commit (if not already installed)
pip install pre-commit
# or on macOS
brew install pre-commit

# Install hooks for this project
./scripts/pre-commit-install.sh
```

### Pre-commit vs CI Checks

The pre-commit hooks are configured to run **exactly the same checks** as GitHub Actions CI:

| Check | Pre-commit Stage | CI Job | Command |
|-------|-----------------|--------|---------|
| Format validation | commit | `format` | `cargo fmt --check` |
| Lint (zero warnings) | commit | `lint` | `cargo clippy --all-features -- -D warnings` |
| All tests | push | `test` | `cargo test --all-features` |
| Conventional commits | commit-msg | N/A | Commit message validation |

### Manual Testing

```bash
# Run all pre-commit checks manually
pre-commit run --all-files

# Run individual checks (same as CI)
cargo fmt --check
cargo clippy --all-features -- -D warnings
cargo test --all-features

# Auto-fix formatting
cargo fmt
```

### Bypass Hooks (Not Recommended)

If you need to bypass hooks temporarily:

```bash
git commit --no-verify     # Skip pre-commit hooks
git push --no-verify       # Skip pre-push hooks
```

**Warning:** This may cause CI to fail. Only use when necessary.

## Commit Message Format

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <description>

[optional body]

[optional footer]
```

**Allowed types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

**Examples:**
```
feat: add backfill mode for generating descriptions
fix: handle empty diffs for merge commits
docs: update README with pre-commit instructions
ci: align pre-commit hooks with GitHub Actions
```

## CI Workflow

The GitHub Actions CI runs on every push to `main` and on pull requests:

1. **Test Job** (Matrix: Rust 1.85 + stable)
   - `cargo test --all-features`

2. **Lint Job**
   - `cargo clippy --all-features -- -D warnings`

3. **Format Job**
   - `cargo fmt --check`

All jobs must pass for CI to succeed.

## Troubleshooting

### Pre-commit hook fails but command passes locally

Make sure you're using the same flags:
```bash
# Wrong (missing --all-features)
cargo clippy -- -D warnings

# Correct (same as CI)
cargo clippy --all-features -- -D warnings
```

### Format check fails

Auto-fix with:
```bash
cargo fmt
```

### Clippy warnings

Fix warnings or add `#[allow(clippy::lint_name)]` with justification:
```rust
// Justification: xyz
#[allow(clippy::redundant_closure)]
fn example() { ... }
```
