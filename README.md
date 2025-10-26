# BetterLink
[![Crates.io](https://img.shields.io/crates/v/mdbook-betterlink?style=flat)](https://crates.io/crates/mdbook-betterlink)
[![Downloads](https://img.shields.io/crates/d/mdbook-betterlink?style=flat)](https://crates.io/crates/mdbook-betterlink)
[![License](https://img.shields.io/crates/l/mdbook-betterlink?style=flat)](https://crates.io/crates/mdbook-betterlink)
[![Stars](https://img.shields.io/github/stars/TickPoints/mdbook-betterlink?style=flat)](https://github.com/TickPoints/mdbook-betterlink)
[![Issues](https://img.shields.io/github/issues/TickPoints/mdbook-betterlink?style=flat)](https://github.com/TickPoints/mdbook-betterlink/issues)

**English**
[**中文**](https://github.com/TickPoints/mdbook-betterlink/blob/main/README_zh.md)

BetterLink is designed to provide better hyperlink effects for MDBOOK.

## Why
- [x] Adds `<a>` tags for titles with more symbols or languages as well
- [x] Checks for invalid links

## How
BetterLink is a plugin that functions as a preprocessor. You can use BetterLink just like any other plugin.

1. Install [**`mdbook-betterlink`**](https://crates.io/crates/mdbook-betterlink):
```shell
cargo install mdbook-betterlink
```
**Tip**: Or use [`binstall`](https://github.com/cargo-bins/cargo-binstall) or download from the releases.

2. Usage: Add the following to `book.toml`:
```toml
[preprocessor.betterlink]
# Enable betterlink
```

更多内容可以访问[HELP.md](https://github.com/TickPoints/mdbook-betterlink/blob/main/docs/HELP.md)。

## Changelog
View the [CHANGELOG.md](https://github.com/TickPoints/mdbook-betterlink/blob/main/docs/CHANGELOG.md) here.
