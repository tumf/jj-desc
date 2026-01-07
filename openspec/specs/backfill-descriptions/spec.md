# backfill-descriptions Specification

## Purpose

Enable batch generation of commit descriptions for multiple commits with empty descriptions. Users can specify target commits via revset, preview changes with `--dry-run`, limit processing count with `--limit`, and interactively review each description with `--interactive` mode. Processing continues even when individual commits fail.
## Requirements
### Requirement: Batch description generation for multiple commits

The `jj-desc` command SHALL generate and set descriptions for commits with empty descriptions within a specified revset using LLM.

#### Scenario: Batch processing of default mutable commits

**Given**: Multiple commits with empty descriptions exist in the repository
**When**: Execute `jj-desc`
**Then**:
- Descriptions are set for all commits with empty descriptions in `::@ & mutable()`
- Processing results (success/failure) are displayed for each commit
- A summary showing success and failure counts is displayed at the end

#### Scenario: Specify target range with revset

**Given**: Multiple branches and commits exist in the repository
**When**: Execute `jj-desc --revisions "mine()"`
**Then**:
- Only commits created by the current user with empty descriptions are processed
- Commits created by others or commits with existing descriptions are ignored

#### Scenario: Preview with dry-run mode

**Given**: 5 commits with empty descriptions exist
**When**: Execute `jj-desc --dry-run`
**Then**:
- Generated descriptions for each commit are displayed
- The `jj describe` command is not actually executed
- Display includes indicators like "Generated description (not applied):"

#### Scenario: Limit processing count

**Given**: 20 commits with empty descriptions exist
**When**: Execute `jj-desc --limit 5`
**Then**:
- Only the first 5 commits are processed
- Remaining 15 commits are ignored
- Summary displays the number of processed commits

### Requirement: Interactive mode

The `--interactive` option SHALL allow users to review generated descriptions for each commit and choose to apply, skip, or edit them.

#### Scenario: Individual confirmation in interactive mode

**Given**: 3 commits with empty descriptions exist
**When**: Execute `jj-desc --interactive`
**Then**:
- The diff and generated description for the first commit are displayed
- User is presented with options: "Accept (a) / Skip (s) / Quit (q)"
- Entering 'a' sets the description and proceeds to the next commit
- Entering 's' skips setting the description and proceeds to the next commit
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

### Requirement: Simplified CLI interface

The tool SHALL provide a unified interface without subcommands.

#### Scenario: Default behavior with no arguments

**Given**: Multiple commits with empty descriptions exist in the repository
**When**: Execute `jj-desc` (no arguments)
**Then**:
- Processes all commits with empty descriptions in `::@ & mutable()` (default revset)
- No errors occur

#### Scenario: Single commit processing

**Given**: User wants to generate description for a specific commit
**When**: Execute `jj-desc -r @`
**Then**:
- Generates description only for the current working copy commit
- Behaves the same as processing a single-commit revset

