# CI ワークフロー仕様

## ADDED Requirements

### Requirement: 自動ユニットテスト実行

CI システム SHALL コード変更が提案またはマージされた際に、すべてのユニットテストを自動的に実行する。

#### Scenario: プルリクエストでテスト実行をトリガー

**Given** 開発者が main ブランチをターゲットとしたプルリクエストを作成する
**When** プルリクエストが開かれるか更新される
**Then** CI ワークフローが `cargo test --all-features` を実行する
**And** テスト結果がプルリクエストに報告される

#### Scenario: main ブランチへのプッシュでテスト実行をトリガー

**Given** コミットが main ブランチにマージされる
**When** プッシュイベントが発生する
**Then** CI ワークフローがすべてのユニットテストを実行する
**And** 結果がリポジトリの Actions タブで確認できる

### Requirement: 複数 Rust バージョンでのテスト

CI システム SHALL 最小サポート Rust バージョン（MSRV）と最新安定版の両方でテストを実行する。

#### Scenario: MSRV 互換性チェック

**Given** プロジェクトが Cargo.toml で rust-version = "1.85" を指定している
**When** CI ワークフローが実行される
**Then** Rust 1.85 でテストが実行される
**And** Rust stable でテストが実行される
**And** ワークフローが成功するには両バージョンで合格する必要がある

### Requirement: コード品質チェック

CI システム SHALL 自動リントおよびフォーマットチェックによりコード品質を強制する。

#### Scenario: Clippy リントチェック

**Given** CI ワークフローが実行される
**When** リントジョブが実行される
**Then** `cargo clippy -- -D warnings` が実行される
**And** clippy の警告があればジョブが失敗する

#### Scenario: フォーマットチェック

**Given** CI ワークフローが実行される
**When** フォーマットジョブが実行される
**Then** `cargo fmt --check` が実行される
**And** フォーマットの問題があればジョブが失敗する

### Requirement: ビルドキャッシュ

CI システム SHALL ワークフロー実行時間を短縮するためにビルド成果物をキャッシュする。

#### Scenario: キャッシュされたビルド

**Given** 以前の CI 実行が完了している
**When** 依存関係が変更されていない状態で新しい CI 実行が開始される
**Then** キャッシュされた Cargo レジストリとビルド成果物が復元される
**And** コールドビルドと比較してビルド時間が大幅に短縮される
