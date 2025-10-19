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
- [x] 为带有更多符号或语言的标题也添加`<a>`标签
- [x] 检查无效链接

关于链接检查，我们做以下处理:

| 链接类型 | 一般语法 | 处理原则 | 其他 |
|:-------:|:-------:|:-------|:-------|
| **损坏** | _受损的链接没有一般语法_ | 总发出警告 | 损坏链接是指语法上就有错误的链接，包括`ReferenceUnknown``ShortcutUnknown``CollapsedUnknown` |
| **内联** | 形如`[name](url)` | 遇到不良链接时发出警告 | 检查链接指向的`url`是否可以被正常访问(或者如果作为文件路径是否存在文件) |
| **参考** | 形如`[name][note]` | 忽视 | 参考链接是指向脚注的一种形式。_未来我们可能会支持_ |
| **折叠** | 形如`[note][]` | 忽视 | 折叠链接是指向脚注的一种形式(与参考相似，表示`name`与`note`相同时省略`note`)。_未来我们可能会支持_ |
| **快捷**(_直接脚注_) | 形如`[note]` | 忽视 | 快捷链接是指向脚注的一种形式(与参考相似，表示`name`与`note`相同时省略`[note]`)。_未来我们可能会支持_ |
| **自动或电子邮件** | 形如`<url>` | 遇到不良链接时发出警告 | 这类链接取消了`name`直接展示指向一般网址或电子邮件的`url`，我们会检查是否可以正常访问`url`(该行为不讨论作为文件路径) |
| **维基** | 形如`[[name|page]]`或`[[page]]` | 忽视 | 维基链接 **不是** CommonMark标准的一部分。**原则上我们不会兼容它。** |

> [!WARNING]
> 自动链接或电子邮件的检查器暂不稳定。

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
