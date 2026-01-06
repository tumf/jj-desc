# Change Proposal: Homebrew Distribution Support

## Overview

Enable users to install jj-desc via `brew install jj-desc` command.

## Background

Currently, jj-desc is only installable via `cargo install` or by downloading binaries from GitHub Releases. For macOS/Linux users, Homebrew is the most common package manager, and supporting Homebrew-based installation will significantly improve usability.

## Current State Analysis

### Existing Configuration

`dist-workspace.toml` already has Homebrew support configured via cargo-dist:

```toml
installers = ["shell", "powershell", "homebrew"]
tap = "tumf/homebrew-tap"
publish-jobs = ["homebrew"]
```

`.github/workflows/release.yml` includes a `publish-homebrew-formula` job that:
- Checks out the `tumf/homebrew-tap` repository
- Commits and pushes the Formula file

### Missing Configuration

1. **The `tumf/homebrew-tap` repository does not exist**
2. **`HOMEBREW_TAP_TOKEN` secret is not configured** (needs verification in repository settings)

## Proposed Changes

### 1. Create homebrew-tap Repository

Create a `tumf/homebrew-tap` repository on GitHub. This repository will function as a Homebrew "tap" and store Formula files.

### 2. Configure GitHub Secret

Set up `HOMEBREW_TAP_TOKEN`. This token requires write access to the `tumf/homebrew-tap` repository.

### 3. Update README

Add Homebrew installation instructions.

## Impact Scope

- New repository: `tumf/homebrew-tap`
- Configuration change: GitHub Secrets
- Documentation update: README.md

## Related Specifications

- New: `homebrew-distribution` specification
