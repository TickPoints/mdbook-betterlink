# BetterLink
[![Crates.io](https://img.shields.io/crates/v/mdbook-betterlink)](https://crates.io/crates/mdbook-betterlink)
[![Downloads](https://img.shields.io/crates/d/mdbook-betterlink)](https://crates.io/crates/mdbook-betterlink)
[![License](https://img.shields.io/crates/l/mdbook-betterlink)](https://crates.io/crates/mdbook-betterlink)
[![Docs.rs](https://img.shields.io/docsrs/mdbook-betterlink)](https://docs.rs/mdbook-betterlink)
[![Stars](https://img.shields.io/github/stars/TickPoints/mdbook-betterlink)](https://github.com/TickPoints/mdbook-betterlink)

[**English**](README.md)
**中文**

**BetterLink** 用来为 [MDBOOK](https://github.com/rust-lang/mdBook) 提供更好的超链接效果。

## Why
解决本人认为的**MDBook**超链接方面的一些痛点:
- [*] 为其它语言(主要为中文)的标题也添加`<a>`标签(为了方便，目前无条件为所有标题添加)
- [x] 检查无效链接

## How
**BetterLink** 是一个用作 [预处理器](https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html) 的插件。您可以向使用别的插件一样使用 **BetterLink**。
