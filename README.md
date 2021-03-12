# bsharp-lang

## Overview
This ***B#(b-sharp)*** is an original programming language written in Rust. This language, ***B#***, is extended from 'Visual Basic'.  

## Development Environment
### Install
- `curl https://sh.rustup.rs -sSf | sh`
- Choose `1) Proceed with installation (default)`
```
   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
```
- `source $HOME/.cargo/env`
- `cargo init --bin`

### Debug
- `cargo run samples/test.bs`
- VS Code `CodeLLDB` を導入
- `launch.json` を `LLDB` で作成する

### Test run
`cargo run samples/test.bs`

## TODO
- [x] IF ELSE 構文のサポート
- [x] FOR構文のサポート
- [x] 文字列出力のサポート
- [x] エラーメッセージをわかりやすく
- [x] FIZZBUZZ
- [ ] キーワードのマッチング方法(全部小文字でチェックする)

## 引用
- VBA構文: https://docs.microsoft.com/ja-jp/office/vba/api/overview/language-reference
- Java BNF: https://users-cs.au.dk/amoeller/RegAut/JavaBNF.html
- Go by Rust: https://github.com/rariyama/imitation_interpreter