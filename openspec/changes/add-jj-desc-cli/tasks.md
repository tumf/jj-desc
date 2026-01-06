# タスク: jj-desc CLI ツールの実装

## 前提条件

- Rust ツールチェーン (rustup) 1.85+ がインストール済み
- jj がインストール済み
- OpenRouter API キーが取得済み

---

## フェーズ 1: プロジェクトセットアップ

### 1.1 Cargo プロジェクト初期化
- [x] `cargo init --name jj-desc` を実行
- [x] Edition 2024 を設定
- [x] 必要な依存クレートを `Cargo.toml` に追加（最新スタック）:
  - `clap` 4.x (derive, env features)
  - `tokio` 1.x (rt-multi-thread, macros, process features)
  - `reqwest` 0.12+ (rustls-tls, json features) ※OpenSSL非依存
  - `serde` 1.x, `serde_json` 1.x
  - `thiserror` 2.x（エラー型定義）
  - `anyhow` 1.x（エラーハンドリング）
  - `tracing` 0.1, `tracing-subscriber` 0.3（構造化ログ）
- [x] リリースプロファイル最適化設定（LTO, strip）
- [x] 検証: `cargo build` が成功すること

### 1.2 モジュール構造作成
- [x] `src/cli.rs` を作成（CLI 引数定義）
- [x] `src/config.rs` を作成（設定管理）
- [x] `src/error.rs` を作成（エラー型定義）
- [x] `src/jj.rs` を作成（jj コマンド連携）
- [x] `src/llm.rs` を作成（OpenRouter API）
- [x] `src/prompt.rs` を作成（プロンプト生成）
- [x] `src/main.rs` でモジュールを宣言
- [x] 検証: `cargo check` が成功すること

---

## フェーズ 2: エラー型と設定管理の実装

### 2.1 エラー型定義
- [x] `error.rs` に `thiserror` を使ったエラー enum を定義
  - `MissingApiKey`: API キー未設定
  - `EmptyDiff`: 差分が空
  - `JjCommand`: jj コマンド失敗
  - `ApiError`: API 呼び出し失敗
- [x] 検証: エラー型が適切にコンパイルされること

### 2.2 環境変数の読み込み
- [x] `config.rs` に `Config` 構造体を実装
  - `api_key: String`
  - `model: String`
  - `base_url: String`
- [x] `OPENROUTER_API_KEY` の読み込み（必須）
- [x] `OPENROUTER_MODEL` の読み込み（デフォルト: `anthropic/claude-sonnet-4-5`）
- [x] `OPENROUTER_BASE_URL` の読み込み（デフォルト: `https://openrouter.ai/api/v1`）
- [x] 検証: API キー未設定時に適切なエラーが返ること

---

## フェーズ 3: jj 連携の実装

### 3.1 jj diff の実行
- [x] `jj.rs` に `get_diff(revision: Option<&str>)` 関数を実装
- [x] `tokio::process::Command` で非同期に `jj diff` を実行
- [x] リビジョン指定時は `-r` オプションを追加
- [x] 検証: 実際の jj リポジトリで差分が取得できること

### 3.2 jj desc の実行
- [x] `jj.rs` に `set_description(desc: &str, revision: Option<&str>)` 関数を実装
- [x] 説明文の適切なエスケープ処理
- [x] 検証: 説明文が正しく適用されること

### 3.3 差分の空チェック
- [x] 差分が空の場合を検出するロジックを追加
- [x] 検証: 変更がない場合にエラーが返ること

---

## フェーズ 4: LLM 連携の実装

### 4.1 プロンプト生成
- [x] `prompt.rs` にシステムプロンプトを定義
- [x] `prompt.rs` に `build_user_prompt(diff: &str)` 関数を実装
- [x] 検証: 期待されるプロンプト形式が生成されること

### 4.2 OpenRouter API クライアント
- [x] `llm.rs` に `OpenRouterClient` 構造体を実装
- [x] `reqwest::Client` を rustls-tls + HTTP/1.1 only で構成
- [x] Chat Completions API 形式のリクエスト/レスポンス構造体を定義
- [x] `generate_description(diff: &str)` メソッドを実装
- [x] 適切なヘッダー設定（Authorization, HTTP-Referer, X-Title）
- [x] 検証: 実際の API 呼び出しが成功すること

### 4.3 エラーハンドリングとタイムアウト
- [x] 接続タイムアウト: 5秒
- [x] リクエストタイムアウト: 30秒
- [x] API エラー時の詳細なエラーメッセージ
- [x] 検証: タイムアウト時に分かりやすいメッセージが表示されること

---

## フェーズ 5: CLI インターフェースの実装

### 5.1 引数解析（clap derive）
- [x] `cli.rs` に `Args` 構造体を定義
  - `--dry-run`: プレビューのみ
  - `--model <MODEL>`: モデル指定（環境変数もサポート）
  - `--revision <REV>`, `-r <REV>`: リビジョン指定
  - `--verbose`, `-v`: 詳細ログ出力
- [x] 検証: `--help` で正しいヘルプが表示されること

### 5.2 ロギング設定
- [x] `tracing_subscriber` を初期化
- [x] `RUST_LOG` 環境変数でログレベル制御
- [x] `--verbose` フラグで debug レベルを有効化
- [x] 検証: ログが適切に出力されること

### 5.3 メインフロー統合
- [x] 全モジュールを統合したメインロジックを実装
  1. ロギング初期化
  2. 設定読み込み
  3. jj diff 実行
  4. 差分チェック
  5. LLM 呼び出し
  6. jj desc 実行（`--dry-run` でない場合）
  7. 結果表示
- [x] `anyhow::Context` でエラーに文脈情報を追加
- [x] 検証: エンドツーエンドで動作すること

---

## フェーズ 6: テストとドキュメント

### 6.1 ユニットテスト
- [x] `error.rs` のテスト
- [x] `config.rs` のテスト
- [x] `prompt.rs` のテスト
- [x] `rstest` を使ったパラメトリックテスト
- [x] 検証: `cargo test` が成功すること

### 6.2 README 作成
- [x] インストール手順（cargo install）
- [x] 使用方法
- [x] 環境変数の説明
- [x] 使用例
- [x] ライセンス情報

### 6.3 最終検証
- [x] クリーンビルド確認: `cargo build --release`
- [x] バイナリサイズ確認（strip 適用）
- [x] 実際の jj リポジトリでの動作確認
- [x] エラーケースの動作確認

---

## 依存関係

```
1.1 ─▶ 1.2 ─▶ 2.1 ─▶ 2.2 ─┬─▶ 3.1 ─▶ 3.2 ─▶ 3.3
                          │
                          └─▶ 4.1 ─▶ 4.2 ─▶ 4.3
                                     │
5.1 ─▶ 5.2 ◀───────────────────────────┘
  │
  ▼
5.3 ─▶ 6.1 ─▶ 6.2 ─▶ 6.3
```

フェーズ 3 と 4 は並行して作業可能。
