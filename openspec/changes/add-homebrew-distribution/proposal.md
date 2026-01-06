# 変更提案: Homebrew 配布サポート

## 概要

`brew install jj-desc` コマンドで jj-desc をインストールできるようにする。

## 背景

現在 jj-desc は `cargo install` または GitHub Releases からのバイナリダウンロードでのみインストール可能である。macOS/Linux ユーザーにとって Homebrew は最も一般的なパッケージマネージャーであり、Homebrew 経由でのインストールをサポートすることでユーザビリティが大幅に向上する。

## 現状分析

### 既存設定

`dist-workspace.toml` には既に cargo-dist による Homebrew サポートが設定されている：

```toml
installers = ["shell", "powershell", "homebrew"]
tap = "tumf/homebrew-tap"
publish-jobs = ["homebrew"]
```

`.github/workflows/release.yml` には `publish-homebrew-formula` ジョブが含まれており、以下を実行する：
- `tumf/homebrew-tap` リポジトリにチェックアウト
- Formula ファイルをコミット・プッシュ

### 不足している設定

1. **`tumf/homebrew-tap` リポジトリが存在しない**
2. **`HOMEBREW_TAP_TOKEN` シークレットが未設定**（リポジトリ設定で確認必要）

## 提案内容

### 1. homebrew-tap リポジトリの作成

GitHub に `tumf/homebrew-tap` リポジトリを作成する。このリポジトリは Homebrew の「tap」として機能し、Formula ファイルを格納する。

### 2. GitHub シークレットの設定

`HOMEBREW_TAP_TOKEN` を設定する。このトークンには `tumf/homebrew-tap` リポジトリへの書き込み権限が必要。

### 3. README の更新

インストール手順に Homebrew を追加する。

## 影響範囲

- 新規リポジトリ: `tumf/homebrew-tap`
- 設定変更: GitHub Secrets
- ドキュメント更新: README.md

## 関連仕様

- 新規: `homebrew-distribution` 仕様
