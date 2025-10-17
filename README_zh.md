# BetterLink
[![Crates.io](https://img.shields.io/crates/v/mdbook-betterlink?style=flat)](https://crates.io/crates/mdbook-betterlink)
[![Downloads](https://img.shields.io/crates/d/mdbook-betterlink?style=flat)](https://crates.io/crates/mdbook-betterlink)
[![License](https://img.shields.io/crates/l/mdbook-betterlink?style=flat)](https://crates.io/crates/mdbook-betterlink)
[![Stars](https://img.shields.io/github/stars/TickPoints/mdbook-betterlink?style=flat)](https://github.com/TickPoints/mdbook-betterlink)
[![Issues](https://img.shields.io/github/issues/TickPoints/mdbook-betterlink?style=flat)](https://github.com/TickPoints/mdbook-betterlink/issues)

[**English**](https://github.com/TickPoints/mdbook-betterlink/blob/main/README.md)
**中文**

**BetterLink** 用来为 [MDBOOK](https://github.com/rust-lang/mdBook) 提供更好的超链接效果。

## Why
解决本人认为的**MDBook**超链接方面的一些痛点:
> [!WARNING]
> `do_link_check` 功能现在存在以下问题:
> - [ ] 对部分情况可能未覆盖
> - [ ] 对几个特殊链接形式还不支持

- [x] 为其它语言(主要为中文)的标题也添加`<a>`标签
- [x] 检查无效链接

## How
**BetterLink** 是一个用作 [预处理器](https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html) 的插件。您可以向使用别的插件一样使用 **BetterLink**。

1. 安装[**`mdbook-betterlink`**](https://crates.io/crates/mdbook-betterlink)
```shell
cargo install mdbook-betterlink
```
**注**: 或使用[`binstall`](https://github.com/cargo-bins/cargo-binstall)或从发行版下载。

2. 使用: 在`book.toml`添加:
```shell
[preprocessor.betterlink]
# 使用betterlink
# 我们还有下面这些可供选择的配置:

## 设为true以仅为中文标题添加`<a>`标签
## 默认为false
## **不推荐**: 因为为所有标题添加`<a>`完全不破坏原始逻辑，而且令像带`-`等特殊符号的英文标题也可以正常使用
add_link_for_chinese = false

## 设为true以显示处理过后的内容(在每一篇文章处理完后输出一次)
## 默认为true
## **特殊**: 只有在Debug模式下编译的文件才有用
display_processed_contexts = true

## 设为true以在预处理时做链接检查
## 默认为true
do_link_check = true

[preprocessor.betterlink.link_checker]
# 配置链接检查器
# **警告**: 以下内容均不稳定，酌情使用

## 设置发现不良链接时提示的等级
## 该提示等级由大到小介于 [1, 5]
## 默认为1 (`Level::Error`)
prompt_level = 1
```

## Changelog
在这里查看[更改日志](https://github.com/TickPoints/mdbook-betterlink/blob/main/CHANGELOG.md)。
