# jj Integration Specification

## ADDED Requirements

### Requirement: Get Diff from jj

The tool MUST retrieve the current diff using the `jj diff` command.

**Command:** `jj diff [-r <revision>]`

#### Scenario: Get diff for current changes

**Given** the current directory is a jj repository
**And** there are uncommitted changes
**When** the tool runs `jj diff`
**Then** the diff output is captured successfully

#### Scenario: Get diff for specific revision

**Given** the current directory is a jj repository
**And** revision `abc123` exists
**When** the tool runs `jj diff -r abc123`
**Then** the diff for that revision is captured

#### Scenario: No changes present

**Given** the current directory is a jj repository
**And** there are no changes (diff is empty)
**When** the tool runs `jj diff`
**Then** an error message is displayed: "Error: No changes found in diff"
**And** the tool exits with code 1

#### Scenario: jj command not found

**Given** `jj` is not installed or not in PATH
**When** the tool attempts to run `jj diff`
**Then** an error message is displayed indicating jj is not available
**And** the tool exits with code 1

#### Scenario: Not a jj repository

**Given** the current directory is not a jj repository
**When** the tool runs `jj diff`
**Then** the jj error output is displayed
**And** the tool exits with code 1

---

### Requirement: Apply Description via jj

The tool MUST apply the generated description using the `jj desc` command.

**Command:** `jj desc -m "{description}" [-r <revision>]`

#### Scenario: Apply description to current changes

**Given** a generated description "Add user authentication"
**When** the tool runs `jj desc -m "Add user authentication"`
**Then** the description is applied successfully

#### Scenario: Apply description to specific revision

**Given** a generated description and revision `abc123`
**When** the tool runs `jj desc -m "{description}" -r abc123`
**Then** the description is applied to the specified revision

#### Scenario: jj desc command fails

**Given** the `jj desc` command fails for any reason
**When** the tool processes the result
**Then** the jj error output is displayed
**And** the tool exits with code 1

---

### Requirement: Description Escaping

The generated description MUST be properly escaped when passed to the shell.

#### Scenario: Description with quotes

**Given** a description containing double quotes: `Add "feature" support`
**When** the description is passed to `jj desc`
**Then** the quotes are properly escaped
**And** the description is applied correctly

#### Scenario: Description with special characters

**Given** a description containing special characters: `Fix bug in $PATH handling`
**When** the description is passed to `jj desc`
**Then** the special characters are properly handled
**And** the description is applied correctly
