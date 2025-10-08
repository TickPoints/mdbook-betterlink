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
It addresses some pain points I perceive regarding hyperlinks in MDBook:
- [x] Adds `<a>` tags to headings in other languages
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
# We also have the following optional configurations available:

## Set to true to add `<a>` tags only for Chinese headings
## Default is false
## **Not recommended**: Because adding `<a>` tags to all headings doesn't break the original logic at all, 
## and allows English headings with special characters like `-` to work normally
add_link_for_chinese = false

## Set to true to display the processed contexts (output once after each article is processed)
## Default is true
## **Special**: Only files compiled in Debug mode are useful
display_processed_contexts = true

## Set to true to do link checking during preprocessing
## Default is true
do_link_check = true
```

## Changelog
View the [change log](https://github.com/TickPoints/mdbook-betterlink/blob/main/CHANGELOG.md) here.
