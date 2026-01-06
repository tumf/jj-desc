# 変更提案: マルチLLMプロバイダーサポート

## 概要

現在 OpenRouter API のみをサポートしている jj-desc に、OpenAI、Anthropic、Gemini の直接 API サポートを追加する。

## 背景

- 現状は OpenRouter 経由でのみ LLM を利用可能
- ユーザーによっては OpenRouter を使わず、各プロバイダーの API を直接利用したい場合がある
- 直接 API を使用することで、レイテンシの削減やコスト最適化が可能になる
- 企業環境では Azure OpenAI やプロキシ経由でのアクセスが必要な場合がある

## 提案内容

### 1. プロバイダー抽象化

LLM プロバイダーを抽象化し、以下のプロバイダーをサポートする：

| プロバイダー | API キー環境変数 | ベース URL 環境変数 | デフォルト ベース URL |
|-------------|-----------------|-------------------|---------------------|
| OpenRouter (デフォルト) | `OPENROUTER_API_KEY` | `OPENROUTER_BASE_URL` | `https://openrouter.ai/api/v1` |
| OpenAI | `OPENAI_API_KEY` | `OPENAI_BASE_URL` | `https://api.openai.com/v1` |
| Anthropic | `ANTHROPIC_API_KEY` | `ANTHROPIC_BASE_URL` | `https://api.anthropic.com` |
| Gemini | `GEMINI_API_KEY` | `GEMINI_BASE_URL` | `https://generativelanguage.googleapis.com/v1beta/openai` |

### 2. プロバイダー選択

- 環境変数 `LLM_PROVIDER` でプロバイダーを選択
- 有効な値: `openrouter`, `openai`, `anthropic`, `gemini`
- デフォルト: `openrouter`

### 3. モデル名の扱い

- 各プロバイダーには適切なデフォルトモデルを設定
- `LLM_MODEL` 環境変数または `--model` オプションでオーバーライド可能

### 4. カスタムベース URL

- 各プロバイダーに対してオプショナルなベース URL 環境変数を提供
- Azure OpenAI、セルフホスト環境、プロキシ経由でのアクセスに対応
- 設定されていない場合は各プロバイダーのデフォルト URL を使用

## スコープ

- プロバイダー抽象化レイヤーの実装
- 各プロバイダー用のクライアント実装
- カスタムベース URL のサポート
- 設定管理の拡張
- ドキュメントの更新

## スコープ外

- ストリーミングレスポンス対応
- 複数プロバイダーのフォールバック機能
- プロバイダー固有の高度な機能（ツール呼び出し等）

## 成功基準

- 4つのプロバイダーすべてで正常にコミットメッセージを生成できる
- カスタムベース URL を使用して Azure OpenAI 等に接続できる
- 既存の OpenRouter 利用者に影響なく移行できる
- 設定が明確でドキュメント化されている
