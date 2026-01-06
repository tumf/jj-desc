# 設計: refactor-error-types

## 設計判断

### エラーバリアントの分類

現在の `JjDescError` を以下のように再分類：

| カテゴリ | バリアント | 用途 |
|----------|------------|------|
| 設定エラー | `MissingApiKey` | API キー未設定 |
| 設定エラー | `InvalidProvider` | Provider 名が不正 |
| jj エラー | `JjCommand` | jj コマンド実行失敗 |
| jj エラー | `EmptyDiff` | diff が空（非マージ） |
| API エラー | `ApiError` | ネットワーク/接続エラー |
| API エラー | `ApiStatus` | HTTP ステータスエラー（4xx, 5xx）|
| API エラー | `ApiResponseError` | レスポンスパース/形式エラー |
| その他 | `InvalidUtf8` | 文字コードエラー |
| その他 | `Io` | I/O エラー |
| その他 | `JsonError` | JSON パースエラー |

### Provider パース戦略

```rust
// 現在の実装（問題あり）
let provider = env::var("LLM_PROVIDER")
    .ok()
    .and_then(|s| s.parse().ok())  // パースエラーを握りつぶし
    .unwrap_or(Provider::OpenRouter);

// 修正後
let provider = match env::var("LLM_PROVIDER") {
    Ok(s) => s.parse()?,  // パースエラーは伝播
    Err(env::VarError::NotPresent) => Provider::OpenRouter,  // 未設定時のみデフォルト
    Err(env::VarError::NotUnicode(_)) => {
        return Err(JjDescError::InvalidProvider("non-UTF8 value".into()))
    }
};
```

### API エラーの使い分け

```rust
// ネットワークエラー（reqwest::Error）
// → ApiError（#[from] で自動変換）

// HTTP ステータスエラー（4xx, 5xx）
if !response.status().is_success() {
    return Err(JjDescError::ApiStatus {
        status: response.status().as_u16(),
        body: response.text().await.unwrap_or_default(),
    });
}

// レスポンス形式エラー（choices が空など）
.ok_or_else(|| JjDescError::ApiResponseError(
    "No choices in API response".to_string()
))?
```

## 後方互換性

- CLI の終了コードには影響なし
- エラーメッセージが改善されるのみ
- 破壊的変更なし
