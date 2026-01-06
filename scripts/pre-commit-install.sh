#!/usr/bin/env bash
# Install pre-commit hooks for the project

set -euo pipefail

# Check if pre-commit is installed
if ! command -v pre-commit &>/dev/null; then
	echo "pre-commit is not installed."
	echo "Install it with: pip install pre-commit"
	echo "Or on macOS: brew install pre-commit"
	exit 1
fi

# Install the pre-commit hooks
pre-commit install
pre-commit install --hook-type pre-push
pre-commit install --hook-type commit-msg

echo "✅ pre-commit hooks installed successfully!"
echo ""
echo "Hooks enabled (same as CI):"
echo "  - pre-commit:"
echo "    • cargo fmt --check (format validation)"
echo "    • cargo clippy --all-features -- -D warnings (lint)"
echo "    • trailing-whitespace, end-of-file-fixer, etc."
echo "  - pre-push:"
echo "    • cargo test --all-features (all tests)"
echo "  - commit-msg:"
echo "    • conventional commits validation"
echo ""
echo "Run 'pre-commit run --all-files' to check all files manually."
