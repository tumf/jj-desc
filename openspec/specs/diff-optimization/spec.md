# diff-optimization Specification

## Purpose
TBD - created by archiving change optimize-large-diff-handling. Update Purpose after archive.
## Requirements
### Requirement: Lock File Exclusion

The system MUST automatically exclude common lock files from diff analysis to reduce token consumption and improve response quality.

Default excluded patterns:
- `Cargo.lock`
- `package-lock.json`
- `pnpm-lock.yaml`
- `yarn.lock`
- `*.lock`
- `*.lockb`

#### Scenario: Lock file changes are excluded from diff

**Given** a repository with changes to both `src/main.rs` and `Cargo.lock`
**When** running `jj-desc generate`
**Then** only `src/main.rs` changes are sent to the LLM
**And** `Cargo.lock` changes are excluded from the diff

---

### Requirement: Binary File Summarization

The system MUST detect binary files in the diff and replace their detailed content with a simple summary message.

#### Scenario: Binary file is summarized

**Given** a repository with changes to `image.png` (binary file)
**When** running `jj-desc generate`
**Then** the diff sent to LLM contains `"Binary file image.png changed"` instead of binary content

---

### Requirement: User-Specified Exclusion

The system MUST allow users to specify additional file patterns to exclude via `--exclude` option.

#### Scenario: User excludes JSON files

**Given** a repository with changes to `config.json` and `src/lib.rs`
**When** running `jj-desc generate --exclude "*.json"`
**Then** only `src/lib.rs` changes are sent to the LLM
**And** `config.json` changes are excluded

#### Scenario: Multiple exclude patterns

**Given** a repository with changes to multiple file types
**When** running `jj-desc generate -x "*.json" -x "*.yaml" --exclude "*.md"`
**Then** all specified patterns are excluded from the diff

---

### Requirement: Large Diff Warning

The system MUST display a warning when the diff size exceeds the configured threshold (default: 50KB).

The warning MUST NOT block execution (following aicommit2's approach).

#### Scenario: Warning displayed for large diff

**Given** a diff larger than 50KB
**When** running `jj-desc generate`
**Then** a warning message is displayed: "⚠ Warning: Diff is large (X bytes). Consider splitting into smaller commits."
**And** the command continues to execute

#### Scenario: No warning for small diff

**Given** a diff smaller than 50KB
**When** running `jj-desc generate`
**Then** no warning message is displayed

---

### Requirement: Diff Statistics Display

The system MUST display diff statistics after filtering when verbose mode is enabled.

#### Scenario: Statistics shown in verbose mode

**Given** a diff with some files excluded
**When** running `jj-desc generate --verbose`
**Then** the output includes:
  - Original diff size
  - Filtered diff size
  - Number of excluded files
  - List of excluded file paths

---

