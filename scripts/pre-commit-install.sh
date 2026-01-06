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

echo "pre-commit hooks installed successfully!"
echo ""
echo "Hooks enabled:"
echo "  - pre-commit: fmt, clippy, trailing-whitespace, etc."
echo "  - pre-push: cargo test"
