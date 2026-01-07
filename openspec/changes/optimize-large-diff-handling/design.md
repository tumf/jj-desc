# 設計ドキュメント: 巨大diff最適化

## 設計方針

aicommit2の実装を参考に、最小限の変更で最大の効果を得る。

## アーキテクチャ

```
┌─────────────────────────────────────────────────────────────┐
│                        jj-desc                               │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────┐    ┌──────────────┐    ┌─────────────────┐   │
│  │ CLI Args │───▶│ DiffFilter   │───▶│ LLM Client      │   │
│  │--exclude │    │              │    │                 │   │
│  └──────────┘    │ - Lock files │    │ generate_desc() │   │
│                  │ - Binary     │    └─────────────────┘   │
│                  │ - User excl  │                          │
│                  └──────────────┘                          │
│                         │                                   │
│                         ▼                                   │
│                  ┌──────────────┐                          │
│                  │ Size Warning │                          │
│                  │ (50KB超で警告)│                          │
│                  └──────────────┘                          │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## 主要コンポーネント

### 1. DiffFilter モジュール (`src/diff_filter.rs`)

```rust
/// デフォルト除外パターン
const DEFAULT_EXCLUDES: &[&str] = &[
    "Cargo.lock",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "*.lock",
    "*.lockb",
];

/// diffフィルタリング結果
pub struct FilteredDiff {
    pub content: String,
    pub original_size: usize,
    pub filtered_size: usize,
    pub excluded_files: Vec<String>,
    pub binary_files: Vec<String>,
}

/// diffをフィルタリング
pub fn filter_diff(
    raw_diff: &str,
    exclude_patterns: &[String],
) -> FilteredDiff;
```

### 2. サイズ警告

```rust
const WARNING_THRESHOLD: usize = 50 * 1024; // 50KB

fn warn_if_large(diff: &FilteredDiff) {
    if diff.filtered_size > WARNING_THRESHOLD {
        eprintln!(
            "⚠ Warning: Diff is large ({} bytes, {} lines)",
            diff.filtered_size,
            diff.content.lines().count()
        );
        eprintln!("  Consider splitting into smaller commits.");
    }
}
```

### 3. CLI拡張

```rust
#[derive(Parser)]
struct GenerateArgs {
    // 既存オプション...
    
    /// Files to exclude from diff (can be specified multiple times)
    #[arg(short = 'x', long = "exclude")]
    exclude: Vec<String>,
}
```

## jj diffのフィルタリング方法

`jj diff` の出力を直接フィルタリングする代わりに、`jj diff` コマンド自体にパス指定を使用：

```bash
# 除外パターンを使用
jj diff -r @ -- '!Cargo.lock' '!*.lock'
```

または、取得後に正規表現でフィルタリング：

```rust
// diff --git a/Cargo.lock b/Cargo.lock で始まるセクションを除外
let filtered = raw_diff
    .split("diff --git")
    .filter(|section| !is_excluded(section, patterns))
    .collect::<Vec<_>>()
    .join("diff --git");
```

## バイナリファイル検出

```rust
fn is_binary_section(section: &str) -> bool {
    section.contains("Binary files") || 
    section.contains("GIT binary patch")
}

fn summarize_binary(section: &str) -> String {
    // "diff --git a/image.png b/image.png" からパスを抽出
    let path = extract_path(section);
    format!("Binary file {} changed\n", path)
}
```

## トレードオフ

| 選択肢 | メリット | デメリット |
|--------|----------|------------|
| jj diffのパス指定 | シンプル、正確 | jjのfileset構文学習コスト |
| 取得後フィルタリング | jj非依存 | 一度全て取得するオーバーヘッド |

**決定**: 取得後フィルタリングを採用（jjのfileset構文は複雑で、バージョン依存の可能性あり）

## テスト戦略

1. **ユニットテスト**: `filter_diff()` の各パターンテスト
2. **統合テスト**: 実際のdiff出力でのフィルタリング確認
3. **手動テスト**: 大きなリポジトリでの動作確認

## 将来の拡張性

- 設定ファイル（`.jj-desc.toml`）での除外パターン設定
- `--force` オプションで警告を無視
- `--summary-only` でファイル名リストのみ送信
