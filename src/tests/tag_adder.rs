use crate::handler::book_handler::tag_adder;

#[test]
fn test_contains_chinese() {
    use tag_adder::contains_chinese;
    assert!(contains_chinese("这是一个标题"));
    assert!(contains_chinese("Hello 世界"));
    assert!(!contains_chinese("Hello World"));
    assert!(contains_chinese("こんにちは你好"));
    assert!(contains_chinese("\u{4e00}")); // Minimum Chinese character
    assert!(contains_chinese("\u{9fff}")); // Maximum frequently used Chinese character
    assert!(contains_chinese("\u{3400}")); // Extended Area A Start
}

#[test]
fn test_add_heading_anchors() {
    use tag_adder::add_heading_anchors;
    let mut content = r#"
# Title1
## Subtitle1

# Title2
## Subtitle2

# SameTitle

# 中文标题

# SameTitle

# 带-标题

```md
# 代码框内标题
```

[Title1](#Title1)

[^note1]

[^脚标测试]

[^note1]: 1

[^脚标测试]: 2
"#.to_string();
    add_heading_anchors(&mut content, false);
    println!("{}", content);
    assert!(content.contains(r#"<a id="title1">"#));   // Note: the title is converted to lowercase by default
    assert!(content.contains(r#"<a id="title2">"#));
    assert!(content.contains(r#"<a id="subtitle1">"#));
    assert!(content.contains(r#"<a id="subtitle2">"#));
    assert!(content.contains(r#"<a id="中文标题">"#));
    assert!(content.contains(r#"<a id="带-标题">"#));
    assert!(content.contains(r#"<a id="sametitle">"#));
    assert!(content.contains(r#"<a id="sametitle-1">"#));
    assert!(!content.contains(r#"<a id="代码框内标题">"#));
}
