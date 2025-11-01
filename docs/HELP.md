# Help
- [Configuration](#configuration)
- [Support](#support)
- [Development](#development)

# Configuration
Configure by editing fields under `[preprocessor.betterlink]` in `book.toml`:

```toml
[preprocessor.betterlink]
# Available options:

## Set true to add `<a>` tags only for Chinese headings
## Default: false
## Not recommended: Adding `<a>` for all headings preserves original logic and handles English titles with special characters (e.g., -)
add_link_for_chinese = false

## Set true to display processed content (output after each article)
## Default: true
## **Special**: Only effective in Debug mode compilations
display_processed_contexts = true

## Set true to enable link checking during preprocessing
## Default: true
do_link_check = true

## Set to true to use the old `tag_adder` (i.e., the `add_a_tag` function)
## The new `tag_adder` is unstable; this option is provided as a temporary workaround
## Default: false (in Debug mode: false)
## **Special**: Behavior differs for files compiled in Debug mode
use_old_tag_adder = true

[preprocessor.betterlink.link_checker]
# Link checker configuration

## Set severity level for bad links (1=highest, 5=lowest)
## Default: 1 (Level::Error)
prompt_level = 1

## Configure Blocked Blacklist
## Links pointing to these URLs will trigger immediate warnings (prioritized over other checks)
## Requires exact URL/path matching (may be optimized in future)
## Should be provided as a list
## Default: [] (`HashSet::default()`)
black_list = ["example"]
```

# Support
Supported content types:

## Markdown
Processed via [`pulldown-cmark`](https://crates.io/crates/pulldown-cmark) with CommonMark compatibility.

Additional extensions:
- GitHub-style footnotes
- TeX formulas ($ syntax only)
- Blockquote tags

Note:
- Blockquote tags are not natively supported by mdBook and require <https://github.com/lambdalisue/rs-mdbook-alerts>
- TeX formulas only support `$` syntax. Native mdBook math extensions are unsupported (use <https://github.com/lzanini/mdbook-katex>)

> [!WARNING]
> The `add_heading_anchors` feature of `pullmark-cmark` is still unstable and only available in the `-pre` version.

## Link Validation
Handling principles:

> [!NOTE]
> **Only validates URL resolvability, not network accessibility.**

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
> Autolink/email validation currently unstable


# Development
## Debug Mode
A **Debug Build** refers to the default compilation configuration, primarily intended for development and debugging purposes. It includes **debug information tables (Debug Info)** and **debug assertions (Debug Assert)**.

Starting from version `v0.3.7-pre`, this project uses **Release builds with debug info—enabling optimizations** while still retaining debug information and assertions. (Available only in -pre preview versions.)

In `v0.3.9-pre`, this behavior is officially stable. In fact, we do this in a few lines:
```toml
[profile.release]
# Only on `-pre` release
debug = true            # Preserve Debug Information Table
debug-assertions = true # Enable Debug Assertions
```
