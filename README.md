# jj-desc

Generate [jj (Jujutsu)](https://github.com/martinvonz/jj) commit descriptions automatically using LLMs.

## Features

- 🤖 Automatically generates meaningful commit descriptions from diffs using LLMs
- 🔄 Works seamlessly with jj's undo workflow (no confirmation prompts needed)
- 🎯 Supports multiple LLM providers: OpenRouter, OpenAI, Anthropic, Gemini
- 🔌 Custom endpoint support (Azure OpenAI, Ollama, proxies, etc.)
- 🔍 Preview mode with `--dry-run`
- 📝 Follows git commit message best practices

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
```

### Preview mode

See what description would be generated without applying it:

```bash
jj-desc --dry-run
```

### Target specific revision

Generate description for a specific revision:

```bash
jj-desc -r @-
```

### Use a different provider

Override the provider:

```bash
jj-desc --provider openai
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
      --dry-run                  Preview the generated description without applying it
      --provider <PROVIDER>      LLM provider to use (openrouter, openai, anthropic, gemini) [env: LLM_PROVIDER]
      --model <MODEL>            Override the LLM model to use [env: LLM_MODEL]
  -r, --revision <REVISION>      Target revision (defaults to current working copy)
  -v, --verbose                  Enable verbose logging
  -h, --help                     Print help
  -V, --version                  Print version
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

## How it works

1. Runs `jj diff` to get the current changes
2. Sends the diff to your chosen LLM provider API with a specialized prompt
3. Applies the generated description using `jj desc -m`

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

### Project structure

```
src/
├── main.rs         # Entry point and main flow
├── cli.rs          # Command-line argument parsing
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
