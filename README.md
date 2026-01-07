# jj-desc

Generate [jj (Jujutsu)](https://github.com/martinvonz/jj) commit descriptions automatically using LLMs.

## Features

- 🤖 Automatically generates meaningful commit descriptions from diffs using LLMs
- 📦 Process single or multiple commits with a unified CLI interface
- 🔄 Works seamlessly with jj's undo workflow (no confirmation prompts needed)
- 🎯 Supports multiple LLM providers: OpenRouter, OpenAI, Anthropic, Gemini
- 🔌 Custom endpoint support (Azure OpenAI, Ollama, proxies, etc.)
- 🔍 Preview mode with `--dry-run`
- 💬 Interactive mode for reviewing each description before applying
- 🎚️ Flexible targeting with jj revset syntax
- 📝 Follows git commit message best practices
- 🔀 Handles merge commits automatically (no LLM call needed for empty merge commits)

## Installation

### Homebrew (macOS/Linux)

The recommended way to install on macOS or Linux:

```bash
brew install tumf/tap/jj-desc
```

### Pre-built Binaries

Download pre-built binaries for your platform from the [latest release](https://github.com/tumf/jj-desc/releases/latest).

#### macOS

**Using installer script (recommended):**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.sh | sh
```

**Manual download:**

- Apple Silicon (M1/M2/M3): [jj-desc-aarch64-apple-darwin.tar.xz](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-aarch64-apple-darwin.tar.xz)
- Intel: [jj-desc-x86_64-apple-darwin.tar.xz](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-apple-darwin.tar.xz)

```bash
# Example for Apple Silicon
curl -LO https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-aarch64-apple-darwin.tar.xz
tar xf jj-desc-aarch64-apple-darwin.tar.xz
sudo mv jj-desc /usr/local/bin/
```

#### Linux

**Using installer script (recommended):**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.sh | sh
```

**Manual download:**

- x86_64: [jj-desc-x86_64-unknown-linux-gnu.tar.xz](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-unknown-linux-gnu.tar.xz)
- ARM64: [jj-desc-aarch64-unknown-linux-gnu.tar.xz](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-aarch64-unknown-linux-gnu.tar.xz)

```bash
# Example for x86_64
curl -LO https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-unknown-linux-gnu.tar.xz
tar xf jj-desc-x86_64-unknown-linux-gnu.tar.xz
sudo mv jj-desc /usr/local/bin/
```

#### Windows

**Using PowerShell installer (recommended):**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.ps1 | iex"
```

**Manual download:**

Download [jj-desc-x86_64-pc-windows-msvc.zip](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-pc-windows-msvc.zip) and extract `jj-desc.exe` to a directory in your PATH.

### From Source (Cargo)

If you have Rust installed:

```bash
cargo install --git https://github.com/tumf/jj-desc
```

Or build from a local clone:

```bash
git clone https://github.com/tumf/jj-desc
cd jj-desc
cargo install --path .
```

### Requirements

- [jj](https://github.com/martinvonz/jj) installed and available in PATH
- API key for your chosen LLM provider
- For building from source: Rust 1.85+ (Edition 2024)

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

By default, `jj-desc` generates descriptions for all mutable commits without descriptions in `::@ & mutable()`:

```bash
jj-desc
```

### Generate description for a single commit

Generate and apply a description for the current working copy:

```bash
jj-desc -r @
```

### Target specific revisions

Target specific revisions using jj's revset syntax:

```bash
# Process your own commits
jj-desc -r "mine()"

# Process commits in a specific range
jj-desc -r "@..main"

# Process a single specific revision
jj-desc -r @-

# Limit the number of commits to process
jj-desc -n 5

# Preview before applying
jj-desc --dry-run

# Interactive mode - confirm each description
jj-desc --interactive
```

### Preview mode

See what description would be generated without applying it:

```bash
jj-desc --dry-run
```

### Use a different provider

Override the provider:

```bash
jj-desc --provider openai
jj-desc --provider anthropic
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

```
Usage: jj-desc [OPTIONS]

Options:
  -v, --verbose                  Enable verbose logging
      --dry-run                  Preview the generated descriptions without applying them
      --provider <PROVIDER>      LLM provider to use [env: LLM_PROVIDER]
      --model <MODEL>            Override the LLM model to use [env: LLM_MODEL]
      --max-tokens <MAX_TOKENS>  Maximum tokens for LLM response [env: LLM_MAX_TOKENS]
      --temperature <TEMPERATURE> Temperature for LLM response (0.0-2.0) [env: LLM_TEMPERATURE]
  -r, --revisions <REVISIONS>    Revset to select target commits [default: "::@ & mutable()"]
  -n, --limit <LIMIT>            Maximum number of commits to process
  -i, --interactive              Ask for confirmation before applying each description
  -h, --help                     Print help
  -V, --version                  Print version
```

## Examples

### Example 1: Basic workflow

```bash
# Make some changes
echo "fn hello() {}" >> lib.rs

# Generate description for current working copy
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

### Example 3: Process multiple commits

```bash
# Fill descriptions for all mutable commits without descriptions
jj-desc

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

### Example 4: Interactive mode

```bash
jj-desc --interactive -r "mine()"

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

The pre-commit hooks run the **same checks as CI**:
- **On every commit:**
  - `cargo fmt --check` - Validate code formatting (fails if not formatted)
  - `cargo clippy --all-features -- -D warnings` - Lint with zero warnings
- **On push:**
  - `cargo test --all-features` - Run all tests

To manually run all checks: `pre-commit run --all-files`

### Release process

This project uses `cargo-release` and `cargo-dist` for releases:

```bash
# Install tools (one-time setup)
cargo install cargo-release git-cliff

# Create a release (example: minor version bump 0.2.0 → 0.3.0)
cargo release minor --execute

# Push tag to trigger GitHub release build
git push --follow-tags
```

See [AGENTS.md](AGENTS.md#release-process) for detailed release instructions.

### Project structure

```
src/
├── main.rs         # Entry point and logging setup
├── cli.rs          # Command-line argument parsing
├── commands/       # Command implementation
│   └── mod.rs          # Unified command execution
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
