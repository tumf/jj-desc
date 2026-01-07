#!/usr/bin/env bash
# Release script for jj-desc
# Usage: ./scripts/release.sh [patch|minor|major]

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if release type is provided
if [ $# -eq 0 ]; then
	echo -e "${RED}Error: Release type not specified${NC}"
	echo "Usage: $0 [patch|minor|major]"
	echo ""
	echo "Examples:"
	echo "  $0 patch  # 0.3.0 → 0.3.1"
	echo "  $0 minor  # 0.3.0 → 0.4.0"
	echo "  $0 major  # 0.3.0 → 1.0.0"
	exit 1
fi

RELEASE_TYPE=$1

# Validate release type
if [[ ! "$RELEASE_TYPE" =~ ^(patch|minor|major)$ ]]; then
	echo -e "${RED}Error: Invalid release type '$RELEASE_TYPE'${NC}"
	echo "Must be one of: patch, minor, major"
	exit 1
fi

# Check if we're on main branch
CURRENT_BRANCH=$(git symbolic-ref --short HEAD 2>/dev/null || echo "")
if [ -z "$CURRENT_BRANCH" ]; then
	echo -e "${RED}Error: Not on a branch (detached HEAD state)${NC}"
	echo "Please checkout main branch first:"
	echo "  git checkout main"
	exit 1
fi

if [ "$CURRENT_BRANCH" != "main" ]; then
	echo -e "${RED}Error: Not on main branch (currently on '$CURRENT_BRANCH')${NC}"
	echo "Please checkout main branch first:"
	echo "  git checkout main"
	exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
	echo -e "${RED}Error: You have uncommitted changes${NC}"
	echo "Please commit or stash your changes first"
	exit 1
fi

# Check for required tools
command -v git-cliff >/dev/null 2>&1 || {
	echo -e "${RED}Error: git-cliff is not installed${NC}"
	echo "Install with: brew install git-cliff"
	exit 1
}

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "${GREEN}Current version: ${CURRENT_VERSION}${NC}"

# Calculate new version
IFS='.' read -r -a version_parts <<<"$CURRENT_VERSION"
MAJOR="${version_parts[0]}"
MINOR="${version_parts[1]}"
PATCH="${version_parts[2]}"

case $RELEASE_TYPE in
patch)
	PATCH=$((PATCH + 1))
	;;
minor)
	MINOR=$((MINOR + 1))
	PATCH=0
	;;
major)
	MAJOR=$((MAJOR + 1))
	MINOR=0
	PATCH=0
	;;
esac

NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}"
echo -e "${GREEN}New version: ${NEW_VERSION}${NC}"

# Confirm with user
echo ""
read -p "Create release v${NEW_VERSION}? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
	echo -e "${YELLOW}Release cancelled${NC}"
	exit 0
fi

# Run pre-release checks
echo ""
echo -e "${YELLOW}Running pre-release checks...${NC}"

echo "  → cargo fmt --check"
cargo fmt --check || {
	echo -e "${RED}Error: Code is not formatted. Run 'cargo fmt'${NC}"
	exit 1
}

echo "  → cargo clippy"
cargo clippy --all-features -- -D warnings || {
	echo -e "${RED}Error: Clippy warnings found${NC}"
	exit 1
}

echo "  → cargo test"
cargo test --all-features || {
	echo -e "${RED}Error: Tests failed${NC}"
	exit 1
}

echo -e "${GREEN}All checks passed!${NC}"
echo ""

# Update version in Cargo.toml
echo -e "${YELLOW}Updating Cargo.toml...${NC}"
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
cargo check --quiet

# Generate CHANGELOG
echo -e "${YELLOW}Generating CHANGELOG...${NC}"
git cliff -o CHANGELOG.md --tag "v${NEW_VERSION}"

# Ensure CHANGELOG.md ends with exactly one newline (for pre-commit hooks)
# git cliff outputs multiple trailing newlines, remove extras
sed -i.bak -e :a -e '/^\n*$/{$d;N;ba' -e '}' CHANGELOG.md && rm -f CHANGELOG.md.bak

# Show changes
echo ""
echo -e "${YELLOW}Changes to be committed:${NC}"
git diff Cargo.toml CHANGELOG.md Cargo.lock

# Commit changes
echo ""
read -p "Commit these changes? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
	echo -e "${YELLOW}Restoring files...${NC}"
	git checkout Cargo.toml CHANGELOG.md Cargo.lock
	exit 0
fi

git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore: release v${NEW_VERSION}"

# Create tag
echo -e "${YELLOW}Creating tag v${NEW_VERSION}...${NC}"
git tag "v${NEW_VERSION}"

# Push
echo ""
echo -e "${GREEN}Ready to push!${NC}"
echo ""
echo "The following commands will be executed:"
echo "  git push"
echo "  git push origin v${NEW_VERSION}"
echo ""
read -p "Push to remote? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
	git push
	git push origin "v${NEW_VERSION}"
	echo ""
	echo -e "${GREEN}✓ Release v${NEW_VERSION} has been pushed!${NC}"
	echo ""
	echo "GitHub Actions will now:"
	echo "  1. Build binaries for all platforms"
	echo "  2. Create a GitHub Release"
	echo "  3. Upload binaries and install scripts"
	echo "  4. Update Homebrew formula"
	echo ""
	echo "Monitor progress at:"
	echo "  https://github.com/tumf/jj-desc/actions"
	echo ""
	echo "Release will be available at:"
	echo "  https://github.com/tumf/jj-desc/releases/tag/v${NEW_VERSION}"
else
	echo -e "${YELLOW}Push cancelled. You can push manually later:${NC}"
	echo "  git push --follow-tags"
fi
