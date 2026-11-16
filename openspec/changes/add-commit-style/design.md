# 設計: add-commit-style

## 設計判断

### 1. スタイルの表現方法

| 方針 | 概要 | 評価 |
|------|------|------|
| A: enum + `clap::ValueEnum` | `CommitStyle` を `prompt.rs` に定義し、clap で直接パース | ✅ 採用 |
| B: 文字列 + `FromStr` | `Provider` と同様に手動で `FromStr` を実装 | △ ボイラープレートが多い |
| C: プロンプトテンプレートファイル | ユーザー定義テンプレートを読み込む | ✗ 現時点では過剰 |

#### 採用理由（方針A）

- `ValueEnum` により `--help` の possible values 表示とバリデーションが自動で得られる
- スタイルとプロンプトの対応を `prompt.rs` に閉じ込められる
- `#[default]` で `Conventional` をデフォルトとして明示できる

### 2. 環境変数の読み取り箇所

`LLM_COMMIT_STYLE` は clap の `env` 属性で読み取り、`Config::from_env()` では読まない。

- CLI → 環境変数 → デフォルトの優先順位を clap が一元的に処理する
- 不正な値は clap がエラーとして報告する（`LLM_TEMPERATURE` のように黙って無視しない）
- `Config::from_env()` は `CommitStyle::default()` で初期化し、`with_style()` で上書きする

### 3. プロンプトの構成

共通要件（命令形、1行目 72 文字以内、説明文のみ出力など）は `build_user_prompt()` に1度だけ記述し、スタイル固有の形式要件のみを `CommitStyle::format_requirements()` から差し込む。

### 4. Scoped Commits の補足ルール

scopedcommits.com は `<scope>: <description>`、任意の本文、任意のトレーラーという骨格のみを定義し、詳細はプロジェクトごとのルールに委ねている。LLM が安定した出力をするため、以下をプロンプトに追加する。

- スコープは変更されたパス（crate、パッケージ、ディレクトリ、コンポーネント名）から導出する
- Conventional Commits の型プレフィックス（`feat:` など）を使わない
- 複数領域にまたがる場合は、より一般的なスコープを使うかカンマ区切りで列挙する。ツリー全体の変更には `treewide` を使う（サイトの FAQ に準拠）
- 説明文は小文字で始め、末尾にピリオドを付けない（サイトの例の大半に準拠）
- 本文はヘッダーと空行で区切る

## 将来の拡張

- プロジェクト固有のスコープ一覧や形式ルールをユーザーが指定できる仕組み（例: `.jj-desc.toml`）
- 追加スタイル（例: gitmoji）
