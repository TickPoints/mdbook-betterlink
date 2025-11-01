## Notes
1. The logs are sorted in reverse order.
2. The preview version has a `-pre` suffix.
3. The preview version comes with a debug feature (after `0.3.7-pre`).

## 0.3.8-pre
- Added `old_tag_adder` to use the old `add_a_tag` function
- Added the `use_old_tag_adder` config to use the old `add_a_tag` function

### Warnings
- Temporarily fixed

## 0.3.7-pre
- Splited `tag_adder` form `book_handler` mod
- `add_a_tag` (`add_heading_anchors`) has been pull-cmarkized
- `pulldown-cmark-to-cmark` has been added

### Warnings
- Changes to `add_a_tag` (`add_heading_anchors`) are not stable yet
- Untested (There are extreme problems, and the new code sucks)
- Bad code will be fixed in the next release. We may need two more previews.

## 0.3.6-pre
- Added the `black_list` config
- Added the `check` subcommand (**todo**)
- Updated docs
- Fixed logger

## 0.3.5
- Supported compatibility with TeX formulas and Blockquote tags parsing in `link_checker`
- Updated deps

## 0.3.4-pre
- Refactored:
    - Splited mods
- Updated docs
- Supported for more Link

## 0.3.3
- Fixed path system
- Refactored:
    - Splited mods
    - Used clearer attributes
- Exported more content to read document comment
- Updated docs

## 0.3.2-pre
- Added the `pulldown-cmark` crate for simplify parsing
- Added more docs comments to help understand
- Refactored the `link_checker` mod

## 0.3.1-pre
- Refactored simply to keep clippy happy
- Updated documentation
- Formatted logger

## 0.3.0-pre
- Added the `do_link_check` config
- Added the `link_checker` mod to complete the link check
- Added the `url` crate to check url

## 0.2.3-pre
- Updated compile profile.
- Added a debug log to alert user debugging_assertions

## 0.2.2
- Optimized version parsing

## 0.2.1
- Added the `display_processed_contexts` config
- Fixed the version parsing error simply

## 0.2.0
- Improved the relevant documents
- Improved relevant metadata
- Added the config system
- Added the `add_link_for_chinese` config

## 0.1.5-pre
- Fixed `.github/workflows/release.yml`
- Added the config system
- Added the `add_link_for_chinese` config

## 0.1.4-pre
- Fixed `.github/workflows/release.yml`
- Added profile options

## 0.1.3-pre
- Try to use `taiki-e/upload-rust-binary-action`

## 0.1.2-pre
- Fixed `.github/workflows/release.yml`

## 0.1.1
- Updated documentation
- Updated automated workflows
- Updated metadata

## 0.1.0
- Initialized the project and completed preliminary construction
- Initialized documentation
