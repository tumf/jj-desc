# タスク: Homebrew 配布サポート

## 前提条件

- GitHub アカウントへのアクセス
- リポジトリ管理権限

## タスク一覧

### 1. homebrew-tap リポジトリの作成

- [ ] GitHub で `tumf/homebrew-tap` リポジトリを作成
  - Public リポジトリとして作成
  - README.md を含める（オプション）
  - `Formula/` ディレクトリを作成

**検証**: `gh repo view tumf/homebrew-tap` でリポジトリが存在することを確認

### 2. Personal Access Token (PAT) の作成

- [ ] GitHub Settings > Developer settings > Personal access tokens で新しいトークンを作成
  - Scope: `repo` (Full control of private repositories) または
  - Fine-grained token の場合: `tumf/homebrew-tap` に対する Contents の Read and write 権限
- [ ] トークンを安全に保存

**検証**: トークンが有効であることを確認

### 3. GitHub シークレットの設定

- [ ] `tumf/jj-desc` リポジトリの Settings > Secrets and variables > Actions で `HOMEBREW_TAP_TOKEN` を追加
  - 値: 手順2で作成した PAT

**検証**: シークレットが設定されていることを確認（値は確認不可）

### 4. README.md の更新

- [ ] インストールセクションに Homebrew の手順を追加

```markdown
### Homebrew (macOS/Linux)

```bash
brew tap tumf/tap
brew install jj-desc
```
```

**検証**: README.md に Homebrew インストール手順が含まれていることを確認

### 5. リリースによる検証

- [ ] 新しいバージョンタグをプッシュしてリリースをトリガー
- [ ] `publish-homebrew-formula` ジョブが成功することを確認
- [ ] `tumf/homebrew-tap` に Formula ファイルがコミットされていることを確認
- [ ] `brew tap tumf/tap && brew install jj-desc` が成功することを確認

**検証**: 
- GitHub Actions のリリースワークフローが成功
- `brew install tumf/tap/jj-desc` でインストール可能

## 依存関係

- タスク 2, 3 は並行して実行不可（PAT 作成後にシークレット設定）
- タスク 4 は他のタスクと並行して実行可能
- タスク 5 は 1, 2, 3 の完了後に実行

## 備考

- cargo-dist による自動生成のため、Formula ファイルの手動作成は不要
- リリースワークフローは既に設定済み（`publish-homebrew-formula` ジョブ）
