# 提案: refactor-config-resolution

## 概要

`Config` の解決ロジックを整理し、設定値の優先順位と LLM リクエストパラメータを明確化する。

## 背景・動機

現在の実装には以下の問題がある：

1. **暗黙的なモデル置換**: `with_provider()` でモデルがデフォルト値と一致する場合、新 provider のデフォルトに自動置換される（`config.rs:76-83`）
   - 問題例: ユーザーが意図的に `gpt-4o` を指定していても、別 provider のデフォルトと一致すると勝手に置換される

2. **API キーの不整合**: provider を CLI で切り替えた際、新 provider の API キーが無いと古い provider のキーが残る（`config.rs:73-75`）

3. **LLM パラメータ不足**: `max_tokens` / `temperature` が設定できず、プロバイダによっては出力が不安定

## 提案内容

### 1. 設定値のソース追跡

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSource {
    Default,
    Environment,
    CommandLine,
}

pub struct Config {
    pub provider: Provider,
    pub api_key: String,
    pub model: String,
    pub model_source: ConfigSource,  // 追加
    pub base_url: String,
    pub max_tokens: Option<u32>,     // 追加
    pub temperature: Option<f32>,    // 追加
}
```

### 2. with_provider() の挙動修正

- `model_source` が `Default` の場合のみ、新 provider のデフォルトモデルに置換
- API キーは新 provider 用を必須とし、無ければエラー

### 3. LLM リクエストパラメータ追加

- `--max-tokens` / `LLM_MAX_TOKENS` オプション追加
- `--temperature` / `LLM_TEMPERATURE` オプション追加
- OpenAI 互換 / Anthropic 両方で使用

## 期待される効果

- 設定の優先順位が明確になり、予期せぬ動作が減る
- LLM 出力の安定性が向上
- デバッグ時に「どこから来た値か」が分かる

## 影響範囲

- `src/config.rs`: `Config` 構造体拡張、解決ロジック修正
- `src/cli.rs`: 新オプション追加
- `src/llm/openai_compat.rs`: `max_tokens`, `temperature` 対応
- `src/llm/anthropic.rs`: `temperature` 対応

## 優先度

**中** — ユーザー体験の改善だが、現状でも動作はする
