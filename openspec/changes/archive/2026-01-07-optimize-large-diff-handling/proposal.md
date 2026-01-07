# Optimize Large Diff Handling

## Why

Currently, `jj-desc` sends the entire output from `jj diff` directly to the LLM without any filtering or optimization. This causes several problems:

| Issue | Impact |
|-------|--------|
| Context window exceeded | API errors (400/413) |
| Processing timeout | 30-second timeout failures |
| Unnecessary token consumption | High cost from lock files and binary content |

Large diffs containing lock files (e.g., `Cargo.lock`, `package-lock.json`) or binary files consume significant tokens without providing useful information for commit message generation. This leads to API failures, timeouts, and increased costs.

## What Changes

Optimize diff processing by implementing filtering strategies inspired by [aicommit2](https://github.com/tak-bro/aicommit2):

### 1. Automatic Lock File Exclusion

Automatically exclude common lock files from diff analysis:
- `Cargo.lock`
- `package-lock.json`
- `pnpm-lock.yaml`
- `yarn.lock`
- `*.lock`
- `*.lockb`

### 2. Binary File Summarization

Replace binary file content with summary messages (e.g., `"Binary file {path} changed"`) instead of sending raw binary diffs.

### 3. Large Diff Warnings

Display warnings when diff size exceeds a threshold (default: 50KB):
- Show: `Diff size: 150KB (3500 lines)`
- Warn on threshold exceeded
- **Do not block execution** (following aicommit2 approach)

### 4. User-Specified Exclusions

Add `--exclude` CLI option to allow users to specify additional file patterns to exclude from diffs.

### Out of Scope

- Automatic diff truncation
- Token count calculation
- Per-file split processing

## References

- [aicommit2 git.ts](https://github.com/tak-bro/aicommit2/blob/main/src/utils/git.ts)
- [aicommit2 README](https://github.com/tak-bro/aicommit2)
