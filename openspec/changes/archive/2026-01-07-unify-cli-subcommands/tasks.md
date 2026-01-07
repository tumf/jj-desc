# Tasks: unify-cli-subcommands

## Implementation Tasks

- [x] 1. Change CLI definition (`src/cli.rs`)
  - Remove `Command` enum and `GenerateArgs`, `BackfillArgs`
  - Define all options directly in `Args` struct
  - `-r/--revisions` option (default: `::@ & mutable()`)
  - `-n/--limit` option
  - `-i/--interactive` option
  - Maintain existing common options (`--dry-run`, `--provider`, `--model`, etc.)

- [x] 2. Unify command execution logic (`src/commands/mod.rs`)
  - Merge `execute_generate` and `execute_backfill` into a single `execute` function
  - Remove subcommand dispatch logic

- [x] 3. Simplify main.rs
  - Remove `match args.command` dispatch
  - Call `commands::execute(args)` directly

- [x] 4. Update tests (`src/cli.rs` tests)
  - Remove subcommand tests
  - Add tests for new option structure
  - `test_generate_subcommand` → Remove
  - `test_backfill_subcommand` → Remove
  - New: `test_revisions_option`, `test_limit_option`, etc.

- [x] 5. Update specification (`openspec/specs/backfill-descriptions/spec.md`)
  - Change `jj-desc backfill` to `jj-desc`
  - Update scenarios

- [x] 6. Update documentation
  - Update command examples in README.md
  - Verify `--help` output

## Verification

- [x] All `cargo test` passes
- [x] `cargo clippy --all-features -- -D warnings` passes without warnings
- [x] `jj-desc --help` displays correctly
- [x] `jj-desc -r @` works
- [x] `jj-desc` (no arguments) uses default behavior
