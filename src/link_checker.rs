use pulldown_cmark::{CowStr, Event, LinkType, Tag, TagEnd};
use std::path::{Path, PathBuf};

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
/// * `root` - Root directory that all links must be contained within
pub fn check_link(context: &str, path: &Option<PathBuf>, root: &Path) {
    // Early return if path is None to avoid unnecessary processing
    let Some(file_path) = path else {
        return;
    };

    let events = pulldown_cmark::Parser::new_ext(context, pulldown_cmark::Options::all());
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
                if !check_path(&link_state.url, file_path, root) {
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

/// Checks if a URL/path is valid.
///
/// Returns true if:
/// - The URL is a valid absolute URL, or
/// - The path exists as a file within the root directory
pub fn check_path(url: &str, path: &Path, root: &Path) -> bool {
    check_url(url) || is_valid_link_target(url, path, root)
}

/// Checks if a string is a valid URL
pub fn check_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

/// Checks if a path is a valid relative path within the root directory.
/// Supports:
/// - Root-relative paths (e.g., "/a/b/c.md" where "/" maps to `root`)
/// - Regular relative paths (e.g., "subdir/file.md")
/// - Current directory relative paths (e.g., "./file.md")
/// - Parent directory relative paths (e.g., "../sibling/file.md")
///
/// There are still some problems related to the title.
/// For links that contain a title, the title portion is ignored now.
/// We may improve later.
///
/// Title Supports:
/// - Pure title (e.g. "#title")
/// - Combined-type title (e.g. "./a.md#title")
///
/// **The function behavior is still unstable.**
pub fn is_valid_link_target(target: &str, base_path: &Path, root: &Path) -> bool {
    if target.starts_with('#') {
        return true;        // Fragments are always considered valid
    }

    // Split off fragment and query parts
    let path_part = match target.split(['#', '?']).next() {
        Some("") => return true,        // Case where only fragment exists (e.g., "#title")
        Some(part) => part,
        None => return false,
    };

    // Handle the path portion
    let full_path = if let Some(relative_path) = path_part.strip_prefix('/') {
        root.join(relative_path)
    } else {
        let base_dir = base_path.parent().unwrap_or(base_path);
        let joined_path = base_dir.join(path_part);
        log::debug!("base_path: {base_path:?}\njoined_path: {joined_path:?}");
        joined_path.canonicalize().unwrap_or(joined_path)
    };

    // Check if the path exists and is within the root directory
    full_path.exists() && full_path.starts_with(root)
}
