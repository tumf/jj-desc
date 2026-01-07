# cli-interface Specification

## Purpose

Provide a simple, unified command-line interface for jj-desc without subcommands. Users can generate commit descriptions with a single `jj-desc` command, using options like `-r` for revision selection, `-n` for limiting commits, `--dry-run` for preview, and `-i` for interactive mode.
## Requirements
### Requirement: Unified CLI interface

The `jj-desc` command SHALL provide a single, unified interface without subcommands for generating commit descriptions.

#### Scenario: Default execution with no arguments

**Given**: Multiple commits with empty descriptions exist in `::@ & mutable()`
**When**: Execute `jj-desc`
**Then**:
- Descriptions are generated for all commits with empty descriptions
- Processing results are displayed for each commit
- A summary is displayed at the end

#### Scenario: Single commit with -r option

**Given**: A commit `@` with an empty description exists
**When**: Execute `jj-desc -r @`
**Then**:
- Description is generated only for the specified commit
- The generated description is applied

#### Scenario: Multiple commits with revset

**Given**: Multiple commits matching `mutable()` exist
**When**: Execute `jj-desc -r "mutable()"`
**Then**:
- Descriptions are generated for all matching commits with empty descriptions
- Each commit is processed in order

#### Scenario: Limit processing count

**Given**: 20 commits with empty descriptions exist
**When**: Execute `jj-desc -n 5`
**Then**:
- Only the first 5 commits are processed
- Summary displays the number of processed commits

#### Scenario: Dry-run mode

**Given**: Commits with empty descriptions exist
**When**: Execute `jj-desc --dry-run`
**Then**:
- Generated descriptions are displayed
- No descriptions are actually applied

#### Scenario: Interactive mode

**Given**: Multiple commits with empty descriptions exist
**When**: Execute `jj-desc -i`
**Then**:
- User is prompted for each commit
- Options: Accept (a) / Skip (s) / Quit (q)
