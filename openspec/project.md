# jj-desc

A CLI tool written in Rust that generates commit descriptions for [jj (Jujutsu)](https://github.com/martinvonz/jj) using LLM.

## Overview

`jj-desc` analyzes the output of `jj diff` and generates meaningful commit descriptions using LLM providers. It then applies the description via `jj desc -m '{desc}'`.

## Goals

- Provide a simple CLI to automate commit message generation
- Integrate with multiple LLM providers for text generation
- Support the jj version control workflow seamlessly

## Technology Stack

- **Language**: Rust (2024 Edition, MSRV 1.85+)
- **LLM Providers**: OpenRouter, OpenAI, Anthropic, Gemini
- **Version Control**: jj (Jujutsu)
- **Async Runtime**: Tokio
- **HTTP Client**: reqwest (rustls-tls)

## Key Features

- Multiple LLM provider support with easy switching
- Configurable via environment variables
- Dry-run mode for previewing generated descriptions
- Custom model selection per provider
- Verbose logging with tracing
