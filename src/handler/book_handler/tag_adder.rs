use pulldown_cmark::{CowStr, Event, HeadingLevel, Tag, TagEnd};
use std::collections::HashMap;

/// Checks if the given text contains Chinese characters (CJK Unified Ideographs).
/// This includes:
/// - Basic CJK: U+4E00–U+9FFF
/// - Ext A:   U+3400–U+4DBF
pub fn contains_chinese(text: &str) -> bool {
    text.chars()
        .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c) || ('\u{3400}'..='\u{4dbf}').contains(&c))
}

/// Processor that appends anchor tags (`<a id="..."></a>`) after heading elements.
/// Only generates anchors for headings containing Chinese characters if `check_chinese` is enabled.
///
/// It works by:
/// 1. Collecting plain text content within a heading
/// 2. Generating a URL-safe ID (slug) from the text or using `{#id}` syntax
/// 3. Inserting `<a id="..."></a>` immediately after the closing `</hN>` tag
pub struct HeadingProcessor {
    in_code_block: bool, // Tracks whether current position is inside a code block
    current_level: HeadingLevel, // Current heading level (H1-H6)
    provided_id: Option<CowStr<'static>>, // Original ID from `{#id}` syntax
    heading_text: String, // Accumulated plain text of the heading
    is_in_heading: bool, // Whether currently processing a heading
    seen_ids: HashMap<String, usize>, // Count of each generated ID to avoid duplicates
}

impl HeadingProcessor {
    /// Creates a new instance with default state.
    pub fn new() -> Self {
        Self {
            in_code_block: false,
            current_level: HeadingLevel::H1,
            provided_id: None,
            heading_text: String::new(),
            is_in_heading: false,
            seen_ids: HashMap::new(),
        }
    }

    /// Processes a single event in the Markdown AST.
    /// Modifies heading start events to include `id` attribute when needed.
    ///
    /// # Arguments
    ///
    /// * `event` - The current event (owned static lifetime)
    /// * `output` - Mutable vector to collect processed events
    /// * `check_chinese` - If true, only add ID to headings with Chinese characters
    pub fn process_heading_event(
        &mut self,
        event: Event<'static>,
        output: &mut Vec<Event<'static>>,
        check_chinese: bool,
    ) {
        // Fast skip: if inside a code block and not closing it, just forward the event
        if self.in_code_block && !matches!(event, Event::End(TagEnd::CodeBlock)) {
            output.push(event);
            return;
        }

        match event {
            Event::Start(Tag::CodeBlock(tag)) => {
                self.in_code_block = true;
                output.push(Event::Start(Tag::CodeBlock(tag)));
            }
            Event::End(TagEnd::CodeBlock) => {
                self.in_code_block = false;
                output.push(Event::End(TagEnd::CodeBlock));
            }

            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            }) => {
                self.enter_heading(output, level, id, classes, attrs);
            }

            Event::End(TagEnd::Heading(level)) if self.is_in_heading => {
                self.exit_heading(output, level, check_chinese);
            }

            event if self.is_in_heading => {
                self.collect_and_forward_content(output, event);
            }

            event => {
                output.push(event);
            }
        }
    }

    /// Handles the beginning of a heading.
    fn enter_heading(
        &mut self,
        output: &mut Vec<Event<'static>>,
        level: HeadingLevel,
        id: Option<CowStr<'static>>,
        classes: Vec<CowStr<'static>>,
        attrs: Vec<(CowStr<'static>, Option<CowStr<'static>>)>,
    ) {
        self.current_level = level;
        self.provided_id = id.clone();
        self.heading_text.clear();
        self.is_in_heading = true;

        output.push(Event::Start(Tag::Heading {
            level,
            id,
            classes,
            attrs,
        }));
    }

    /// Collects visible text from event for slug generation, and forwards the event.
    fn collect_and_forward_content(
        &mut self,
        output: &mut Vec<Event<'static>>,
        event: Event<'static>,
    ) {
        match &event {
            Event::Text(text) | Event::Code(text) => {
                self.heading_text.push_str(text);
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                let plain = html.replace(['<', '>', '&', ';'], " ");
                self.heading_text.push_str(&plain);
            }
            _ => {}
        }
        output.push(event);
    }

    /// Finalizes the heading and injects <a id="..."> after it if needed.
    fn exit_heading(
        &mut self,
        output: &mut Vec<Event<'static>>,
        level: HeadingLevel,
        check_chinese: bool,
    ) {
        let should_add_id = !check_chinese || contains_chinese(self.heading_text.trim());

        let generated_id = if should_add_id && self.provided_id.is_none() {
            Some(self.generate_unique_id())
        } else {
            self.provided_id.clone()
        };

        output.push(Event::End(TagEnd::Heading(level)));

        if let Some(id) = generated_id {
            let anchor_html = format!(r#"<a id="{}"></a>"#, id);
            output.push(Event::Html(anchor_html.into()));
        }

        self.reset_heading();
    }

    /// Generates a URL-safe, unique ID from collected heading text.
    /// Used as the `id` value in inserted `<a id="...">` anchor elements.
    fn generate_unique_id(&mut self) -> CowStr<'static> {
        let base: String = self
            .heading_text
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .to_lowercase();

        let parts: Vec<_> = base.split('-').filter(|s| !s.is_empty()).collect();
        let base_id = parts.join("-");

        let counter = self.seen_ids.entry(base_id.clone()).or_insert(0);
        let final_id = if *counter == 0 {
            base_id
        } else {
            format!("{}-{}", base_id, *counter)
        };
        *counter += 1;

        CowStr::from(final_id)
    }

    /// Resets internal state after finishing a heading.
    fn reset_heading(&mut self) {
        self.is_in_heading = false;
        self.heading_text.clear();
        self.provided_id = None;
    }
}

impl Default for HeadingProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Processes the entire Markdown string and appends anchor tags (`<a id="..."></a>`)
/// after headings to enable deep linking. Does not modify the original heading tags.
/// Only adds anchors for headings with Chinese characters if `check_chinese` is enabled.
///
/// # Arguments
///
/// * `content` - Mutable reference to the Markdown content (will be overwritten with HTML)
/// * `check_chinese` - If true, only headings containing Chinese characters will get anchors
pub fn add_heading_anchors(content: &mut String, check_chinese: bool) {
    let parser = pulldown_cmark::Parser::new_ext(context, crate::attributes::DEFAULT_PARSER_OPTIONS);
    let mut processor = HeadingProcessor::new();
    let mut events = Vec::new();

    for event in parser {
        // Convert to owned 'static events early
        processor.process_heading_event(event.into_static(), &mut events, check_chinese);
    }

    let mut out = String::new();
    match pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut out) {
        Ok(_) => *content = out,
        Err(e) => log::error!("The tag addition failed. Message: {}", e),
    }
}
