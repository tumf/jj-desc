# Implementation Tasks

## タスク一覧

### 1. `src/cli.rs` のユニットテスト追加

- [x] **1.1** `tests` モジュールを作成
- [x] **1.2** サブコマンドパーステスト
  - `generate` サブコマンド（デフォルト）
  - `backfill` サブコマンド
- [x] **1.3** グローバルオプションのテスト
  - `--provider` / `-p`
  - `--model` / `-m`
  - `--verbose` / `-v`
- [x] **1.4** サブコマンド固有のオプションテスト
  - `generate --revision`
  - `backfill --revisions`, `--dry-run`, `--interactive`
- [x] **1.5** バリデーションテスト（不正な引数の拒否）

**検証:** `cargo test cli::tests` ✅ **12テストが成功**

---

### 2. `src/llm/mod.rs` のユニットテスト追加

- [x] **2.1** `tests` モジュールを作成
- [x] **2.2** `create_client` のプロバイダー振り分けテスト
  - Anthropic プロバイダー
  - OpenAI プロバイダー
  - OpenRouter プロバイダー
  - Gemini プロバイダー
- [x] **2.3** 各クライアントの初期化が成功することを確認

**検証:** `cargo test llm::tests` ✅ **4テストが成功**

---

### 3. `src/llm/anthropic.rs` のユニットテスト追加

- [x] **3.1** `tests` モジュールを作成
- [x] **3.2** `AnthropicClient::new` の初期化テスト
  - デフォルトモデルの設定
  - カスタムモデルの設定
  - カスタムbase_urlの設定
- [x] **3.3** `AnthropicRequest` のJSON構造テスト
  - `model` フィールド
  - `messages` フィールド（system, user）
  - `max_tokens` フィールド
- [x] **3.4** `ANTHROPIC_VERSION` 定数のテスト

**検証:** `cargo test llm::anthropic::tests` ✅ **5テストが成功**

---

### 4. `src/llm/openai_compat.rs` のユニットテスト追加

- [x] **4.1** `tests` モジュールを作成
- [x] **4.2** `OpenAICompatClient::new` の初期化テスト
  - OpenAI, OpenRouter, Gemini 各プロバイダー
  - カスタムモデルの設定
  - カスタムbase_urlの設定
- [x] **4.3** `ChatCompletionRequest` のJSON構造テスト
  - `model` フィールド
  - `messages` フィールド（system, user）

**検証:** `cargo test llm::openai_compat::tests` ✅ **6テストが成功**

---

### 5. カバレッジ検証

- [x] **5.1** `cargo llvm-cov` で全体カバレッジを確認
- [x] **5.2** カバレッジが60%以上であることを確認
- [x] **5.3** 各モジュールのカバレッジレポートを確認

**検証:** `cargo llvm-cov --all-features -- --test-threads=1` ✅ **67.75% (目標60%を+7.75%上回る)**

---

### 6. ドキュメント更新

- [x] **6.1** `README.md` にテストコマンドを追加（既存の場合はスキップ）
- [x] **6.2** 開発ガイドにカバレッジ測定方法を記載（`AGENTS.md` 内）

**検証:** ドキュメントの目視確認 ✅ **AGENTS.mdに既に記載済み**

---

## 依存関係

- タスク 2-4 は並行実行可能
- タスク 5 はすべてのテスト追加後に実行
- タスク 6 は最後に実行

## 成功基準

✅ すべてのテストがパスする → **46テストがパス（27個の新規テスト追加）**
✅ カバレッジが60%以上に向上 → **67.75%達成（+14.09%向上）**
✅ 新規テストが高速に実行される（< 1秒） → **0.01秒で完了**
✅ CI/CDパイプラインが壊れない → **すべてのテストが成功**

## 実装結果サマリー

- **追加テスト数:** 27個
  - `cli.rs`: 12テスト
  - `llm/mod.rs`: 4テスト
  - `llm/anthropic.rs`: 5テスト
  - `llm/openai_compat.rs`: 6テスト

- **カバレッジ向上:**
  - 全体: 53.66% → **67.75%** (+14.09%)
  - `cli.rs`: 0% → **89.04%**
  - `llm/mod.rs`: 0% → **100%**
  - `llm/anthropic.rs`: 0% → **92.98%**
  - `llm/openai_compat.rs`: 0% → **96.97%**
