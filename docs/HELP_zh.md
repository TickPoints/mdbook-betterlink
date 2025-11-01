# 帮助
- [配置](#配置)
- [支持](#支持)
- [开发](#开发)

# 配置
我们支持您在`book.toml`中编辑`[preprocessor.betterlink]`下面的字段来进行配置:
```toml
[preprocessor.betterlink]
# 我们还有下面这些可供选择的配置:

## 设为true以仅为中文标题添加`<a>`标签
## 默认: false
## **不推荐**: 因为为所有标题添加`<a>`完全不破坏原始逻辑，而且令像带`-`等特殊符号的英文标题也可以正常使用
add_link_for_chinese = false

## 设为true以显示处理过后的内容(在每一篇文章处理完后输出一次)
## 默认: true
## **特殊**: 只有在Debug模式下编译的文件才有用
display_processed_contexts = true

## 设为true以在预处理时做链接检查
## 默认: true
do_link_check = true

## 设为true以使用旧的`tag_adder`(即`add_a_tag`函数)
## 新的`tag_adder`功能不稳定，该配置仅为临时处理
## 默认: false (debug: false)
## **特殊**: 在Debug模式下编译的文件行为不同
use_old_tag_adder = true

[preprocessor.betterlink.link_checker]
# 配置链接检查器

## 设置发现不良链接时提示的等级
## 该提示等级由大到小介于 [1, 5]
## 默认: 1 (`Level::Error`)
prompt_level = 1

## 设置屏蔽的黑名单
## 如果指向这些链接，它们将会被直接警告(这先于其他判断逻辑)
## 这些链接需要完全匹配，并且可以是路径(未来可能会优化)
## 该配置应该以列表形式提供
## 默认: [] (`HashSet::default()`)
black_list = ["example"]
```

# 支持
我们对一些内容进行支持:

## Markdown
我们使用[`pulldown-cmark`库](https://crates.io/crates/pulldown-cmark)来进行`Markdown`处理。由于其支持`CommonMark`，所以我们也同样支持。

另外我们支持下面一些扩展情况:
- GitHub-compatible footnote syntax
- TeX formulas
- Blockquote tags

其中，**Blockquote tags**原生MDBOOK不支持，可以通过[其他插件](https://github.com/lambdalisue/rs-mdbook-alerts)以支持。**TeX formulas**现在只支持使用`$`式的，自定义的可能导致错误(同理，原生的MDBOOK数学扩展我们不支持，因为它 **不是TeX formulas** ，可以通过[其他插件](https://github.com/lzanini/mdbook-katex)来使用)。

> [!WARNING]
> `pulldown-cmark`化的`add_heading_anchors`功能仍不稳定，仅`-pre`版本可用。

## 链接形式
我们进行链接检查时，会对以下链接进行特定情况的处理:

> [!NOTE]
> **从多方面考虑，我们只检查`url`能否被正常解析，不会尝试测试访问连通性。**

| 链接类型 | 一般语法 | 处理原则 | 其他 |
|:-------:|:-------:|:-------|:-------|
| **损坏** | _受损的链接没有一般语法_ | 总发出警告 | 损坏链接是指语法上就有错误的链接，包括`ReferenceUnknown`，`ShortcutUnknown`，`CollapsedUnknown` |
| **内联** | 形如`[name](url)` | 遇到不良链接时发出警告 | 检查链接指向的`url`是否可以被正常访问(或者如果作为文件路径是否存在文件) |
| **参考** | 形如`[name][note]` | 忽视 | 参考链接是指向脚注的一种形式。_未来我们可能会支持_ |
| **折叠** | 形如`[note][]` | 忽视 | 折叠链接是指向脚注的一种形式(与参考相似，表示`name`与`note`相同时省略`note`)。_未来我们可能会支持_ |
| **快捷**(_直接脚注_) | 形如`[note]` | 忽视 | 快捷链接是指向脚注的一种形式(与参考相似，表示`name`与`note`相同时省略`[note]`)。_未来我们可能会支持_ |
| **自动或电子邮件** | 形如`<url>` | 遇到不良链接时发出警告 | 这类链接取消了`name`直接展示指向一般网址或电子邮件的`url`，我们会检查是否可以正常访问`url`(该行为不讨论作为文件路径) |
| **维基** | 形如`[[page]]` | 忽视 | 维基链接 **不是** CommonMark标准的一部分。**原则上我们不会兼容它。** |

> [!WARNING]
> 自动链接或电子邮件的检查器暂不稳定。

# 开发
## Debug模式
**debug 编译(Debug Build)** 是指使用默认配置进行的编译方式，主要用于开发和调试阶段。包含 **调试信息表(Debug Info)** 和 **调试断言(Debug Assert)**。

本项目自`v0.3.7-pre`使用 **带debug的release编译** 来 应用优化等其他内容的情况下使用调试信息表和断言。(仅`-pre`预览版)
