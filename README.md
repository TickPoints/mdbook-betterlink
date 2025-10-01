# BetterLink
[![Crates.io](https://img.shields.io/crates/v/mdbook-betterlink)](https://crates.io/crates/mdbook-betterlink)
[![Downloads](https://img.shields.io/crates/d/mdbook-betterlink)](https://crates.io/crates/mdbook-betterlink)
[![License](https://img.shields.io/crates/l/mdbook-betterlink)](https://crates.io/crates/mdbook-betterlink)
[![Docs.rs](https://img.shields.io/docsrs/mdbook-betterlink)](https://docs.rs/mdbook-betterlink)
[![Stars](https://img.shields.io/github/stars/TickPoints/mdbook-betterlink)](https://github.com/TickPoints/mdbook-betterlink)

**English**
[**中文**](README_zh.md)

BetterLink is designed to provide better hyperlink effects for MDBOOK.

## Why
It addresses some pain points I perceive regarding hyperlinks in MDBook:
- [x] Adds `<a>` tags to headings in other languages (for convenience, currently adds them unconditionally to all headings)
- [ ] Checks for invalid links

## How
BetterLink is a plugin that functions as a preprocessor. You can use BetterLink just like any other plugin.
