# Change Proposal: unify-cli-subcommands

## Why

The current CLI has two subcommands that provide essentially identical functionality:
- `generate`: Generates description for a single commit (`--revision` option)
- `backfill`: Generates descriptions for multiple commits (`--revisions` option)

This separation creates unnecessary complexity:
- Users must remember which subcommand to use for single vs. multiple commits
- Code duplication between the two command implementations
- Interface is less intuitive than `jj describe`, which uses a single command with `-r`

Unifying these subcommands will:
- Reduce cognitive load by eliminating the single/multiple distinction
- Provide a more intuitive interface consistent with jj conventions
- Reduce code duplication and simplify maintenance

## What Changes

### CLI Interface

Remove the `generate` and `backfill` subcommands and provide all options directly on the main command:

```bash
# Before
jj-desc generate --revision @
jj-desc backfill --revisions "mutable()"

# After
jj-desc -r @
jj-desc -r "mutable()"
```

### Command-Line Options

- Remove: `generate` and `backfill` subcommands
- Add: `-r/--revisions` option (accepts any revset, defaults to `::@ & mutable()`)
- Consolidate: All LLM options (`--provider`, `--model`, etc.) directly on main command
- Maintain: `-n/--limit` and `-i/--interactive` options

### Usage Examples

```bash
jj-desc                           # Default: ::@ & mutable() commits with empty descriptions
jj-desc -r @                      # Single commit (current working copy)
jj-desc -r "mutable()"            # Multiple commits
jj-desc -r @ --dry-run            # Preview mode
jj-desc -r "::@" -n 5             # Process up to 5 commits
jj-desc -r @ --interactive        # Interactive mode
```

### Implementation Scope

**Code Changes:**
- `src/cli.rs`: Remove `Command` enum, merge all options into `Args` struct
- `src/main.rs`: Remove subcommand dispatch, call unified `execute()` directly
- `src/commands/mod.rs`: Merge `execute_generate` and `execute_backfill` into single function

**Documentation Updates:**
- README.md: Update all command examples
- Help messages: Consolidated help output

**Specification Updates:**
- Create new `cli-interface` spec documenting the unified interface
- Update related specs that reference subcommands

### Backward Compatibility

**Breaking Changes:**
- `jj-desc generate` and `jj-desc backfill` will no longer work
- Users must migrate to `jj-desc -r <revset>`

**Migration Path:**
```bash
# Old → New
jj-desc generate              → jj-desc -r @
jj-desc generate --revision @ → jj-desc -r @
jj-desc backfill              → jj-desc
jj-desc backfill -r "::@"     → jj-desc -r "::@"
```

### Related Specifications

- Adds: `openspec/specs/cli-interface/spec.md` (new)
- Updates: `openspec/specs/backfill-descriptions/spec.md` (references to subcommands)
