use pulldown_cmark::{CowStr, Event, HeadingLevel, Tag, TagEnd};
use std::collections::HashMap;

/// Checks if the text contains Chinese characters.
fn contains_chinese(text: &str) -> bool {
    text.chars()
        .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c) || ('\u{3400}'..='\u{4dbf}').contains(&c))
}

/// Processes Markdown headings to add anchor tags.
pub struct HeadingProcessor {
    in_code_block: bool,
    current_heading_level: HeadingLevel,
    heading_id: Option<CowStr<'static>>,
    heading_text: String,
    is_processing_heading: bool,
    heading_counts: HashMap<String, usize>, // Tracks duplicate headings
}

impl HeadingProcessor {
    /// Creates a new HeadingProcessor with default state.
    pub fn new() -> Self {
        Self {
            in_code_block: false,
            current_heading_level: HeadingLevel::H1,
            heading_id: None,
            heading_text: String::new(),
            is_processing_heading: false,
            heading_counts: HashMap::new(),
        }
    }
}

impl HeadingProcessor {
    /// Generates an ID for the anchor tag, handling duplicates.
    fn generate_anchor_id(&mut self) -> String {
        let base_id = match &self.heading_id {
            Some(id) => id.to_string(),
            None => self
                .heading_text
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>()
                .to_lowercase(),
        };

        let count = self.heading_counts.entry(base_id.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            format!("{}-{}", base_id, *count - 1)
        } else {
            base_id
        }
    }
}

impl HeadingProcessor {
    /// Updates code block state without modifying events.
    fn update_code_block_state(&mut self, event: &Event) {
        match event {
            Event::Start(Tag::CodeBlock(_)) => self.in_code_block = true,
            Event::End(TagEnd::CodeBlock) => self.in_code_block = false,
            _ => {}
        }
    }
}

impl HeadingProcessor {
    /// Handles heading-related events and inserts anchors.
    fn process_heading_event(
        &mut self,
        event: Event<'static>,
        output_events: &mut Vec<Event>,
        check_chinese: bool,
    ) {
        match event {
            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            }) if !self.in_code_block => {
                self.start_heading(level, id, classes, attrs, output_events);
            }
            Event::Text(text) if self.is_processing_heading => {
                self.heading_text.push_str(&text);
                output_events.push(Event::Text(text));
            }
            Event::End(TagEnd::Heading(_)) if self.is_processing_heading => {
                self.finalize_heading(output_events, check_chinese);
            }
            _ => {
                self.update_code_block_state(&event);
                output_events.push(event);
            }
        }
    }

    /// Handles the start of a heading.
    fn start_heading(
        &mut self,
        level: HeadingLevel,
        id: Option<CowStr<'static>>,
        classes: Vec<CowStr<'static>>,
        attrs: Vec<(CowStr<'static>, Option<CowStr<'static>>)>,
        output_events: &mut Vec<Event>,
    ) {
        self.current_heading_level = level;
        self.heading_id = id.clone();
        self.is_processing_heading = true;
        output_events.push(Event::Start(Tag::Heading {
            level,
            id,
            classes,
            attrs,
        }));
    }

    /// Finalizes the heading and inserts anchor if needed.
    fn finalize_heading(&mut self, output_events: &mut Vec<Event>, check_chinese: bool) {
        if !check_chinese || contains_chinese(&self.heading_text) {
            let anchor_id = self.generate_anchor_id();
            output_events.push(Event::Html(format!("<a id=\"{}\"></a>", anchor_id).into()));
        }

        output_events.push(Event::End(TagEnd::Heading(self.current_heading_level)));
        self.reset_heading_state();
    }

    /// Resets heading processing state.
    fn reset_heading_state(&mut self) {
        self.is_processing_heading = false;
        self.heading_text.clear();
        self.heading_id = None;
    }
}

impl Default for HeadingProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Processes Markdown content to add heading anchors.
pub fn add_heading_anchors(content: &mut String, check_chinese: bool) {
    let parser =
        pulldown_cmark::Parser::new_ext(content, crate::attributes::DEFAULT_PARSER_OPTIONS);
    let mut processor = HeadingProcessor::new();
    let mut processed_events = Vec::new();

    for event in parser {
        processor.process_heading_event(event.into_static(), &mut processed_events, check_chinese);
    }

    let _ = pulldown_cmark_to_cmark::cmark(processed_events.into_iter(), content);
}
