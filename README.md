# jj-desc

Generate [jj (Jujutsu)](https://github.com/martinvonz/jj) commit descriptions automatically using LLMs via OpenRouter.

## Features

- 🤖 Automatically generates meaningful commit descriptions from diffs using LLMs
- 🔄 Works seamlessly with jj's undo workflow (no confirmation prompts needed)
- 🎯 Supports multiple LLM models via OpenRouter
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
- OpenRouter API key

## Configuration

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `OPENROUTER_API_KEY` | ✅ | - | Your OpenRouter API key |
| `OPENROUTER_MODEL` | ❌ | `anthropic/claude-sonnet-4` | LLM model to use |
| `OPENROUTER_BASE_URL` | ❌ | `https://openrouter.ai/api/v1` | API base URL |

### Setup

1. Get an API key from [OpenRouter](https://openrouter.ai/)
2. Set the environment variable:

```bash
export OPENROUTER_API_KEY="your-api-key-here"
```

For permanent setup, add it to your shell configuration (`~/.bashrc`, `~/.zshrc`, etc.).

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

### Use a different model

Override the default model:

```bash
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
      --dry-run              Preview the generated description without applying it
      --model <MODEL>        Override the LLM model to use [env: OPENROUTER_MODEL]
  -r, --revision <REVISION>  Target revision (defaults to current working copy)
  -v, --verbose              Enable verbose logging
  -h, --help                 Print help
  -V, --version              Print version
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
2. Sends the diff to OpenRouter API with a specialized prompt
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
├── main.rs     # Entry point and main flow
├── cli.rs      # Command-line argument parsing
├── config.rs   # Configuration management
├── error.rs    # Error type definitions
├── jj.rs       # jj command integration
├── llm.rs      # OpenRouter API client
└── prompt.rs   # LLM prompt generation
```

## Technology Stack

- **Rust Edition 2024** (requires Rust 1.85+)
- **clap** - CLI argument parsing
- **tokio** - Async runtime
- **reqwest** - HTTP client (rustls-tls, no OpenSSL dependency)
- **serde** - JSON serialization
- **thiserror** / **anyhow** - Error handling
- **tracing** - Structured logging

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## See Also

- [jj documentation](https://martinvonz.github.io/jj/)
- [OpenRouter](https://openrouter.ai/)
