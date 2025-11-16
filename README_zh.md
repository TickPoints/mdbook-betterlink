# BetterLink
[![Crates.io](https://img.shields.io/crates/v/mdbook-betterlink?style=flat)](https://crates.io/crates/mdbook-betterlink)
[![Downloads](https://img.shields.io/crates/d/mdbook-betterlink?style=flat)](https://crates.io/crates/mdbook-betterlink)
[![License](https://img.shields.io/crates/l/mdbook-betterlink?style=flat)](https://crates.io/crates/mdbook-betterlink)
[![Stars](https://img.shields.io/github/stars/TickPoints/mdbook-betterlink?style=flat)](https://github.com/TickPoints/mdbook-betterlink)
[![Issues](https://img.shields.io/github/issues/TickPoints/mdbook-betterlink?style=flat)](https://github.com/TickPoints/mdbook-betterlink/issues)

[**English**](https://github.com/TickPoints/mdbook-betterlink/blob/main/README.md)
**中文**

> [!WARNING]
> 该仓库现已不再积极更新。

**BetterLink** 用来为 [MDBOOK](https://github.com/rust-lang/mdBook) 提供更好的超链接效果。

## Why
解决本人认为的**MDBook**超链接方面的一些痛点:
- [x] 为带有更多符号或语言的标题也添加`<a>`标签
- [x] 检查无效链接

## How
**BetterLink** 是一个用作 [预处理器](https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html) 的插件。您可以向使用别的插件一样使用 **BetterLink**。

1. 安装[**`mdbook-betterlink`**](https://crates.io/crates/mdbook-betterlink)
```shell
cargo install mdbook-betterlink
```
**注**: 或使用[`binstall`](https://github.com/cargo-bins/cargo-binstall)或从发行版下载。

2. 使用: 在`book.toml`添加:
```toml
[preprocessor.betterlink]
# 使用betterlink
```

更多内容可以访问[HELP.md](https://github.com/TickPoints/mdbook-betterlink/blob/main/docs/HELP_zh.md)。

## Changelog
在这里查看[CHANGELOG.md](https://github.com/TickPoints/mdbook-betterlink/blob/main/docs/CHANGELOG.md)。
