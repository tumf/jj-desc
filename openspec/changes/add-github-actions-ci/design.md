# 設計: GitHub Actions CI ワークフロー

## 設計方針

### ワークフローの構成

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
```

### ジョブ構成

1. **test**: ユニットテストの実行
2. **lint**: Clippy によるコード品質チェック
3. **format**: rustfmt によるフォーマットチェック

### 並列実行 vs 直列実行

- **採用**: 並列実行
- **理由**: 各ジョブは独立しており、並列実行により CI 時間を短縮できる

### Rust バージョン戦略

| バージョン | 目的 |
|-----------|------|
| 1.85 (MSRV) | 最小サポートバージョンでの互換性確認 |
| stable | 最新安定版での動作確認 |

### キャッシュ戦略

- `Swatinem/rust-cache@v2` を使用
- `~/.cargo` および `target/` ディレクトリをキャッシュ
- ビルド時間を大幅に短縮（初回: 2-3分 → キャッシュ時: 30秒程度）

## 代替案

### 代替案1: 単一ジョブですべてを実行

```yaml
jobs:
  ci:
    steps:
      - cargo fmt --check
      - cargo clippy
      - cargo test
```

**不採用理由**: 最初のステップで失敗すると後続のフィードバックが得られない

### 代替案2: Matrix で複数 OS をテスト

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
```

**不採用理由**: 現時点では過剰。将来的にクロスプラットフォームサポートが必要になれば追加

## 成果物

```
.github/
└── workflows/
    └── ci.yml
```
