# 設計: Homebrew 配布サポート

## アーキテクチャ概要

```
┌─────────────────────────────────────────────────────────────┐
│                    Release Workflow                          │
│  (.github/workflows/release.yml)                            │
├─────────────────────────────────────────────────────────────┤
│  1. plan          - dist plan でビルド計画を生成            │
│  2. build-local   - 各プラットフォーム用バイナリをビルド    │
│  3. build-global  - インストーラー・チェックサムを生成      │
│  4. host          - GitHub Release を作成                   │
│  5. publish-homebrew-formula - Formula を tap にプッシュ    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  tumf/homebrew-tap                          │
├─────────────────────────────────────────────────────────────┤
│  Formula/                                                   │
│  └── jj-desc.rb   ← cargo-dist が自動生成                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  User Installation                          │
├─────────────────────────────────────────────────────────────┤
│  $ brew tap tumf/tap                                        │
│  $ brew install jj-desc                                     │
│  # または                                                    │
│  $ brew install tumf/tap/jj-desc                            │
└─────────────────────────────────────────────────────────────┘
```

## コンポーネント

### 1. cargo-dist 設定 (既存)

`dist-workspace.toml`:
```toml
installers = ["shell", "powershell", "homebrew"]
tap = "tumf/homebrew-tap"
publish-jobs = ["homebrew"]
```

cargo-dist は以下を自動生成する:
- Formula ファイル (`jj-desc.rb`)
- ダウンロード URL とチェックサム

### 2. homebrew-tap リポジトリ (新規)

Homebrew の tap は特定の命名規則に従う必要がある:
- リポジトリ名: `homebrew-tap` (任意の名前可、ただし `homebrew-` プレフィックスが慣例)
- 構造:
  ```
  homebrew-tap/
  └── Formula/
      └── jj-desc.rb
  ```

tap 名: `tumf/tap` (`homebrew-` プレフィックスは省略される)

### 3. GitHub Secrets

`HOMEBREW_TAP_TOKEN`:
- 用途: `publish-homebrew-formula` ジョブが tap リポジトリにプッシュする際に使用
- 権限: `tumf/homebrew-tap` への write access
- トークンタイプ: Personal Access Token (classic または fine-grained)

## Formula ファイル構造

cargo-dist が生成する Formula の例:

```ruby
class JjDesc < Formula
  desc "Generate jj commit descriptions using LLM"
  homepage "https://github.com/tumf/jj-desc"
  version "0.1.0"
  
  on_macos do
    on_arm do
      url "https://github.com/tumf/jj-desc/releases/download/v0.1.0/jj-desc-aarch64-apple-darwin.tar.xz"
      sha256 "..."
    end
    on_intel do
      url "https://github.com/tumf/jj-desc/releases/download/v0.1.0/jj-desc-x86_64-apple-darwin.tar.xz"
      sha256 "..."
    end
  end
  
  on_linux do
    on_arm do
      url "https://github.com/tumf/jj-desc/releases/download/v0.1.0/jj-desc-aarch64-unknown-linux-gnu.tar.xz"
      sha256 "..."
    end
    on_intel do
      url "https://github.com/tumf/jj-desc/releases/download/v0.1.0/jj-desc-x86_64-unknown-linux-gnu.tar.xz"
      sha256 "..."
    end
  end

  def install
    bin.install "jj-desc"
  end
end
```

## セキュリティ考慮事項

### トークンの権限最小化

Fine-grained PAT を使用する場合:
- Repository access: `tumf/homebrew-tap` のみ
- Permissions:
  - Contents: Read and write
  - Metadata: Read

### トークンの有効期限

- 推奨: 90日または無期限
- 無期限の場合、定期的なローテーションを検討

## 代替案

### 1. homebrew-core への PR

メリット:
- `brew install jj-desc` で直接インストール可能（tap 不要）
- 信頼性が高い

デメリット:
- 審査プロセスが必要
- 更新に時間がかかる
- 一定の人気/ダウンロード数が必要な場合がある

**結論**: 初期は独自 tap を使用し、将来的に人気が出れば homebrew-core への移行を検討

### 2. GitHub Actions から直接 Formula を commit

現状の cargo-dist アプローチと同等だが、手動で Formula を管理する方法。

デメリット:
- 手動でのメンテナンスが必要
- バージョン更新時に手動でチェックサムを更新

**結論**: cargo-dist の自動化を活用する現行アプローチが優れている
