# jj-desc

[jj (Jujutsu)](https://github.com/martinvonz/jj) のコミット説明文を LLM を使って自動生成するCLIツールです。

[English](README.md) | 日本語

## 特徴

- 🤖 diff から LLM を使って意味のあるコミット説明文を自動生成
- 📦 **バッチ処理**: revset を使って複数のコミットを一括処理
- 🔄 jj の undo ワークフローとシームレスに連携（確認プロンプト不要）
- 🎯 複数の LLM プロバイダーに対応: OpenRouter, OpenAI, Anthropic, Gemini
- 🔌 カスタムエンドポイント対応（Azure OpenAI, Ollama, LM Studio など）
- 🔍 `--dry-run` でプレビューモード
- 💬 インタラクティブモードで適用前に確認
- 🎚️ jj の revset 構文で柔軟にターゲット指定
- 📝 [Conventional Commits](https://www.conventionalcommits.org/) 形式に準拠
- 🔀 マージコミットを自動処理（空のマージコミットは LLM 呼び出し不要）
- ⚡ 大きな diff に最適化: ロックファイルを自動除外、バイナリファイルを簡略化
- 🎛️ `--exclude` オプションでファイル除外をカスタマイズ可能

## デモ

![jj-desc demo](docs/demo.gif)

## インストール

### Homebrew (macOS/Linux)

macOS または Linux での推奨インストール方法:

```bash
brew install tumf/tap/jj-desc
```

### ビルド済みバイナリ

[最新リリース](https://github.com/tumf/jj-desc/releases/latest)からプラットフォーム用のビルド済みバイナリをダウンロードできます。

#### macOS

**インストーラースクリプトを使用（推奨）:**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.sh | sh
```

**手動ダウンロード:**

- Apple Silicon (M1/M2/M3): [jj-desc-aarch64-apple-darwin.tar.xz](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-aarch64-apple-darwin.tar.xz)
- Intel: [jj-desc-x86_64-apple-darwin.tar.xz](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-apple-darwin.tar.xz)

```bash
# Apple Silicon の例
curl -LO https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-aarch64-apple-darwin.tar.xz
tar xf jj-desc-aarch64-apple-darwin.tar.xz
sudo mv jj-desc /usr/local/bin/
```

#### Linux

**インストーラースクリプトを使用（推奨）:**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.sh | sh
```

**手動ダウンロード:**

- x86_64: [jj-desc-x86_64-unknown-linux-gnu.tar.xz](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-unknown-linux-gnu.tar.xz)
- ARM64: [jj-desc-aarch64-unknown-linux-gnu.tar.xz](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-aarch64-unknown-linux-gnu.tar.xz)

```bash
# x86_64 の例
curl -LO https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-unknown-linux-gnu.tar.xz
tar xf jj-desc-x86_64-unknown-linux-gnu.tar.xz
sudo mv jj-desc /usr/local/bin/
```

#### Windows

**PowerShell インストーラーを使用（推奨）:**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.ps1 | iex"
```

**手動ダウンロード:**

[jj-desc-x86_64-pc-windows-msvc.zip](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-pc-windows-msvc.zip) をダウンロードし、`jj-desc.exe` を PATH の通ったディレクトリに展開してください。

### ソースからビルド (Cargo)

Rust がインストールされている場合:

```bash
cargo install --git https://github.com/tumf/jj-desc
```

または、ローカルクローンからビルド:

```bash
git clone https://github.com/tumf/jj-desc
cd jj-desc
cargo install --path .
```

### 必要条件

- [jj](https://github.com/martinvonz/jj) がインストールされ、PATH で利用可能であること
- 使用する LLM プロバイダーの API キー
- ソースからビルドする場合: Rust 1.85+ (Edition 2024)

## 設定

### LLM プロバイダーの選択

`LLM_PROVIDER` 環境変数または `--provider` CLI オプションで LLM プロバイダーを選択:

```bash
export LLM_PROVIDER=openai        # オプション: openrouter, openai, anthropic, gemini
```

### 環境変数

#### 共通設定

| 変数 | 必須 | デフォルト | 説明 |
|------|------|------------|------|
| `LLM_PROVIDER` | ❌ | `openrouter` | 使用する LLM プロバイダー |
| `LLM_MODEL` | ❌ | (プロバイダーのデフォルト) | モデルを上書き |

#### プロバイダー別 API キー

| プロバイダー | 環境変数 | API キー取得先 |
|-------------|---------|---------------|
| OpenRouter | `OPENROUTER_API_KEY` | [OpenRouter](https://openrouter.ai/) |
| OpenAI | `OPENAI_API_KEY` | [OpenAI Platform](https://platform.openai.com/) |
| Anthropic | `ANTHROPIC_API_KEY` | [Anthropic Console](https://console.anthropic.com/) |
| Gemini | `GEMINI_API_KEY` | [Google AI Studio](https://aistudio.google.com/) |

#### プロバイダー別ベース URL（オプション）

カスタムセットアップ用にデフォルトの API エンドポイントを上書き:

| プロバイダー | 環境変数 | デフォルト値 |
|-------------|---------|-------------|
| OpenRouter | `OPENROUTER_BASE_URL` | `https://openrouter.ai/api/v1` |
| OpenAI | `OPENAI_BASE_URL` | `https://api.openai.com/v1` |
| Anthropic | `ANTHROPIC_BASE_URL` | `https://api.anthropic.com` |
| Gemini | `GEMINI_BASE_URL` | `https://generativelanguage.googleapis.com/v1beta/openai` |

#### プロバイダー別デフォルトモデル

| プロバイダー | デフォルトモデル |
|-------------|-----------------|
| OpenRouter | `anthropic/claude-sonnet-4` |
| OpenAI | `gpt-4o` |
| Anthropic | `claude-sonnet-4-20250514` |
| Gemini | `gemini-2.0-flash` |

### 設定例

#### OpenRouter（デフォルト）

```bash
export OPENROUTER_API_KEY="your-api-key-here"
```

#### OpenAI

```bash
export LLM_PROVIDER=openai
export OPENAI_API_KEY="sk-..."
```

#### Anthropic

```bash
export LLM_PROVIDER=anthropic
export ANTHROPIC_API_KEY="sk-ant-..."
```

#### Gemini

```bash
export LLM_PROVIDER=gemini
export GEMINI_API_KEY="..."
```

#### Azure OpenAI

```bash
export LLM_PROVIDER=openai
export OPENAI_API_KEY="your-azure-key"
export OPENAI_BASE_URL="https://your-resource.openai.azure.com/openai/deployments/your-deployment"
export LLM_MODEL="gpt-4"
```

#### Ollama（ローカル LLM）

```bash
export LLM_PROVIDER=openai
export OPENAI_API_KEY="dummy"  # Ollama はキー不要
export OPENAI_BASE_URL="http://localhost:11434/v1"
export LLM_MODEL="llama2"
```

#### LM Studio（ローカル LLM）

```bash
export LLM_PROVIDER=openai
export OPENAI_API_KEY="dummy"  # LM Studio はキー不要
export OPENAI_BASE_URL="http://localhost:1234/v1"
export LLM_MODEL="your-model-name"
```

恒久的な設定には、これらをシェル設定ファイル（`~/.bashrc`, `~/.zshrc` など）に追加してください。

## 使い方

### 基本的な使い方

デフォルトでは、`jj-desc` は `::@ & mutable()` 内の説明文がないすべての変更可能なコミットに対して説明文を生成します:

```bash
jj-desc
```

### 単一のコミットに説明文を生成

現在のワーキングコピーに説明文を生成して適用:

```bash
jj-desc -r @
```

### 特定のリビジョンをターゲット

jj の revset 構文を使って特定のリビジョンをターゲット:

```bash
# 自分のコミットを処理
jj-desc -r "mine()"

# 特定の範囲のコミットを処理
jj-desc -r "@..main"

# 特定のリビジョンを処理
jj-desc -r @-

# 処理するコミット数を制限
jj-desc -n 5

# 適用前にプレビュー
jj-desc --dry-run

# インタラクティブモード - 各説明文を確認
jj-desc -i
```

### プレビューモード

適用せずに生成される説明文を確認:

```bash
jj-desc --dry-run
```

### 別のプロバイダーを使用

プロバイダーを上書き:

```bash
jj-desc --provider openai
jj-desc --provider anthropic
```

### 別のモデルを使用

デフォルトモデルを上書き:

```bash
jj-desc --model gpt-4o
# または
jj-desc --model anthropic/claude-3.5-sonnet
```

### 詳細ログ

デバッグ用に詳細ログを有効化:

```bash
jj-desc --verbose
```

または `RUST_LOG` 環境変数を使用:

```bash
RUST_LOG=debug jj-desc
```

### diff からファイルを除外

LLM に送信する diff から特定のファイルやパターンを除外:

```bash
# 特定のファイルを除外
jj-desc --exclude "*.json" --exclude "*.yaml"

# 短縮形
jj-desc -x "docs/*" -x "*.lock"
```

**自動的に除外されるファイル:**
- ロックファイル: `Cargo.lock`, `package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`, `*.lock`, `*.lockb`
- バイナリファイルは自動的に `"Binary file {path} changed"` に簡略化

**ファイルを除外する理由:**
- トークン使用量とコストを削減
- コンテキスト制限超過による API エラーを防止
- 意味のある変更に焦点を当てて説明文の品質を向上

**大きな diff の警告:**
フィルタリング後の diff が 50KB を超えると警告が表示されます:
```
⚠ Warning: Diff is large (75000 bytes, 3500 lines)
  Consider splitting into smaller commits.
```

### コマンドラインオプション

```
Usage: jj-desc [OPTIONS]

Options:
      --dry-run                    適用せずに生成される説明文をプレビュー
      --provider <PROVIDER>        使用する LLM プロバイダー (openrouter, openai, anthropic, gemini)
      --model <MODEL>              使用する LLM モデルを上書き
      --max-tokens <MAX_TOKENS>    LLM レスポンスの最大トークン数
      --temperature <TEMPERATURE>  LLM レスポンスの temperature (0.0-2.0)
  -r, --revisions <REVISIONS>      ターゲットコミットを選択する revset [default: "::@ & mutable()"]
  -n, --limit <LIMIT>              処理するコミットの最大数
  -i, --interactive                各説明文を適用する前に確認
  -x, --exclude <EXCLUDE>          diff から除外するファイル（複数指定可）
  -v, --verbose                    詳細ログを有効化
  -h, --help                       ヘルプを表示
  -V, --version                    バージョンを表示
```

## 使用例

### 例1: 基本的なワークフロー

```bash
# 変更を加える
echo "fn hello() {}" >> lib.rs

# 現在のワーキングコピーに説明文を生成
jj-desc

# 出力:
# Applied description:
# ─────────────────────
# feat: add hello function
```

### 例2: 適用前にプレビュー

```bash
jj-desc --dry-run

# 出力:
# Generated description (not applied):
# ─────────────────────
# feat(auth): add JWT authentication
```

### 例3: 複数のコミットを処理

```bash
# 説明文がないすべての変更可能なコミットに説明文を生成
jj-desc

# 出力:
# Found 3 commit(s) without descriptions
# Processing 3 commit(s)
#
# Processing: 1/3 (33%)
# Commit: abc123def456
# Generated description:
#   feat(auth): add authentication endpoint
# ✓ Description applied
#
# Processing: 2/3 (66%)
# Commit: def456ghi789
# Generated description:
#   fix(auth): fix validation bug in login form
# ✓ Description applied
#
# Processing: 3/3 (100%)
# Commit: ghi789jkl012
# Generated description:
#   chore(deps): update dependencies
# ✓ Description applied
#
# ═══════════════════════
# Summary:
#   Success:  3
#   Skipped:  0
#   Failed:   0
# ═══════════════════════
```

### 例4: インタラクティブモード

```bash
jj-desc -i -r "mine()"

# 各コミットで以下が表示されます:
# Processing: 1/5 (20%)
# Commit: abc123
# Generated description:
#   feat(auth): add user authentication
#
# Full description:
# ─────────────────────
# feat(auth): add JWT authentication
#
# Implements login and logout endpoints with secure
# token generation and validation.
# ─────────────────────
# Accept (a) / Skip (s) / Quit (q): a
# ✓ Description applied
```

## 動作の仕組み

1. `jj diff` を実行して現在の変更を取得
2. LLM 処理のために **diff をフィルタリング**:
   - ロックファイル（`Cargo.lock`, `package-lock.json` など）を自動除外
   - バイナリファイルを `"Binary file {path} changed"` に簡略化
   - `--exclude` で指定されたユーザー除外を適用
   - diff が 50KB を超えると警告
3. フィルタリング後の diff が空の場合:
   - マージコミットかどうかを確認（`jj log -T 'parents.len()'` を使用）
   - マージコミットの場合、LLM を呼び出さずに "Merge commit" を設定
   - そうでない場合はエラーを返す
4. diff が空でない場合、専用のプロンプトとともに選択した LLM プロバイダー API に送信
5. `jj desc -m` を使用して生成された説明文を適用

### マージコミットの処理

jj はマージコミット自体が新しい変更を導入しないため、「空」としてマークすることがよくあります（[jj FAQ](https://docs.jj-vcs.dev/latest/FAQ/#why-are-most-merge-commits-marked-as-empty) を参照）。`jj-desc` はマージコミットを自動検出し、LLM API 呼び出しなしで適切な説明文を設定します。

## Tips: jj push エイリアスで自動化

jj エイリアスを追加することで、`jj-desc` を push ワークフローに統合できます。これにより、push のたびに `jj-desc` が自動実行されます:

```bash
# jj 設定を編集
jj config edit --user
```

以下のエイリアスを追加:

```toml
[aliases]
push = ["util", "exec", "--", "bash", "-c", """
set -e
# 説明文がないコミットに説明文を生成（jj-desc がインストールされている場合）
command -v jj-desc &> /dev/null && jj-desc
# pre-commit 設定がある場合はチェックを実行
[ ! -f .pre-commit-config.yaml ] || pre-commit run --all-files
# Push
jj git push \"$@\"
""", ""]
```

これで `jj push` が以下を実行します:
1. 説明文がないコミットに自動で説明文を生成
2. pre-commit チェックを実行（設定されている場合）
3. リモートに push

バイパスするには `jj git push` を直接使用してください。

## なぜ確認プロンプトがないのか？

git とは異なり、jj では任意の操作を非常に簡単に取り消すことができます:

- `jj undo` - 最後の操作を取り消し
- `jj op log` - 操作履歴を表示
- すべての変更は復元可能

この設計哲学により、説明文を即座に適用しても安全で、ワークフローがより高速でスムーズになります。

## ライセンス

MIT

## コントリビュート

コントリビュートを歓迎します！開発セットアップ、コーディングガイドライン、変更の提出方法については [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

## マイグレーションガイド

### 既存ユーザー向け (v0.1.x)

以前のバージョンは OpenRouter のみをサポートしていました。新バージョンは完全な後方互換性を維持しています:

- 既存の `OPENROUTER_API_KEY` 環境変数は引き続き動作
- 既存の `OPENROUTER_MODEL` 環境変数は引き続き動作
- 設定の変更は不要

新しいプロバイダーを利用するには、`LLM_PROVIDER` と適切な API キーを設定するだけです。

## 関連リンク

- [jj ドキュメント](https://martinvonz.github.io/jj/)
- [OpenRouter](https://openrouter.ai/)
- [OpenAI Platform](https://platform.openai.com/)
- [Anthropic Console](https://console.anthropic.com/)
- [Google AI Studio](https://aistudio.google.com/)
- [jj-desc についてのブログ記事](https://blog.tumf.dev/posts/diary/2026/1/8/jj-desc-release/?utm_source=github&utm_medium=readme&utm_campaign=jj-desc)
