# 設計: refactor-diff-result

## 設計判断

### 方針の選択

以下の3つの方針を検討した結果、**方針A** を採用：

| 方針 | 概要 | 評価 |
|------|------|------|
| A: enum 返却 | `get_diff()` が `DiffResult` enum を返す | ✅ 採用 |
| B: 判定を呼び出し側へ | `get_diff()` は純粋に diff を返し、マージ判定は呼び出し側 | △ 重複が残る |
| C: 共通関数化 | `generate_description_for_revision()` に統合 | △ 変更範囲が大きい |

### 方針A を選んだ理由

1. **型安全性**: コンパイラが全ケースの処理を強制する
2. **最小限の変更**: 既存の関数シグネチャ変更のみで済む
3. **明確な責務分離**: diff 取得の責務が `jj.rs` に集約される

## データ構造

```rust
/// jj diff の結果を表す enum
#[derive(Debug, Clone)]
pub enum DiffResult {
    /// 通常の diff 内容（空でない）
    Content(String),
    /// マージコミットで diff が空の場合
    EmptyMerge,
}
```

### 検討したが採用しなかった案

```rust
// 案1: より詳細な情報を持つ構造体
pub struct DiffResult {
    pub content: Option<String>,
    pub is_merge: bool,
    pub parent_count: usize,
}
// → 過剰な情報。現時点では不要。

// 案2: 3バリアント enum
pub enum DiffResult {
    Content(String),
    EmptyMerge,
    EmptyRegular,  // 空 diff の非マージ
}
// → EmptyRegular はエラーとして扱うべきなので Err(EmptyDiff) で十分
```

## 呼び出し側のパターン

```rust
// generate.rs / backfill.rs での使用例
match jj::get_diff(revision).await? {
    DiffResult::Content(diff) => {
        // LLM で説明を生成
        client.generate_description(&diff).await?
    }
    DiffResult::EmptyMerge => {
        // 固定メッセージを使用
        "Merge commit".to_string()
    }
}
```

## 後方互換性

- 外部 API なし（CLI ツール）
- 内部関数のシグネチャ変更のみ
- 動作結果は同等（バグ修正により改善）
