# backfill-empty-descriptions

## Why

Currently, `jj-desc` only generates descriptions for a single commit (default: `@`). However, there are several use cases that require setting descriptions for historical commits:

- When adopting `jj-desc` in an existing repository, users need to batch-process historical commits with empty descriptions
- During development, users may skip descriptions and want to organize them later in bulk
- Before code review, users need to generate descriptions for multiple commits at once

This feature enables users to retroactively generate and set LLM-powered descriptions for commits that lack them, improving repository documentation and commit history quality.

## What Changes

### New Command: `jj-desc backfill`

Adds a new subcommand that generates and sets descriptions for multiple historical commits in batch.

**Key Options:**
- `--revisions` / `-r`: Specify target range using jj revset syntax (default: `mutable()`)
- `--dry-run`: Preview generated descriptions without applying them
- `--limit` / `-n`: Limit the number of commits to process
- `--interactive` / `-i`: Prompt for confirmation on each commit
- `--verbose` / `-v`: Enable detailed logging

### CLI Structure Refactoring

Refactor from single-command to subcommand structure:
- `jj-desc` → `jj-desc generate` (existing functionality, maintained as default behavior)
- `jj-desc backfill` → new batch processing feature

### Error Handling

Implement resilient error handling that skips failed commits and continues processing remaining commits.

### Non-Functional Enhancements

- Progress display for large batch operations
- Rate limiting considerations for API calls (delay options)
- Safe interruption handling (Ctrl+C safe)

## Acceptance Criteria

1. `jj-desc backfill` successfully generates and sets descriptions for commits with empty descriptions
2. `--revisions` allows flexible target range specification
3. `--dry-run` provides preview before execution
4. `--interactive` enables per-commit confirmation, skipping, and editing
5. Error handling skips failed commits and continues processing
6. Existing `jj-desc` command (no arguments) maintains backward compatibility

## Impact Scope

- CLI structure (clap subcommand migration)
- jj command execution logic (multi-commit processing)
- Error handling (partial failure management)
