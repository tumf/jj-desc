# jj-desc

A CLI tool written in Rust that generates commit descriptions for [jj (Jujutsu)](https://github.com/martinvonz/jj) using LLM.

## Overview

`jj-desc` analyzes the output of `jj diff` and generates meaningful commit descriptions using an LLM service (OpenRouter). It then applies the description via `jj desc -m '{desc}'`.

## Goals

- Provide a simple CLI to automate commit message generation
- Integrate with OpenRouter for LLM-based text generation
- Support the jj version control workflow seamlessly

## Technology Stack

- **Language**: Rust
- **LLM Provider**: OpenRouter API
- **Version Control**: jj (Jujutsu)
