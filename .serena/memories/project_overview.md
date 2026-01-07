# jj-desc Project Overview

## Purpose
jj-desc is a CLI tool that generates commit descriptions for [jj (Jujutsu)](https://github.com/martinvonz/jj) version control system using LLM providers.

## Key Features
- Auto-generates meaningful commit descriptions from diffs using LLMs
- Batch processing with revset targeting
- Multiple LLM providers: OpenRouter, OpenAI, Anthropic, Gemini
- Custom endpoint support (Azure OpenAI, Ollama, LM Studio)
- Preview mode (`--dry-run`) and interactive mode (`-i`)
- Follows Conventional Commits format
- Handles merge commits automatically
- Excludes lock files and binary files from diff

## Tech Stack
- **Language**: Rust 2024 Edition
- **MSRV**: 1.85+
- **License**: MIT

## Key Dependencies
| Crate | Purpose |
|-------|---------|
| clap | CLI parsing (derive macros) |
| tokio | Async runtime |
| reqwest | HTTP client (rustls-tls) |
| serde | JSON serialization |
| thiserror/anyhow | Error handling |
| tracing | Structured logging |
| rstest | Test framework (dev) |

## Project Structure
```
src/
├── main.rs       # Entry point, CLI orchestration
├── cli.rs        # Command-line args (clap)
├── config.rs     # Configuration, env vars
├── error.rs      # Custom errors (thiserror)
├── jj.rs         # jj VCS interaction
├── diff_filter.rs # Diff filtering logic
├── prompt.rs     # Prompt templates
├── provider.rs   # Provider enum/traits
├── commands/     # Command implementations
│   └── mod.rs
└── llm/          # LLM client implementations
    ├── mod.rs
    ├── anthropic.rs
    └── openai_compat.rs
```

## Environment Variables
- `LLM_PROVIDER`: LLM provider (openrouter, openai, anthropic, gemini)
- `LLM_MODEL`: Override model
- Provider-specific API keys: `OPENROUTER_API_KEY`, `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GEMINI_API_KEY`
- Provider-specific base URLs: `OPENROUTER_BASE_URL`, `OPENAI_BASE_URL`, `ANTHROPIC_BASE_URL`, `GEMINI_BASE_URL`
