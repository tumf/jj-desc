# jj-desc

Generate [jj (Jujutsu)](https://github.com/martinvonz/jj) commit descriptions automatically using LLMs.

## Features

- 🤖 Automatically generates meaningful commit descriptions from diffs using LLMs
- 📦 **Backfill mode**: Generate descriptions for multiple commits at once
- 🔄 Works seamlessly with jj's undo workflow (no confirmation prompts needed)
- 🎯 Supports multiple LLM providers: OpenRouter, OpenAI, Anthropic, Gemini
- 🔌 Custom endpoint support (Azure OpenAI, Ollama, proxies, etc.)
- 🔍 Preview mode with `--dry-run`
- 💬 Interactive mode for reviewing each description before applying
- 🎚️ Flexible targeting with jj revset syntax
- 📝 Follows git commit message best practices
- 🔀 Handles merge commits automatically (no LLM call needed for empty merge commits)

## Installation

### From source

```bash
cargo install --path .
```

### Requirements

- Rust 1.85+ (Edition 2024)
- [jj](https://github.com/martinvonz/jj) installed and available in PATH
- API key for your chosen LLM provider

## Configuration

### LLM Provider Selection

Choose your LLM provider using the `LLM_PROVIDER` environment variable or `--provider` CLI option:

```bash
export LLM_PROVIDER=openai        # Options: openrouter, openai, anthropic, gemini
```

### Environment Variables

#### Common Settings

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `LLM_PROVIDER` | ❌ | `openrouter` | LLM provider to use |
| `LLM_MODEL` | ❌ | (provider default) | Override the model |

#### Provider-Specific API Keys

| Provider | Environment Variable | Get API Key |
|----------|---------------------|-------------|
| OpenRouter | `OPENROUTER_API_KEY` | [OpenRouter](https://openrouter.ai/) |
| OpenAI | `OPENAI_API_KEY` | [OpenAI Platform](https://platform.openai.com/) |
| Anthropic | `ANTHROPIC_API_KEY` | [Anthropic Console](https://console.anthropic.com/) |
| Gemini | `GEMINI_API_KEY` | [Google AI Studio](https://aistudio.google.com/) |

#### Provider-Specific Base URLs (Optional)

Override the default API endpoint for custom setups:

| Provider | Environment Variable | Default Value |
|----------|---------------------|---------------|
| OpenRouter | `OPENROUTER_BASE_URL` | `https://openrouter.ai/api/v1` |
| OpenAI | `OPENAI_BASE_URL` | `https://api.openai.com/v1` |
| Anthropic | `ANTHROPIC_BASE_URL` | `https://api.anthropic.com` |
| Gemini | `GEMINI_BASE_URL` | `https://generativelanguage.googleapis.com/v1beta/openai` |

#### Default Models by Provider

| Provider | Default Model |
|----------|--------------|
| OpenRouter | `anthropic/claude-sonnet-4` |
| OpenAI | `gpt-4o` |
| Anthropic | `claude-sonnet-4-20250514` |
| Gemini | `gemini-2.0-flash` |

### Setup Examples

#### OpenRouter (Default)

```bash
export OPENROUTER_API_KEY="your-api-key-here"
```

#### OpenAI

```bash
export LLM_PROVIDER=openai
export OPENAI_API_KEY="sk-..."
```

#### Anthropic

```bash
export LLM_PROVIDER=anthropic
export ANTHROPIC_API_KEY="sk-ant-..."
```

#### Gemini

```bash
export LLM_PROVIDER=gemini
export GEMINI_API_KEY="..."
```

#### Azure OpenAI

```bash
export LLM_PROVIDER=openai
export OPENAI_API_KEY="your-azure-key"
export OPENAI_BASE_URL="https://your-resource.openai.azure.com/openai/deployments/your-deployment"
export LLM_MODEL="gpt-4"
```

#### Ollama (Local LLM)

```bash
export LLM_PROVIDER=openai
export OPENAI_API_KEY="dummy"  # Ollama doesn't require a key
export OPENAI_BASE_URL="http://localhost:11434/v1"
export LLM_MODEL="llama2"
```

For permanent setup, add these to your shell configuration (`~/.bashrc`, `~/.zshrc`, etc.).

## Usage

### Basic usage

Generate and apply a description for the current working copy:

```bash
jj-desc
# or explicitly
jj-desc generate
```

### Backfill descriptions for multiple commits

Generate descriptions for all commits without descriptions:

```bash
jj-desc backfill
```

#### Backfill options

Target specific revisions using jj's revset syntax:

```bash
# Backfill your own commits
jj-desc backfill --revisions "mine()"

# Backfill commits in a specific range
jj-desc backfill --revisions "@..main"

# Limit the number of commits to process
jj-desc backfill --limit 5

# Preview before applying
jj-desc backfill --dry-run

# Interactive mode - confirm each description
jj-desc backfill --interactive
```

### Preview mode

See what description would be generated without applying it:

```bash
jj-desc --dry-run
jj-desc generate --dry-run
```

### Target specific revision

Generate description for a specific revision:

```bash
jj-desc generate -r @-
```

### Use a different provider

Override the provider:

```bash
jj-desc --provider openai
jj-desc backfill --provider anthropic
```

### Use a different model

Override the default model:

```bash
jj-desc --model gpt-4o
# or    
jj-desc --model anthropic/claude-3.5-sonnet
```

### Verbose logging

Enable detailed logging for debugging:

```bash
jj-desc --verbose
```

Or use the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug jj-desc
```

### Command-line options

#### Main command

```
Usage: jj-desc [OPTIONS] [COMMAND]

Commands:
  generate  Generate description for a single commit (default)
  backfill  Backfill descriptions for multiple commits
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Enable verbose logging
  -h, --help     Print help
  -V, --version  Print version
```

#### Generate subcommand

```
Usage: jj-desc generate [OPTIONS]

Options:
      --dry-run                  Preview the generated description without applying it
      --provider <PROVIDER>      LLM provider to use [env: LLM_PROVIDER]
      --model <MODEL>            Override the LLM model to use [env: LLM_MODEL]
  -r, --revision <REVISION>      Target revision (defaults to current working copy)
  -h, --help                     Print help
```

#### Backfill subcommand

```
Usage: jj-desc backfill [OPTIONS]

Options:
      --dry-run                    Preview the generated descriptions without applying them
      --provider <PROVIDER>        LLM provider to use [env: LLM_PROVIDER]
      --model <MODEL>              Override the LLM model to use [env: LLM_MODEL]
  -r, --revisions <REVISIONS>      Revset to select target commits [default: mutable()]
  -n, --limit <LIMIT>              Maximum number of commits to process
  -i, --interactive                Ask for confirmation before applying each description
  -h, --help                       Print help
```

## Examples

### Example 1: Basic workflow

```bash
# Make some changes
echo "fn hello() {}" >> lib.rs

# Generate description
jj-desc

# Output:
# Applied description:
# ─────────────────────
# Add hello function to lib.rs
```

### Example 2: Preview before applying

```bash
jj-desc --dry-run

# Output:
# Generated description (not applied):
# ─────────────────────
# Add user authentication with JWT tokens
```

### Example 3: Revert if needed

```bash
jj-desc
# (description applied)

# Don't like it? Just undo!
jj undo
```

### Example 4: Backfill multiple commits

```bash
# Fill descriptions for all mutable commits without descriptions
jj-desc backfill

# Output:
# Found 3 commit(s) without descriptions
# Processing 3 commit(s)
# 
# Processing: 1/3 (33%)
# Commit: abc123def456
# Generated description:
#   Add user authentication endpoint
# ✓ Description applied
# 
# Processing: 2/3 (66%)
# Commit: def456ghi789
# Generated description:
#   Fix validation bug in login form
# ✓ Description applied
# 
# Processing: 3/3 (100%)
# Commit: ghi789jkl012
# Generated description:
#   Update dependencies
# ✓ Description applied
# 
# ═══════════════════════
# Summary:
#   Success:  3
#   Skipped:  0
#   Failed:   0
# ═══════════════════════
```

### Example 5: Interactive backfill

```bash
jj-desc backfill --interactive --revisions "mine()"

# For each commit, you'll see:
# Processing: 1/5 (20%)
# Commit: abc123
# Generated description:
#   Add user authentication
# 
# Full description:
# ─────────────────────
# Add user authentication with JWT tokens
# 
# Implements login and logout endpoints with secure
# token generation and validation.
# ─────────────────────
# Accept (a) / Skip (s) / Quit (q): a
# ✓ Description applied
```

## How it works

1. Runs `jj diff` to get the current changes
2. If the diff is empty:
   - Checks if it's a merge commit (using `jj log -T 'parents.len()'`)
   - If yes, sets description to "Merge commit" without calling LLM
   - If no, returns an error
3. If the diff is not empty, sends it to your chosen LLM provider API with a specialized prompt
4. Applies the generated description using `jj desc -m`

### Merge Commit Handling

jj often marks merge commits as "empty" because they don't introduce new changes themselves (see [jj FAQ](https://docs.jj-vcs.dev/latest/FAQ/#why-are-most-merge-commits-marked-as-empty)). `jj-desc` detects merge commits automatically and sets an appropriate description without requiring LLM API calls.

## Why no confirmation prompt?

Unlike git, jj makes it extremely easy to undo any operation:

- `jj undo` - Undo the last operation
- `jj op log` - View operation history
- All changes are recoverable

This design philosophy means we can safely apply descriptions immediately, making the workflow faster and more streamlined.

## Development

### Build

```bash
cargo build --release
```

### Run tests

```bash
cargo test
```

### Set up pre-commit hooks

```bash
# Install pre-commit (if not already installed)
pip install pre-commit
# or on macOS
brew install pre-commit

# Install hooks
./scripts/pre-commit-install.sh
```

The pre-commit hooks include:
- `cargo fmt` - Format code on commit
- `cargo clippy` - Lint code on commit
- `cargo test` - Run tests on push

### Project structure

```
src/
├── main.rs         # Entry point and command dispatching
├── cli.rs          # Command-line argument parsing (subcommands)
├── commands/       # Command implementations
│   ├── mod.rs          # Command module exports
│   ├── generate.rs     # Generate single commit description
│   └── backfill.rs     # Backfill multiple commit descriptions
├── config.rs       # Configuration management
├── provider.rs     # Provider enumeration
├── error.rs        # Error type definitions
├── jj.rs           # jj command integration
├── llm/
│   ├── mod.rs          # LLM client trait and factory
│   ├── openai_compat.rs # OpenAI-compatible client
│   └── anthropic.rs     # Anthropic Messages API client
└── prompt.rs       # LLM prompt generation
```

## Technology Stack

- **Rust Edition 2024** (requires Rust 1.85+)
- **clap** - CLI argument parsing
- **tokio** - Async runtime
- **async-trait** - Trait support for async methods
- **reqwest** - HTTP client (rustls-tls, no OpenSSL dependency)
- **serde** - JSON serialization
- **thiserror** / **anyhow** - Error handling
- **tracing** - Structured logging

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Migration Guide

### For Existing Users (v0.1.x)

The previous version only supported OpenRouter. The new version maintains full backward compatibility:

- Existing `OPENROUTER_API_KEY` environment variable continues to work
- Existing `OPENROUTER_MODEL` environment variable continues to work
- No changes needed to your configuration

To take advantage of new providers, simply set `LLM_PROVIDER` and the appropriate API key.

## See Also

- [jj documentation](https://martinvonz.github.io/jj/)
- [OpenRouter](https://openrouter.ai/)
- [OpenAI Platform](https://platform.openai.com/)
- [Anthropic Console](https://console.anthropic.com/)
- [Google AI Studio](https://aistudio.google.com/)
