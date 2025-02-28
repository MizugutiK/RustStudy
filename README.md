# RustStudy
- [環境構築](#-環境構築)
- [実行方法](#-実行方法)
***
# 環境構築
### ① Rustがインストールされているか確認
PowerShellで以下を実行し、バージョンが表示されることを確認
  - `rustc --version　`
  - `cargo --version`

### ② Rustのプロジェクトを作成
コマンドでプロジェクトを作る　←必須
- `cargo new @@@`
@部分がフォルダ名になる

- ` cd @@@`
で移動しておくと楽

### ③ @@@/src/main.rs を編集
作ったプロジェクトの中にsrc/main.rsが自動で作られるのでそこにRustのコードを記入
***
# 実行方法
`cargo run`
をターミナルに入れる

#### おまけ
F5やlaunch.jsonの作成でもデバッグ可能のはず
launch.jsonは拡張機能CodeLLDBとの関連性あり
