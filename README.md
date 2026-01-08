# jj-desc

Generate [jj (Jujutsu)](https://github.com/martinvonz/jj) commit descriptions automatically using LLMs.

English | [日本語](README.ja.md)

## Features

- 🤖 Automatically generates meaningful commit descriptions from diffs using LLMs
- 📦 **Batch processing**: Process multiple commits at once with revset targeting
- 🔄 Works seamlessly with jj's undo workflow (no confirmation prompts needed)
- 🎯 Supports multiple LLM providers: OpenRouter, OpenAI, Anthropic, Gemini
- 🔌 Custom endpoint support (Azure OpenAI, Ollama, LM Studio, etc.)
- 🔍 Preview mode with `--dry-run`
- 💬 Interactive mode for reviewing each description before applying
- 🎚️ Flexible targeting with jj revset syntax
- 📝 Follows [Conventional Commits](https://www.conventionalcommits.org/) format
- 🔀 Handles merge commits automatically (no LLM call needed for empty merge commits)
- ⚡ Optimized for large diffs: automatically excludes lock files and simplifies binary files
- 🎛️ Customizable file exclusions with `--exclude` option

## Demo

![jj-desc demo](docs/demo.gif)

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

#### LM Studio (Local LLM)

```bash
export LLM_PROVIDER=openai
export OPENAI_API_KEY="dummy"  # LM Studio doesn't require a key
export OPENAI_BASE_URL="http://localhost:1234/v1"
export LLM_MODEL="your-model-name"
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
jj-desc -i
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

### Exclude files from diff

Exclude specific files or patterns from the diff sent to the LLM:

```bash
# Exclude specific files
jj-desc --exclude "*.json" --exclude "*.yaml"

# Short form
jj-desc -x "docs/*" -x "*.lock"
```

**Automatically excluded files:**
- Lock files: `Cargo.lock`, `package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`, `*.lock`, `*.lockb`
- Binary files are automatically simplified to `"Binary file {path} changed"`

**Why exclude files?**
- Reduces token usage and costs
- Prevents API errors from exceeding context limits
- Improves description quality by focusing on meaningful changes

**Large diff warning:**
If your diff exceeds 50KB after filtering, you'll see a warning:
```
⚠ Warning: Diff is large (75000 bytes, 3500 lines)
  Consider splitting into smaller commits.
```

### Command-line options

```
Usage: jj-desc [OPTIONS]

Options:
      --dry-run                    Preview the generated descriptions without applying them
      --provider <PROVIDER>        LLM provider to use (openrouter, openai, anthropic, gemini)
      --model <MODEL>              Override the LLM model to use
      --max-tokens <MAX_TOKENS>    Maximum tokens for LLM response
      --temperature <TEMPERATURE>  Temperature for LLM response (0.0-2.0)
  -r, --revisions <REVISIONS>      Revset to select target commits [default: "::@ & mutable()"]
  -n, --limit <LIMIT>              Maximum number of commits to process
  -i, --interactive                Ask for confirmation before applying each description
  -x, --exclude <EXCLUDE>          Files to exclude from diff (can be specified multiple times)
  -v, --verbose                    Enable verbose logging
  -h, --help                       Print help
  -V, --version                    Print version
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
# feat: add hello function
```

### Example 2: Preview before applying

```bash
jj-desc --dry-run

# Output:
# Generated description (not applied):
# ─────────────────────
# feat(auth): add JWT authentication
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
#   feat(auth): add authentication endpoint
# ✓ Description applied
#
# Processing: 2/3 (66%)
# Commit: def456ghi789
# Generated description:
#   fix(auth): fix validation bug in login form
# ✓ Description applied
#
# Processing: 3/3 (100%)
# Commit: ghi789jkl012
# Generated description:
#   chore(deps): update dependencies
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
jj-desc -i -r "mine()"

# For each commit, you'll see:
# Processing: 1/5 (20%)
# Commit: abc123
# Generated description:
#   feat(auth): add user authentication
#
# Full description:
# ─────────────────────
# feat(auth): add JWT authentication
#
# Implements login and logout endpoints with secure
# token generation and validation.
# ─────────────────────
# Accept (a) / Skip (s) / Quit (q): a
# ✓ Description applied
```

## How it works

1. Runs `jj diff` to get the current changes
2. **Filters the diff** to optimize for LLM processing:
   - Automatically excludes lock files (`Cargo.lock`, `package-lock.json`, etc.)
   - Simplifies binary files to `"Binary file {path} changed"`
   - Applies user-specified exclusions via `--exclude`
   - Warns if diff exceeds 50KB
3. If the filtered diff is empty:
   - Checks if it's a merge commit (using `jj log -T 'parents.len()'`)
   - If yes, sets description to "Merge commit" without calling LLM
   - If no, returns an error
4. If the diff is not empty, sends it to your chosen LLM provider API with a specialized prompt
5. Applies the generated description using `jj desc -m`

### Merge Commit Handling

jj often marks merge commits as "empty" because they don't introduce new changes themselves (see [jj FAQ](https://docs.jj-vcs.dev/latest/FAQ/#why-are-most-merge-commits-marked-as-empty)). `jj-desc` detects merge commits automatically and sets an appropriate description without requiring LLM API calls.

## Tips: Automate with jj push alias

You can integrate `jj-desc` into your push workflow by adding a jj alias. This runs `jj-desc` automatically before every push:

```bash
# Edit your jj config
jj config edit --user
```

Add the following alias:

```toml
[aliases]
push = ["util", "exec", "--", "bash", "-c", """
set -e
# Generate descriptions for commits without them (if jj-desc is available)
command -v jj-desc &> /dev/null && jj-desc
# Run pre-commit checks if config exists
[ ! -f .pre-commit-config.yaml ] || pre-commit run --all-files
# Push
jj git push \"$@\"
""", ""]
```

Now `jj push` will:
1. Auto-generate descriptions for commits without them
2. Run pre-commit checks (if configured)
3. Push to remote

To bypass, use `jj git push` directly.

## Why no confirmation prompt?

Unlike git, jj makes it extremely easy to undo any operation:

- `jj undo` - Undo the last operation
- `jj op log` - View operation history
- All changes are recoverable

This design philosophy means we can safely apply descriptions immediately, making the workflow faster and more streamlined.

## License

MIT

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, coding guidelines, and how to submit changes.

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
- [jj-desc についてのブログ記事 (日本語)](https://blog.tumf.dev/posts/diary/2026/1/8/jj-desc-release/?utm_source=github&utm_medium=readme&utm_campaign=jj-desc)
