# gh-activity プロジェクト概要

このリポジトリは GitHub CLI の拡張機能（`gh` extension）です。`gh pr list` の各オプションを対話的に組み立て、ターミナル上で素早く Pull Request を検索できるラッパーを提供します。

## 主な機能
- イベント種別と期間指定: `created` / `closed` / `merged` を選択し、開始日・終了日をカレンダー入力して `--search "<type>:YYYY-MM-DD..YYYY-MM-DD"` を生成。
- リポジトリ指定: 空入力ならカレント、指定時は `-R <owner>/<repo>` を付与。
- 作者指定: 任意で `--author`（デフォルト候補 `@me`）。
- 取得件数: `--limit`（デフォルト 30）。
- 出力形式: 通常出力または `--json`。JSON の場合は出力フィールドを複数選択可能。
- 実行方法の選択: 生成コマンドをそのまま実行するか、クリップボードにコピーするかを選べます。

## 使い方（概要）
- インストール: `gh extension install nogtk/gh-activity`
- 実行: `gh activity`
- アップグレード/削除: `README.md` を参照してください。

## 内部構成
- `src/main.rs`: エントリポイント。`builder::gh_build()` でコマンド文字列を生成し、`runner::gh_run()` で実行/コピーを行います。
- `src/builder.rs`: `gh pr list` のサブコマンドに対し、各オプションを連結して最終的な文字列を構築します。
- `src/builder/option_builder/`
  - `date.rs`: イベント種別と期間の選択（過去2年〜当月末の範囲）。
  - `repo.rs`: 対象リポジトリの指定（空ならカレント）。
  - `author.rs`: 作者指定（任意、`@me` をヒント表示）。
  - `limit.rs`: 取得件数の指定。
  - `output.rs`: JSON 出力の有無とフィールド選択。
- `src/runner.rs`: `arboard` によるクリップボードコピー、または `tokio::process` で `gh` プロセスを起動。

## 依存と前提
- 主要依存: `inquire`（対話 UI）、`chrono`（日付）、`tokio`（非同期/プロセス）、`arboard`（クリップボード）。
- 前提: GitHub CLI (`gh`) がインストール済みで PATH にあり、`gh auth login` 済みであること。
- プロンプト表示は英語です。

## ビルド/配布
- Rust 製。`script/build.sh` はビルド済みバイナリ（`gh-activity`）を `dist/` に移動する補助スクリプトです。
- 拡張としては `gh activity` コマンドで起動されます。

## ライセンス/作者
- MIT License / Author: `nogtk`

