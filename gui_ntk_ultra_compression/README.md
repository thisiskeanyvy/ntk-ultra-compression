# Interface Graphique Tauri

## Dark Mode (Par défaut)

<img src="https://raw.githubusercontent.com/thisiskeanyvy/ntk-ultra-compression/refs/heads/main/demo-1.png" alt="demo-1" style="zoom:50%;" />

<img src="https://raw.githubusercontent.com/thisiskeanyvy/ntk-ultra-compression/refs/heads/main/demo-2.png" alt="demo-2" style="zoom:50%;" />

## Light Mode

<img src="https://raw.githubusercontent.com/thisiskeanyvy/ntk-ultra-compression/refs/heads/main/demo-3.png" alt="demo-3" style="zoom:50%;" />

<img src="https://raw.githubusercontent.com/thisiskeanyvy/ntk-ultra-compression/refs/heads/main/demo-4.png" alt="demo-4" style="zoom:50%;" />

## Setup Recommandé

- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

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