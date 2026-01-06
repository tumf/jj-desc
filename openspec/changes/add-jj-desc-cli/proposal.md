# 提案: jj-desc CLI ツールの追加

## 概要

Rust で実装された CLI ツール `jj-desc` を新規作成する。このツールは jj (Jujutsu) の差分から LLM を使用してコミット説明文を自動生成し、適用する。

## 背景と動機

- jj を使用した開発において、適切なコミットメッセージを書くのは時間がかかる作業である
- LLM を活用することで、差分の内容から意味のある説明文を自動生成できる
- OpenRouter を使用することで、様々な LLM モデルを柔軟に選択できる
- jj は履歴の書き換えが容易なため、確認プロンプトは不要（いつでも `jj undo` で戻せる）

## 提案内容

### 基本機能

1. `jj diff` を実行して現在の変更内容を取得
2. 差分を OpenRouter API 経由で LLM に送信し、説明文を生成
3. `jj desc -m '{desc}'` で説明文を即座に適用

### CLI オプション

| オプション | 説明 |
|-----------|------|
| `--dry-run` | 生成結果をプレビューのみ（適用しない） |
| `--model <MODEL>` | 使用する LLM モデルを指定 |
| `--revision <REV>`, `-r <REV>` | 対象のリビジョンを指定 |
| `--verbose`, `-v` | 詳細ログを表示 |

### 環境変数

| 変数名 | 必須 | 説明 |
|--------|------|------|
| `OPENROUTER_API_KEY` | ✓ | OpenRouter API キー |
| `OPENROUTER_MODEL` | - | デフォルトモデル（未設定時: `anthropic/claude-sonnet-4-5`） |
| `OPENROUTER_BASE_URL` | - | API ベース URL（未設定時: `https://openrouter.ai/api/v1`） |

### エラーハンドリング

- `jj diff` が空の場合: エラーメッセージを表示して終了
- API キー未設定: エラーメッセージを表示して終了
- API 呼び出し失敗: エラー内容を表示して終了

## 技術スタック（2025年最新）

### Rust バージョン

- **Edition 2024**（Rust 1.85+）

### 主要依存クレート

| クレート | バージョン | 用途 |
|---------|-----------|------|
| `clap` | 4.x | CLI 引数解析（derive マクロ） |
| `tokio` | 1.x | 非同期ランタイム |
| `reqwest` | 0.12+ | HTTP クライアント（rustls-tls） |
| `serde` / `serde_json` | 1.x | JSON シリアライズ |
| `thiserror` | 2.x | エラー型定義 |
| `anyhow` | 1.x | エラーハンドリング |
| `tracing` | 0.1 | 構造化ログ |
| `rstest` | 0.23 | テスト（dev） |

### 標準ライブラリ活用

- `std::sync::LazyLock`（~~lazy_static~~ 不要）
- `std::sync::OnceLock`（~~once_cell~~ 不要）
- ネイティブ async fn in traits（~~async_trait~~ 不要）

### ビルド最適化

- LTO (Link Time Optimization) 有効
- strip によるバイナリサイズ削減
- rustls-tls による OpenSSL 非依存

## 影響範囲

- 新規プロジェクトの作成（既存コードへの影響なし）

## 関連する仕様変更

- `specs/cli-interface/spec.md` — CLI インターフェース仕様
- `specs/llm-integration/spec.md` — LLM 連携仕様
- `specs/jj-integration/spec.md` — jj 連携仕様
