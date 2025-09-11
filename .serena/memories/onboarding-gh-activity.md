# gh-activity Onboarding

- 目的: `gh pr list` の検索オプションを対話的に組み立てる GitHub CLI 拡張（`gh activity`）。
- 主要依存: `inquire`, `chrono`, `tokio`, `arboard`。テスト用に `tempfile`。

## 構成
- `src/main.rs`: エントリ。`builder::gh_build()` → `runner::gh_run(...)`。
- `src/builder.rs`: コマンド構築。テスト用 `pub use GhOption` と `gh_build_with(...)` を提供。
- `src/builder/option_builder/*`: `date`/`repo`/`author`/`limit`/`output` の各オプション組立。
- `src/runner.rs`: クリップボード or 実行。`RunMode` と `gh_run_with_mode(...)` を提供。

## 開発・テスト
- Rust: 1.89.0（`rust-toolchain.toml`）。
- `cargo build` / `cargo test` で動作確認。
- E2E: PATH 先頭にダミー `gh` を配置し、`runner` の実行経路と引数受け渡しを検証（外部 API 不要）。
- クリップボード経路は環境依存のため E2E から除外（必要なら抽象化）。

## CI
- `.github/workflows/test.yml`: PR と `main` push でテスト実行。
- `ubuntu-latest`, `macos-latest` のマトリクス。Linux では `arboard` 向けに XCB 開発ライブラリを apt で導入。

## 方針
- 既存の対話挙動・CLI 互換を守る小さな変更を優先。
- テスト容易性のための最小限の公開/分離は許容（`gh_build_with`/`RunMode`）。
- 破壊的変更や大規模リファクタは別 Issue/PR で検討。

## コマンド
- `cargo test -q` / `cargo test -- --nocapture`
- インストール後の実行: `gh activity`

## 参考
- リポジトリ: `nogtk/gh-activity`
- PR (#23): 自動テスト拡充 + CI 追加
