# merge-detection Specification Delta

## MODIFIED Requirements

### Requirement: REQ-MERGE-002: Description Setting for Empty Diff Merge Commits

When the diff is empty and the commit is a merge commit, the system SHALL automatically set a description of "Merge branches".

#### Scenario: Merge Commit with Empty Diff
- **Given**: `jj diff` output is empty
- **And**: The commit is a merge commit (2+ parents)
- **When**: `jj-desc` is executed
- **Then**: A description "Merge branches" is set
- **And**: No error occurs

#### Scenario: Non-Merge Commit with Empty Diff
- **Given**: `jj diff` output is empty
- **And**: The commit is a regular commit (1 parent)
- **When**: `jj-desc` is executed
- **Then**: The commit is skipped with a message "(empty non-merge commit)"
- **And**: No description is set
- **And**: Skip count is incremented

---

### Requirement: REQ-MERGE-003: Empty Diff Handling

When encountering an empty diff, the system SHALL check if the commit is a merge commit. For merge commits, the system SHALL set "Merge branches" as the description; for non-merge commits, the system SHALL skip the commit without setting any description.

#### Scenario: Empty Diff with Merge Commit Check
- **Given**: `jj diff` output is empty
- **When**: Processing the commit
- **Then**: The system SHALL check if it's a merge commit
- **And**: If merge commit (2+ parents), set "Merge branches" description
- **And**: If non-merge commit (1 parent), skip the commit and log a message

---

## REMOVED Requirements

### ~~EMPTY_NON_MERGE_DESCRIPTION constant~~

The `EMPTY_NON_MERGE_DESCRIPTION` constant (`"(empty commit)"`) is removed as it is no longer used.
