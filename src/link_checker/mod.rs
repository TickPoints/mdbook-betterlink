use pulldown_cmark::{Event, Tag, TagEnd};
use std::path::{Path, PathBuf};

pub mod config;
pub mod path_checker;
pub mod link_state;

use config::LinkCheckerConfig;
use link_state::LinkState;

/// Formats a range for display purposes
pub fn format_range(range: &std::ops::Range<usize>) -> String {
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
/// * `conf` - Configuration for link checking
///
/// # Notes
/// - Returns early if path is None
/// - Logs issues according to the configured prompt_level
pub fn check_link(context: &str, path: &Option<PathBuf>, root: &Path, conf: &LinkCheckerConfig) {
    // Early return if path is None to avoid unnecessary processing
    let Some(file_path) = path else {
        return;
    };

    let events = pulldown_cmark::Parser::new_ext(context, crate::attributes::DEFAULT_PARSER_OPTIONS);
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
                if link_state.is_broken() {
                    link_state.prompt_broken(file_path, range, conf.prompt_level);
                    link_state.reset();
                    continue;
                }
                if !path_checker::check_path(link_state.url(), file_path, root) {
                    link_state.prompt_valid(file_path, range, conf.prompt_level);
                }
                link_state.reset();
            }
            _ => {}
        }
    }
}
