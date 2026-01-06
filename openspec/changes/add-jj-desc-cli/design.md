# 設計: jj-desc CLI ツール

## アーキテクチャ概要

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   jj diff   │────▶│  jj-desc    │────▶│  jj desc    │
│  (入力取得)  │     │  (LLM処理)   │     │  (適用)     │
└─────────────┘     └─────────────┘     └─────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ OpenRouter  │
                    │    API      │
                    └─────────────┘
```

## 設計方針

### 確認プロンプト不要の理由

jj (Jujutsu) は Git と異なり、履歴の書き換えが安全かつ容易:

- `jj undo` で直前の操作を取り消し可能
- `jj op log` で操作履歴を確認可能
- 全ての変更は即座にリカバリ可能

そのため、確認プロンプトは不要とし、即座に適用する設計とする。

## モジュール構成

```
src/
├── main.rs          # エントリーポイント
├── cli.rs           # CLI 引数定義 (clap derive)
├── jj.rs            # jj コマンド連携
├── llm.rs           # OpenRouter API クライアント
├── prompt.rs        # LLM プロンプト生成
├── config.rs        # 設定・環境変数管理
└── error.rs         # エラー型定義 (thiserror)
```

## 技術選定（2025年最新スタック）

### Rust Edition

- **Edition 2024** を採用
- 最新の言語機能を活用

### 依存クレート

| クレート | バージョン | 用途 | 選定理由 |
|---------|-----------|------|---------|
| `clap` | 4.x | CLI 引数解析 | derive マクロによる宣言的定義、Rust 標準的な選択 |
| `tokio` | 1.x | 非同期ランタイム | デファクトスタンダード、full features |
| `reqwest` | 0.12+ | HTTP クライアント | rustls-tls backend（OpenSSL 非依存）、非同期対応 |
| `serde` | 1.x | シリアライズ | デファクトスタンダード |
| `serde_json` | 1.x | JSON 処理 | デファクトスタンダード |
| `thiserror` | 2.x | エラー定義 | ライブラリ/アプリ両方で使える型安全なエラー |
| `anyhow` | 1.x | エラーハンドリング | `.context()` による豊富なエラーメッセージ |
| `tracing` | 0.1 | 構造化ログ | 最新のロギング標準 |
| `tracing-subscriber` | 0.3 | ログ出力 | `RUST_LOG` 環境変数対応 |

### 標準ライブラリ活用

以下の機能は外部クレートではなく標準ライブラリを使用:

| 機能 | 標準ライブラリ | 旧クレート |
|------|--------------|-----------|
| 遅延初期化 | `std::sync::LazyLock` | ~~lazy_static~~ |
| 一回限り初期化 | `std::sync::OnceLock` | ~~once_cell~~ |
| 非同期トレイト | ネイティブ `async fn` in traits | ~~async_trait~~ |

### reqwest 設定方針

```rust
// rustls-tls を使用（OpenSSL 非依存）
// HTTP/1.1 のみ（HTTP/2 はセッションハング問題があるため）
reqwest::ClientBuilder::new()
    .use_rustls_tls()
    .http1_only()
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(5))
    .user_agent(concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
    ))
    .build()
```

## Cargo.toml 依存関係

```toml
[package]
name = "jj-desc"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
description = "Generate jj commit descriptions using LLM"
license = "MIT"
repository = "https://github.com/tumf/jj-desc"

[dependencies]
# CLI
clap = { version = "4", features = ["derive", "env"] }

# Async runtime
tokio = { version = "1", features = ["rt-multi-thread", "macros", "process"] }

# HTTP client (rustls for no OpenSSL dependency)
reqwest = { version = "0.12", default-features = false, features = [
    "rustls-tls",
    "json",
] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error handling
thiserror = "2"
anyhow = "1"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
rstest = "0.23"

[profile.release]
lto = true
codegen-units = 1
strip = true
```

## OpenRouter API 連携

OpenRouter は OpenAI 互換の API を提供するため、標準的な Chat Completions API 形式を使用する。

```
POST https://openrouter.ai/api/v1/chat/completions
```

リクエストヘッダー:
- `Authorization: Bearer {OPENROUTER_API_KEY}`
- `Content-Type: application/json`
- `HTTP-Referer: https://github.com/tumf/jj-desc` (optional)
- `X-Title: jj-desc` (optional)

## 設計判断

### 1. エラーハンドリング戦略

`thiserror` でドメイン固有のエラー型を定義し、`anyhow` で文脈情報を追加:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JjDescError {
    #[error("OPENROUTER_API_KEY environment variable is not set")]
    MissingApiKey,
    
    #[error("No changes found in diff")]
    EmptyDiff,
    
    #[error("jj command failed: {0}")]
    JjCommand(String),
    
    #[error("API request failed: {0}")]
    ApiError(#[from] reqwest::Error),
}
```

### 2. プロンプト設計

差分から意味のある説明文を生成するため、以下の構造のプロンプトを使用:

```
You are a helpful assistant that generates concise git commit descriptions.
Analyze the provided diff and generate a clear, meaningful commit message.
```

ユーザープロンプト:
```
Generate a commit message for the following diff:

<diff>
{jj diff の出力}
</diff>

Requirements:
- Use imperative mood (e.g., "Add", "Fix", "Update")
- First line should not exceed 72 characters
- Be concise but descriptive
- Focus on the "why" and "what", not the "how"
```

### 3. 出力フォーマット

即座に適用し、結果を表示:

```
Applied description:
─────────────────────
Add user authentication with JWT tokens
```

`--dry-run` の場合:

```
Generated description (not applied):
─────────────────────
Add user authentication with JWT tokens
```

### 4. ロギング戦略

`tracing` を使用した構造化ログ:

```rust
use tracing::{info, debug, instrument};

#[instrument(skip(diff))]
async fn generate_description(diff: &str) -> Result<String> {
    debug!(diff_len = diff.len(), "Sending diff to LLM");
    // ...
    info!("Description generated successfully");
}
```

環境変数 `RUST_LOG` で制御:
- `RUST_LOG=debug jj-desc` で詳細ログ表示
- `--verbose` フラグでも debug レベルを有効化

### 5. 終了コード

| コード | 意味 |
|--------|------|
| 0 | 成功 |
| 1 | エラー（設定不備、API失敗など） |

## 将来の拡張性

以下の機能は初期実装には含めないが、拡張可能な設計とする:

- 設定ファイル対応 (`~/.config/jj-desc/config.toml`)
- 複数のプロンプトテンプレート
- カスタムプロンプトの指定
- 生成履歴の保存
- ストリーミングレスポンス対応
