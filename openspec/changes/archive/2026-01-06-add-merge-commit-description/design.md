# 設計: マージコミット検出とdescription設定

## アーキテクチャ

### マージコミット検出の仕組み

jjでは`parents`テンプレートキーワードを使用してコミットの親を取得できる。

```bash
# 親の数を取得
jj log -T 'parents.len()' -r @ --no-graph
```

- 通常のコミット: 親が1つ → `1`
- マージコミット: 親が2つ以上 → `2`以上
- ルートコミット: 親が0 → `0`

### 処理フロー

```
┌─────────────────┐
│   jj diff実行    │
└────────┬────────┘
         │
    ┌────▼────┐
    │差分あり？ │
    └────┬────┘
         │
    No ──┴── Yes
    │         │
┌───▼───┐ ┌───▼───┐
│マージ？ │ │LLM生成 │
└───┬───┘ └───┬───┘
    │         │
Yes─┴─No      │
│     │       │
▼     ▼       │
マージ エラー   │
説明    │      │
│       │      │
└───────┴──────┘
         │
    ┌────▼────┐
    │jj desc設定│
    └─────────┘
```

### コード変更

#### `src/jj.rs` への追加

```rust
/// マージコミットかどうかを判定する
pub async fn is_merge_commit(revision: Option<&str>) -> Result<bool, JjDescError> {
    let mut cmd = Command::new("jj");
    cmd.args(["log", "-T", "parents.len()", "--no-graph"]);

    if let Some(rev) = revision {
        cmd.arg("-r").arg(rev);
    } else {
        cmd.arg("-r").arg("@");
    }

    let output = cmd.output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(JjDescError::JjCommand(stderr.to_string()));
    }

    let count_str = String::from_utf8(output.stdout)?;
    let parent_count: usize = count_str.trim().parse().unwrap_or(0);

    Ok(parent_count >= 2)
}
```

#### `src/main.rs` の変更

```rust
// 既存のdiff取得でEmptyDiffエラーが発生した場合
match jj::get_diff(revision).await {
    Ok(diff) => {
        // LLMでdescription生成
    }
    Err(JjDescError::EmptyDiff) => {
        // マージコミットか確認
        if jj::is_merge_commit(revision).await? {
            // マージコミット用のdescriptionを設定
            jj::set_description("Merge commit", revision).await?;
        } else {
            return Err(JjDescError::EmptyDiff);
        }
    }
    Err(e) => return Err(e),
}
```

## 代替案

### 案A: LLMにマージ情報を渡す
マージされたブランチ情報（親のdescription等）をLLMに渡して、より詳細なマージメッセージを生成する。

**メリット**: より意味のあるマージメッセージが生成できる
**デメリット**: 実装が複雑、追加のjjコマンド実行が必要

### 案B: 固定メッセージ（採用）
シンプルに「Merge commit」という固定メッセージを設定する。

**メリット**: 実装がシンプル
**デメリット**: マージの詳細がわからない

### 案C: ユーザー設定可能なテンプレート
マージコミット用のテンプレートをユーザーが設定できるようにする。

**メリット**: 柔軟性が高い
**デメリット**: 設定項目が増える、初期実装としては過剰

## 採用方針
まずは案Bのシンプルな実装を行い、必要に応じて案A/Cを追加実装する。
