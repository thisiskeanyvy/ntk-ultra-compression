# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Compilation
-----------

Requiert Rust 1.55+

``````bash
$ npm install
``````

``````bash
$ npm run tauri dev
``````

``````bash
$ npm run tauri build -- --no-bundle
# bundle for distribution outside the macOS App Store
$ npm run tauri bundle -- --bundles app,dmg
# bundle for App Store distribution
$ npm run tauri bundle -- --bundles app --config src-tauri/tauri.appstore.conf.json
``````

[https://v2.tauri.app/fr/distribute/](https://v2.tauri.app/fr/distribute/)