# homebrew-distribution Specification

## Purpose

Enable users to install jj-desc via Homebrew, the most common package manager for macOS and Linux users. This specification defines the requirements for automated Homebrew formula publishing and distribution through a dedicated tap repository.
## Requirements
### Requirement: REQ-BREW-001: Homebrew Tap Repository

A dedicated GitHub repository SHALL exist to host Homebrew formula files.

#### Scenario: Tap Repository Structure

**Given** the user wants to install jj-desc via Homebrew
**When** they add the tap `tumf/tap`
**Then** the tap is successfully added
**And** the repository `tumf/homebrew-tap` contains a `Formula/` directory

#### Scenario: Formula File Presence

**Given** a release has been published
**When** the release workflow completes successfully
**Then** the `tumf/homebrew-tap` repository contains `Formula/jj-desc.rb`
**And** the formula references the correct version and checksums

### Requirement: REQ-BREW-002: Automated Formula Publishing

The release workflow SHALL automatically publish updated Homebrew formulas on each release.

#### Scenario: Successful Formula Update on Release

**Given** the `HOMEBREW_TAP_TOKEN` secret is configured
**And** the `tumf/homebrew-tap` repository exists
**When** a new version tag (e.g., `v0.2.0`) is pushed
**Then** the `publish-homebrew-formula` job runs successfully
**And** the formula file is updated in `tumf/homebrew-tap`
**And** the formula contains the new version number
**And** the formula contains correct checksums for all platform binaries

#### Scenario: Prerelease Handling

**Given** a prerelease tag (e.g., `v0.2.0-rc.1`) is pushed
**When** the release workflow runs
**Then** the Homebrew formula is NOT published (unless `publish_prereleases` is true)

### Requirement: REQ-BREW-003: User Installation Experience

Users SHALL be able to install jj-desc using standard Homebrew commands.

#### Scenario: First-time Installation with Tap

**Given** the user has Homebrew installed
**And** the user has not added the tumf tap
**When** the user runs `brew tap tumf/tap`
**And** the user runs `brew install jj-desc`
**Then** jj-desc is installed successfully
**And** `jj-desc --version` outputs the installed version

#### Scenario: Direct Installation without Explicit Tap

**Given** the user has Homebrew installed
**When** the user runs `brew install tumf/tap/jj-desc`
**Then** the tap is automatically added
**And** jj-desc is installed successfully

#### Scenario: Upgrade to New Version

**Given** jj-desc is installed via Homebrew
**And** a new version has been released
**When** the user runs `brew upgrade jj-desc`
**Then** jj-desc is upgraded to the latest version

### Requirement: REQ-BREW-004: Multi-Platform Support

The Homebrew formula SHALL support both macOS and Linux on common architectures.

#### Scenario: macOS ARM64 Installation

**Given** the user is on macOS with Apple Silicon (arm64)
**When** the user installs jj-desc via Homebrew
**Then** the `aarch64-apple-darwin` binary is downloaded and installed

#### Scenario: macOS x86_64 Installation

**Given** the user is on macOS with Intel processor (x86_64)
**When** the user installs jj-desc via Homebrew
**Then** the `x86_64-apple-darwin` binary is downloaded and installed

#### Scenario: Linux ARM64 Installation

**Given** the user is on Linux with ARM64 architecture
**When** the user installs jj-desc via Homebrew
**Then** the `aarch64-unknown-linux-gnu` binary is downloaded and installed

#### Scenario: Linux x86_64 Installation

**Given** the user is on Linux with x86_64 architecture
**When** the user installs jj-desc via Homebrew
**Then** the `x86_64-unknown-linux-gnu` binary is downloaded and installed
