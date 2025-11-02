use pulldown_cmark::{CowStr, Event, HeadingLevel, Tag, TagEnd};
use std::collections::HashMap;

/// Checks if the text contains Chinese characters.
fn contains_chinese(text: &str) -> bool {
    text.chars()
        .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c) || ('\u{3400}'..='\u{4dbf}').contains(&c))
}

/// Processes Markdown headings to add anchor tags via `id` attribute.
pub struct HeadingProcessor {
    in_code_block: bool,
    current_heading_level: HeadingLevel,
    heading_id: Option<CowStr<'static>>,
    heading_text: String,
    is_processing_heading: bool,
    heading_counts: HashMap<String, usize>,
}

impl HeadingProcessor {
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

    /// Generates an ID for the heading.
    fn generate_id(&mut self) -> String {
        let base_id = match &self.heading_id {
            Some(id) => id.to_string(),
            None => self
                .heading_text
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                .collect::<String>()
                .to_lowercase(),
        };

        // Collapse multiple dashes
        let parts = base_id.split('-').filter(|s| !s.is_empty());
        let cleaned = parts.collect::<Vec<_>>().join("-");

        let count = self.heading_counts.entry(cleaned.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            format!("{}-{}", cleaned, *count - 1)
        } else {
            cleaned
        }
    }

    /// Updates code block state.
    fn update_code_block_state(&mut self, event: &Event) {
        match event {
            Event::Start(Tag::CodeBlock(_)) => self.in_code_block = true,
            Event::End(TagEnd::CodeBlock) => self.in_code_block = false,
            _ => {}
        }
    }

    /// Main processing function.
    pub fn process_heading_event(
        &mut self,
        event: Event<'static>,
        output_events: &mut Vec<Event>,
        check_chinese: bool,
    ) {
        match event {
            // Start of heading
            Event::Start(Tag::Heading { level, id, classes, attrs }) if !self.in_code_block => {
                self.current_heading_level = level;
                self.heading_id = id.clone(); // already owned CowStr
                self.is_processing_heading = true;
                self.heading_text.clear();

                // Forward the start tag (we'll modify it later in End)
                output_events.push(Event::Start(Tag::Heading { level, id, classes, attrs }));
            }

            // Collect text content
            Event::Text(text) if self.is_processing_heading => {
                self.heading_text.push_str(&text); // `text: CowStr` → use directly
                output_events.push(Event::Text(text));
            }

            Event::Code(code) if self.is_processing_heading => {
                self.heading_text.push(' ');
                self.heading_text.push_str(&code);
                output_events.push(Event::Code(code));
            }

            Event::Html(html) if self.is_processing_heading => {
                // Simplified plain text extraction
                let plain = html.replace(['<', '>', '&', ';'], " ");
                self.heading_text.push(' ');
                self.heading_text.push_str(&plain);
                output_events.push(Event::Html(html));
            }

            Event::InlineHtml(html) if self.is_processing_heading => {
                let plain = html.replace(['<', '>', '&', ';'], " ");
                self.heading_text.push(' ');
                self.heading_text.push_str(&plain);
                output_events.push(Event::InlineHtml(html));
            }

            // End of heading — now we can inject `id` attribute
            Event::End(TagEnd::Heading(level)) if self.is_processing_heading => {
                let should_add_id = !check_chinese || contains_chinese(&self.heading_text);

                // Only generate id if not already present
                if should_add_id && self.heading_id.is_none() {
                    let generated_id = self.generate_id();
                    let mut attrs = vec![(CowStr::Borrowed("id"), Some(CowStr::from(generated_id)))];

                    // Re-emit the start tag with new attrs (replace last one)
                    if let Some(Event::Start(Tag::Heading {
                        level: prev_level,
                        id: _,
                        classes: prev_classes,
                        attrs: prev_attrs,
                    })) = output_events.last_mut()
                    {
                        // Append new attrs to existing ones
                        let mut all_attrs = prev_attrs.clone();
                        all_attrs.extend(attrs.drain(..));
                        *output_events.last_mut().unwrap() = Event::Start(Tag::Heading {
                            level: *prev_level,
                            id: None,
                            classes: prev_classes.clone(),
                            attrs: all_attrs,
                        });
                    }
                }

                // Emit end tag
                output_events.push(Event::End(TagEnd::Heading(level)));
                self.reset_heading_state();
            }

            // All other events
            _ => {
                self.update_code_block_state(&event);
                output_events.push(event);
            }
        }
    }

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

/// Processes Markdown content to add heading anchors using `id` attribute.
pub fn add_heading_anchors(content: &mut String, check_chinese: bool) {
    let parser = pulldown_cmark::Parser::new(content);
    let mut processor = HeadingProcessor::new();
    let mut events = Vec::new();

    for event in parser {
        processor.process_heading_event(event.into_static(), &mut events, check_chinese);
    }

    let mut out = String::new();
    let _ = pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut out);
    *content = out;
}
