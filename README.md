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

Regarding the link check, we do the following:

**For various reasons, we only check whether the `url` can be parsed normally, and do not try to test the access connectivity.**


| Link Type | General Syntax | Handling Principles | Others |
|:-------:|:-------:|:-------|:-------|
| **Broken**(_Unknown_) | _The broken link has no general syntax_ | Always issues warnings | A broken link is one with syntax errors, including `ReferenceUnknown`, `ShortcutUnknown`, and `CollapsedUnknown`. |
| **Inline** | `[name](url)` format | Warn on bad links | Check if the `url` is accessible (or if the file path exists). |
| **Reference** | `[name][note]` format | Neglect | Reference links point to footnotes. _Future support possible_. |
| **Collapsed** | `[note][]` format | Neglect | Collapsed links point to footnotes (similar to references, omitting `note` when name `matches`). _Future support possible_. |
| **Shortcut**(_Direct footnote_) | `[note]` format | Neglect | Shortcut links point to footnotes (like references but omit `[note]` when name matches). _Future support possible_. |
| **Autolink or Email** | `<url>` format | Warn on bad links | This type of link removes the `name` and directly displays the `url` (such as a regular web address or email). We will verify the accessibility of the `url` (excluding file path checks). |
| **WikiLink** | `[[page]]` format | Neglect | WikiLinks are **not** part of the CommonMark standard. **In principle, we will not support them.** |

> [!WARNING]
> The checker of Autolink or Email isn't stable.

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

[preprocessor.betterlink.link_checker]
# Configure Link Checker
# **Warn**: None of the following is stable, use as appropriate

## Set the level of the prompt when a bad link is found
## The level of the prompt is from high to low in [1, 5]
## Default is 1 (`Level::Error`)
prompt_level = 1
```

## Changelog
View the [change log](https://github.com/TickPoints/mdbook-betterlink/blob/main/CHANGELOG.md) here.
