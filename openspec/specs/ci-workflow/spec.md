# ci-workflow Specification

## Purpose

This specification defines the Continuous Integration (CI) workflow for the `jj-desc` project using GitHub Actions. The CI system automatically executes unit tests, code quality checks, and format validation on pull requests and merges to the main branch.

## Requirements

### Requirement: Automated Unit Test Execution

The CI system SHALL automatically execute all unit tests whenever code changes are proposed or merged.

#### Scenario: Trigger test execution on pull request

**Given** a developer creates a pull request targeting the main branch  
**When** the pull request is opened or updated  
**Then** the CI workflow SHALL execute `cargo test --all-features`  
**And** test results SHALL be reported on the pull request

#### Scenario: Trigger test execution on push to main branch

**Given** a commit is merged to the main branch  
**When** a push event occurs  
**Then** the CI workflow SHALL execute all unit tests  
**And** results SHALL be visible in the repository's Actions tab

### Requirement: Multi-Version Rust Testing

The CI system SHALL execute tests on both the minimum supported Rust version (MSRV) and the latest stable version.

#### Scenario: MSRV compatibility check

**Given** the project specifies `rust-version = "1.85"` in Cargo.toml  
**When** the CI workflow is executed  
**Then** tests SHALL run on Rust 1.85  
**And** tests SHALL run on Rust stable  
**And** the workflow SHALL require passing on both versions to succeed

### Requirement: Code Quality Checks

The CI system SHALL enforce code quality through automated linting and format checking.

#### Scenario: Clippy lint check

**Given** the CI workflow is executed  
**When** the lint job runs  
**Then** `cargo clippy -- -D warnings` SHALL be executed  
**And** the job SHALL fail if clippy warnings are present

#### Scenario: Format check

**Given** the CI workflow is executed  
**When** the format job runs  
**Then** `cargo fmt --check` SHALL be executed  
**And** the job SHALL fail if formatting issues are present

### Requirement: Build Caching

The CI system SHALL cache build artifacts to reduce workflow execution time.

#### Scenario: Cached build

**Given** a previous CI execution has completed  
**When** a new CI execution starts with unchanged dependencies  
**Then** cached Cargo registry and build artifacts SHALL be restored  
**And** build time SHALL be significantly reduced compared to a cold build
