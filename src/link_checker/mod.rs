use pulldown_cmark::{CowStr, Event, LinkType, Tag, TagEnd};
use std::path::{Path, PathBuf};

pub mod config;
mod path_checker;

fn format_range(range: &std::ops::Range<usize>) -> String {
    if range.is_empty() {
        range.start.to_string()
    } else {
        format!("{}..{}", range.start, range.end)
    }
}

/// Checks markdown content for invalid links.
///
/// # Arguments
/// * `context` - The markdown content to check
/// * `path` - Optional path to the markdown file (used for relative path resolution)
/// * `root` - Root directory that all links must be contained within (It's the `src/` of the current book)
pub fn check_link(context: &str, path: &Option<PathBuf>, root: &Path) {
    // Early return if path is None to avoid unnecessary processing
    let Some(file_path) = path else {
        return;
    };

    let events =
        pulldown_cmark::Parser::new_ext(context, crate::attributes::DEFAULT_PARSER_OPTIONS);
    let mut link_state = LinkState::new();

    for (event, range) in events.into_offset_iter() {
        match event {
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                ..
            }) => {
                link_state.start_link(dest_url, link_type);
            }
            Event::Text(text) if link_state.is_active() => {
                link_state.append_text(&text);
            }
            Event::End(TagEnd::Link) if link_state.should_check() => {
                if !path_checker::check_path(&link_state.url, file_path, root) {
                    log::error!(
                        "[{}][{}] [{}]({}) isn't a valid URL (or path)",
                        file_path.display(),
                        format_range(&range),
                        link_state.text,
                        link_state.url
                    );
                }
                link_state.reset();
            }
            _ => {}
        }
    }
}

/// Tracks the state of a link being processed
struct LinkState<'a> {
    active: bool,
    text: String,
    url: CowStr<'a>,
    link_type: LinkType,
}

impl<'a> LinkState<'a> {
    fn new() -> Self {
        Self {
            active: false,
            text: String::new(),
            url: CowStr::Borrowed(""),
            link_type: LinkType::Inline,
        }
    }

    fn start_link(&mut self, url: CowStr<'a>, link_type: LinkType) {
        self.active = true;
        self.url = url;
        self.link_type = link_type;
    }

    fn append_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn should_check(&self) -> bool {
        self.active && matches!(self.link_type, LinkType::Inline)
    }

    fn reset(&mut self) {
        self.active = false;
        self.text.clear();
        self.url = CowStr::Borrowed("");
    }
}
