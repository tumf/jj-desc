# CLI Interface Specification

## ADDED Requirements

### Requirement: Command Line Interface

The `jj-desc` CLI MUST provide a simple interface for generating commit descriptions from jj diffs using LLM.

**Usage:**
```
jj-desc [OPTIONS]
```

**Options:**

| Option | Short | Description |
|--------|-------|-------------|
| `--dry-run` | | Preview generated description without applying |
| `--model <MODEL>` | | Override the LLM model to use |
| `--revision <REV>` | `-r` | Specify target revision (default: current) |
| `--verbose` | `-v` | Enable debug logging |
| `--help` | `-h` | Display help information |
| `--version` | `-V` | Display version information |

#### Scenario: Basic usage (immediate apply)

**Given** the user has uncommitted changes in a jj repository
**And** `OPENROUTER_API_KEY` is set
**When** the user runs `jj-desc`
**Then** the tool generates a description using LLM
**And** immediately applies it via `jj desc`
**And** displays the applied description

#### Scenario: Preview with --dry-run flag

**Given** the user has uncommitted changes in a jj repository
**When** the user runs `jj-desc --dry-run`
**Then** the tool displays the generated description
**And** does not apply the description

#### Scenario: Specify revision

**Given** the user has a jj repository with revision `abc123`
**When** the user runs `jj-desc -r abc123`
**Then** the tool generates description for the specified revision

#### Scenario: Override model

**Given** the user has uncommitted changes
**When** the user runs `jj-desc --model openai/gpt-4o`
**Then** the tool uses the specified model for generation

#### Scenario: Verbose output

**Given** the user has uncommitted changes
**When** the user runs `jj-desc --verbose`
**Then** debug-level logs are displayed
**And** the description is generated and applied

---

### Requirement: Output Format

The tool MUST display the generated/applied description clearly.

**Display format (on success):**
```
Applied description:
─────────────────────
{applied description text}
```

**Display format (dry-run):**
```
Generated description (not applied):
─────────────────────
{generated description text}
```

#### Scenario: Display applied description

**Given** the description "Add user authentication" is generated
**When** the description is applied successfully
**Then** the output shows "Applied description:" followed by the text

#### Scenario: Display dry-run description

**Given** the description "Add user authentication" is generated
**And** `--dry-run` flag is used
**Then** the output shows "Generated description (not applied):" followed by the text

---

### Requirement: Exit Codes

The CLI MUST use appropriate exit codes to indicate success or failure.

| Exit Code | Meaning |
|-----------|---------|
| 0 | Success |
| 1 | Error (configuration, API, jj command failure) |

#### Scenario: Successful execution

**Given** all operations complete successfully
**Then** the tool exits with code 0

#### Scenario: Error during execution

**Given** an error occurs (missing API key, API failure, etc.)
**Then** the tool displays an error message to stderr
**And** exits with code 1
