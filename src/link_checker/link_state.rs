use pulldown_cmark::{CowStr, LinkType};
use std::path::Path;
use log;

/// Tracks the state of a link being processed
pub struct LinkState<'a> {
    active: bool,
    text: String,
    url: CowStr<'a>,
    link_type: LinkType,
}

impl<'a> LinkState<'a> {
    pub fn new() -> Self {
        Self {
            active: false,
            text: String::with_capacity(128), // Pre-allocate reasonable capacity
            url: CowStr::Borrowed(""),
            link_type: LinkType::Inline,
        }
    }
    
    pub fn url(&self) -> &CowStr<'a> {
        &self.url
    }

    pub fn start_link(&mut self, url: CowStr<'a>, link_type: LinkType) {
        self.active = true;
        self.url = url;
        self.link_type = link_type;
        self.text.clear(); // Reset text when starting new link
    }

    pub fn append_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_broken(&self) -> bool {
        self.active
            && matches!(
                self.link_type,
                LinkType::ShortcutUnknown | LinkType::CollapsedUnknown | LinkType::ReferenceUnknown
            )
    }

    pub fn should_check(&self) -> bool {
        self.active && (matches!(self.link_type, LinkType::Inline) || self.is_broken())
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.text.clear();
        self.url = CowStr::Borrowed("");
    }

    pub fn prompt_valid(
        &self,
        file_path: &Path,
        range: std::ops::Range<usize>,
        prompt_level: log::Level,
    ) {
        log::log!(
            prompt_level,
            "[{}][{}] [{}]({}) isn't a valid URL (or path)",
            file_path.display(),
            super::format_range(&range),
            self.text,
            self.url
        );
    }

    pub fn prompt_broken(
        &self,
        file_path: &Path,
        range: std::ops::Range<usize>,
        prompt_level: log::Level,
    ) {
        log::log!(
            prompt_level,
            "[{}][{}] [{}] isn't a broken URL (or path)",
            file_path.display(),
            super::format_range(&range),
            self.text
        );
    }
}
