# merge-detection Specification

## Purpose
This specification defines the functionality to detect merge commits with empty diffs and set appropriate descriptions. When `jj diff` returns an empty output, the system checks if the commit is a merge commit (having 2+ parents) and automatically sets "Merge commit" as the description instead of returning an error. This addresses the common case in jj where merge commits are often treated as "empty" but still require proper documentation.
## Requirements
### Requirement: REQ-MERGE-001: Merge Commit Detection
The system SHALL use `jj log` template functionality to retrieve the number of parents and determine if a commit is a merge commit.

#### Scenario: Regular Commit Detection
- **Given**: A commit with 1 parent
- **When**: `is_merge_commit` function is called
- **Then**: Returns `false`

#### Scenario: Merge Commit Detection
- **Given**: A commit with 2 or more parents
- **When**: `is_merge_commit` function is called
- **Then**: Returns `true`

#### Scenario: Root Commit Detection
- **Given**: A commit with 0 parents (root)
- **When**: `is_merge_commit` function is called
- **Then**: Returns `false`

---

### Requirement: REQ-MERGE-002: Description Setting for Empty Diff Merge Commits
When the diff is empty and the commit is a merge commit, the system SHALL automatically set a description of "Merge commit".

#### Scenario: Merge Commit with Empty Diff
- **Given**: `jj diff` output is empty
- **And**: The commit is a merge commit (2+ parents)
- **When**: `jj-desc` is executed
- **Then**: A description "Merge commit" is set
- **And**: No error occurs

#### Scenario: Non-Merge Commit with Empty Diff
- **Given**: `jj diff` output is empty
- **And**: The commit is a regular commit (1 parent)
- **When**: `jj-desc` is executed
- **Then**: An `EmptyDiff` error is returned
- **And**: The description is not changed

---

### Requirement: REQ-MERGE-003: Empty Diff Error Handling
When encountering an empty diff, the system SHALL check if the commit is a merge commit before returning an error. For merge commits, the system SHALL set "Merge commit" as the description; for non-merge commits, the system SHALL return an `EmptyDiff` error.

#### Scenario: Empty Diff with Merge Commit Check
- **Given**: `jj diff` output is empty
- **When**: Processing the commit
- **Then**: The system SHALL check if it's a merge commit
- **And**: If merge commit (2+ parents), set "Merge commit" description
- **And**: If non-merge commit (1 parent), return `EmptyDiff` error

---

