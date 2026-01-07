# Release Guide

This document describes how to create a new release of jj-desc.

## Prerequisites

Install required tools (one-time setup):

```bash
brew install git-cliff
cargo install cargo-release
```

## Release Process

### Quick Method (Recommended)

Use the provided release script:

```bash
# Patch release (0.3.0 → 0.3.1)
./scripts/release.sh patch

# Minor release (0.3.0 → 0.4.0)
./scripts/release.sh minor

# Major release (0.3.0 → 1.0.0)
./scripts/release.sh major
```

### Manual Method

If you prefer to run commands manually:

```bash
# 1. Determine the new version
# - Patch: bug fixes only (0.3.0 → 0.3.1)
# - Minor: new features (0.3.0 → 0.4.0)
# - Major: breaking changes (0.3.0 → 1.0.0)

# 2. Update version in Cargo.toml
# Edit the version field manually or use:
cargo release minor --no-publish --no-tag --no-push --execute

# 3. Generate CHANGELOG
git cliff -o CHANGELOG.md --tag v0.X.Y

# 4. Commit changes
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore: release vX.Y.Z"

# 5. Create and push tag
git tag vX.Y.Z
git push --follow-tags
```

## What Happens After Push

1. **GitHub Actions triggers** (`release.yml`)
2. **cargo-dist builds** binaries for:
   - macOS (Intel & Apple Silicon)
   - Linux (x86_64 & ARM64)
   - Windows (x64)
3. **GitHub Release created** with:
   - Release notes from CHANGELOG
   - Pre-built binaries
   - Install scripts
   - Homebrew formula
4. **Homebrew tap updated** automatically

## Workflow Details

### Automated Steps

- ✅ Binary compilation for all platforms
- ✅ Checksum generation
- ✅ GitHub Release creation
- ✅ Homebrew formula update
- ✅ Install script generation

### Manual Steps Required

- 🔧 Version bump in Cargo.toml
- 🔧 CHANGELOG generation
- 🔧 Git commit and tag
- 🔧 Push to GitHub

## Troubleshooting

### Problem: cargo-release fails

**Solution:** Use the manual method instead.

### Problem: git-cliff generates wrong version

**Solution:** Always specify the exact tag:
```bash
git cliff -o CHANGELOG.md --tag v0.3.0
```

### Problem: GitHub Actions release fails

**Check:**
1. Tag format is correct (`vX.Y.Z`)
2. CHANGELOG.md is properly formatted
3. Cargo.toml version matches the tag
4. Check workflow logs: https://github.com/tumf/jj-desc/actions

## Version Numbering Guide

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
  - Remove/rename CLI flags
  - Change default behavior
  - Remove supported providers

- **MINOR** (0.X.0): New features (backward compatible)
  - Add new providers
  - Add new CLI options
  - Add new configuration options

- **PATCH** (0.0.X): Bug fixes only
  - Fix crashes
  - Fix incorrect behavior
  - Update dependencies

## jj-Specific Notes

This project uses jj (Jujutsu) for version control, which has some differences from git:

### Working with jj

```bash
# Instead of git add + git commit
jj commit -m "chore: release v0.3.0"

# Create a tag
jj git export  # Export to git first
git tag v0.3.0
git push --follow-tags
```

### Commit Messages

Unlike git projects, we **do NOT require Conventional Commits** format. Natural language commit messages are fine:

- ✅ "Add support for new LLM provider"
- ✅ "Fix crash when API key is missing"
- ✅ "Update installation instructions"

The CHANGELOG generator (git-cliff) is configured to parse both conventional and natural language commits.

## Release Checklist

Before creating a release, verify:

- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] No clippy warnings: `cargo clippy --all-features -- -D warnings`
- [ ] Version number is correct in Cargo.toml
- [ ] CHANGELOG.md is updated with all notable changes
- [ ] README.md reflects current features (if needed)

## Example Release Session

```bash
# 1. Run tests
cargo test --all-features
cargo clippy --all-features -- -D warnings
cargo fmt --check

# 2. Bump version to 0.4.0
vim Cargo.toml  # Change version = "0.3.0" to "0.4.0"

# 3. Generate CHANGELOG
git cliff -o CHANGELOG.md --tag v0.4.0

# 4. Review changes
git diff

# 5. Commit
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore: release v0.4.0"

# 6. Tag and push
git tag v0.4.0
git push --follow-tags

# 7. Wait for GitHub Actions
# Check: https://github.com/tumf/jj-desc/actions
# Release will appear at: https://github.com/tumf/jj-desc/releases
```

## Post-Release

After the release is published:

1. Verify binaries are available for all platforms
2. Test installation scripts work:
   ```bash
   curl --proto '=https' --tlsv1.2 -LsSf \
     https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.sh | sh
   ```
3. Check Homebrew formula was updated:
   ```bash
   brew update
   brew info tumf/tap/jj-desc
   ```
4. Announce the release (if applicable)

## Emergency Rollback

If a release has critical issues:

```bash
# 1. Delete the GitHub Release (via web UI)
# 2. Delete the tag locally and remotely
git tag -d v0.X.Y
git push origin :refs/tags/v0.X.Y

# 3. Fix the issue and create a new patch release
```

## References

- [cargo-dist documentation](https://opensource.axo.dev/cargo-dist/)
- [git-cliff documentation](https://git-cliff.org/)
- [Semantic Versioning](https://semver.org/)
