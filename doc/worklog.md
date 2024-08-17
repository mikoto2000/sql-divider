# プロジェクト初期化

```sh
$ npm create tauri-app@latest -- --rc
✔ Project name · sql-divider
✔ Identifier · dev.mikoto2000.sql.divider
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, bun)
✔ Choose your package manager · npm
✔ Choose your UI template · React - (https://react.dev/)
✔ Choose your UI flavor · TypeScript

Template created! To get started run:
  cd sql-divider
  npm install
  npm run tauri android init

For Desktop development, run:
  npm run tauri dev

For Android development, run:
  npm run tauri android dev

npm notice
npm notice New minor version of npm available! 10.7.0 -> 10.8.2
npm notice Changelog: https://github.com/npm/cli/releases/tag/v10.8.2
npm notice To update run: npm install -g npm@10.8.2
npm notice
```

# 依存関係追加

## SQL 発行

```sh
cargo add dotenv
cargo add tokio --features full
cargo add sqlx --features postgres,runtime-tokio
```

## UI コンポーネントフレームワーク追加

```sh
npm install @mui/material @emotion/react @emotion/styled
```

## SQL 発行結果を表示するための表を描画するためのコンポーネントを追加

```sh
npm i @mui/x-data-grid
```

## SQL パーサーの追加

```sh
cargo add sqlparser
```

## アイコンの追加

```sh
npm i @mui/icons-material
```
