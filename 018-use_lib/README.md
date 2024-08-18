Cargo.tomlにクレート名とバージョンを追加

```toml
[package]
name = "use_lib"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.7" # <- 追加
```

以下のコマンドを実行するとライブラリをダウンロードする。

```shell
$ cargo  build
```
