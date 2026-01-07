# Changelog

All notable changes to this project will be documented in this file.

## [0.4.1] - 2026-01-07

### Bug Fixes

- Fix: apply pre-commit formatting fixes (trailing whitespace, EOF)

### Documentation

- Docs: add proposal to skip empty non-merge commits
- Docs: add CONTRIBUTING.md and refactor README development section
- Docs(specs): update purpose sections and refine requirements

### Features

- Feat: skip empty non-merge commits instead of setting placeholder
- Feat(cli): add force regeneration mode for explicit revset usage
- Feat(release): add branch and uncommitted changes validation

### Refactoring

- Refactor(cli): simplify CLI structure by removing subcommands

## [0.4.0] - 2026-01-07

### Documentation

- Docs: update to emphasize Conventional Commits format
- Docs: archive completed refactor-config-resolution change

### Features

- Feat(diff): add diff filtering with exclusion patterns and size tracking
- Feat: add diff filtering to optimize large diff handling

Implement automatic exclusion of lock files, binary file simplification,
and customizable --exclude option to reduce token usage and improve
LLM processing efficiency. Add size warnings for diffs exceeding 50KB.
- Add design documentation for large diff optimization

Introduce comprehensive planning docs for implementing diff filtering
and size optimization features inspired by aicommit2, including lock
file exclusion, binary file summarization, and user-configurable
exclude patterns to reduce LLM token consumption and API errors.
- Add design documentation for CLI subcommand unification

Document the proposal to merge generate and backfill subcommands into a
unified interface without subcommands, including design decisions,
implementation tasks, and updated specifications.

### Miscellaneous Tasks

- Chore: release v0.4.0

### Refactoring

- Refactor release documentation and add automated release script

## [0.3.0] - 2026-01-07

### Bug Fixes

- Fix: format code to comply with rustfmt rules
- Resolve merge conflict in config tests and add provider validation

### Documentation

- Update README to clarify backfill as default mode

Reorganize usage sections to emphasize that backfill mode is the default behavior, improving clarity for new users about the tool's primary operation.
- Docs: add comprehensive installation guide for all platforms

- Update README.md with platform-specific installation instructions
- Add detailed INSTALL.md covering:
  - Homebrew installation (macOS/Linux)
  - Shell/PowerShell installers
  - Manual binary downloads for all platforms
  - Building from source
  - Troubleshooting common issues
  - Architecture-specific guidance (Apple Silicon, Intel, ARM64, x86_64)

### Features

- Add model_source, max_tokens, and temperature fields to LLM configs
- Add unit tests for low-coverage modules to improve test coverage

Implement comprehensive unit tests for CLI parsing, LLM client factory,
and provider-specific client initialization. Increases overall test
coverage from 53.66% to 67.75% by adding 27 new test cases across
cli.rs, llm/mod.rs, llm/anthropic.rs, and llm/openai_compat.rs.
- Add design documents for three refactoring proposals

Introduce comprehensive design documentation for refactoring config
resolution, diff result handling, and error types to improve code
maintainability and user experience.

### Miscellaneous Tasks

- Chore: release v0.3.0
- Chore: migrate from release-plz to cargo-release for better jj compatibility

### Refactoring

- Refactor-error-types
- Refactor-diff-result
- Refactor-config-resolution

## [0.2.0] - 2026-01-06

### Documentation

- Update README examples to show default generate command

### Features

- Feat: support description generation for empty merge commits

- Include merge commits in description generation even if empty
- Exclude non-merge empty commits (existing behavior)
- Use default 'Merge branches' description for empty merges
- Update get_diff to allow empty diff for merge commits

### Miscellaneous Tasks

- Chore: configure cargo-dist for Homebrew distribution

- Add Homebrew tap: tumf/homebrew-tap
- Configure multi-platform builds (macOS, Linux, Windows)
- Add shell/powershell/homebrew installers
- Auto-generated GitHub Actions release workflow
- Chore: release v0.2.0
- Chore: bump version to 0.2.0 for feat release
- Chore: release v0.1.0

## [0.1.0] - 2026-01-06

### Bug Fixes

- Fix: simplify release-plz config to use only valid fields
- Fix: correct release-plz workspace configuration structure
- Resolve merge conflict in CI workflow specification

Add automated testing, multi-version support, and quality checks

### Documentation

- Docs: add comprehensive development guide and align pre-commit with CI
- Update documentation to reflect multi-provider LLM support

Add comprehensive development guide with Rust patterns, project structure,
and CI/CD information. Update project overview to show support for multiple
LLM providers (OpenRouter, OpenAI, Anthropic, Gemini) instead of just
OpenRouter.

### Features

- Add merge commit detection for empty diff handling

Implement special handling for merge commits that have empty diffs by detecting when a commit has multiple parents and providing a default "Merge commit" description instead of failing with an empty diff error.
- Add backfill mode for generating descriptions for multiple commits

Add `jj-desc backfill` subcommand to process multiple commits with empty
descriptions at once. Features include revset targeting, interactive mode
for reviewing each description, dry-run preview, and progress tracking.
Maintains backward compatibility with existing `jj-desc` usage.
- Add design documentation for merge commit detection feature
- Add GitHub Actions CI workflow for automated testing

Implement comprehensive CI pipeline with unit tests, code quality checks,
and format validation on pull requests and main branch pushes. Includes
multi-version Rust testing (MSRV 1.85 and stable) with build caching
to ensure code quality and prevent regressions.
- Add release automation and CI/CD infrastructure

Set up automated release workflows with release-plz for changelog generation, GitHub Actions for multi-platform builds and releases, and conventional commit validation via pre-commit hooks.
- Add pre-commit hooks configuration for code quality

Set up pre-commit hooks with cargo fmt, clippy, and tests to enforce
consistent code formatting, linting, and ensure tests pass before
commits and pushes.
- Add GitHub Actions CI workflow with testing, linting, and formatting
- Add GitHub Actions CI workflow and organize project changes

- Set up automated testing, linting, and formatting checks
- Create CI workflow with Rust 1.85 MSRV and stable version testing
- Add comprehensive change management documentation
- Update .gitignore to exclude openspec/changes directory
- Add multi-provider support for OpenAI, Anthropic, and Gemini

Implement provider abstraction with LlmClient trait to support multiple
LLM providers beyond OpenRouter. Add Provider enum with OpenAI-compatible
and Anthropic Messages API clients. Support custom base URLs for Azure
OpenAI, Ollama, and proxy configurations. Maintain backward compatibility
with existing OPENROUTER_* environment variables while adding new
LLM_PROVIDER and LLM_MODEL options.
- Add OpenSpec AI agent instructions file
- Add custom base URL support documentation for multi-provider
- Add openspec/changes to .gitignore

### Miscellaneous Tasks

- Chore: configure release-plz for GitHub-only releases

- Move publish setting from workspace to package level
- Explicitly disable crates.io publishing for jj-desc
- Keep GitHub releases enabled via git_release_enable
