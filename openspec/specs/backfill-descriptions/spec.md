# backfill-descriptions Specification

## Purpose
TBD - created by archiving change backfill-empty-descriptions. Update Purpose after archive.
## Requirements
### Requirement: Batch description generation for multiple commits

The `jj-desc backfill` command SHALL generate and set descriptions for commits with empty descriptions within a specified revset using LLM.

#### Scenario: Batch processing of default mutable commits

**Given**: Multiple commits with empty descriptions exist in the repository
**When**: Execute `jj-desc backfill`
**Then**:
- Descriptions are set for all commits with empty descriptions in `mutable()`
- Processing results (success/failure) are displayed for each commit
- A summary showing success and failure counts is displayed at the end

#### Scenario: Specify target range with revset

**Given**: Multiple branches and commits exist in the repository
**When**: Execute `jj-desc backfill --revisions "mine()"`
**Then**:
- Only commits created by the current user with empty descriptions are processed
- Commits created by others or commits with existing descriptions are ignored

#### Scenario: Preview with dry-run mode

**Given**: 5 commits with empty descriptions exist
**When**: Execute `jj-desc backfill --dry-run`
**Then**:
- Generated descriptions for each commit are displayed
- The `jj describe` command is not actually executed
- Display includes indicators like "Generated description (not applied):"

#### Scenario: Limit processing count

**Given**: 20 commits with empty descriptions exist
**When**: Execute `jj-desc backfill --limit 5`
**Then**:
- Only the first 5 commits are processed
- Remaining 15 commits are ignored
- Summary displays the number of processed commits

### Requirement: Interactive mode

The `--interactive` option SHALL allow users to review generated descriptions for each commit and choose to apply, skip, or edit them.

#### Scenario: Individual confirmation in interactive mode

**Given**: 3 commits with empty descriptions exist
**When**: Execute `jj-desc backfill --interactive`
**Then**:
- The diff and generated description for the first commit are displayed
- User is presented with options: "Accept (a) / Skip (s) / Edit (e) / Quit (q)"
- Entering 'a' sets the description and proceeds to the next commit
- Entering 's' skips setting the description and proceeds to the next commit
- Entering 'e' opens `$EDITOR` for manual editing before setting
- Entering 'q' aborts the processing

### Requirement: Error handling

Processing SHALL continue for remaining commits even if individual commit processing fails.

#### Scenario: Continue on API error

**Given**: 5 commits with empty descriptions exist
**When**: An API error occurs during processing of the 3rd commit
**Then**:
- An error message is displayed for the 3rd commit
- Processing continues for the 4th and 5th commits
- Final summary displays "Success: 4, Failed: 1"

### Requirement: Backward compatibility

The tool SHALL maintain existing behavior of the `jj-desc` command (without subcommand).

#### Scenario: Compatibility with no-argument execution

**Given**: Existing users execute `jj-desc` without arguments
**When**: Execute `jj-desc` (no subcommand)
**Then**:
- Generates description for the current commit (`@`) (existing behavior)
- No errors occur

#### Scenario: Compatibility with existing options

**Given**: Existing options (`--revision`, `--dry-run`, etc.) are used
**When**: Execute `jj-desc --revision abc123`
**Then**:
- Generates description for the specified commit
- Behaves identically to the `generate` subcommand

