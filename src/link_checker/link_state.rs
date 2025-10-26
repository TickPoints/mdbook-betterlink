use super::config::LinkCheckerConfig;
use log;
use pulldown_cmark::{CowStr, LinkType};
use std::path::Path;

/// Tracks the state of a link being processed
#[derive(Clone, Debug)]
pub struct LinkState<'a> {
    active: bool,
    text: String,
    url: CowStr<'a>,
    link_type: LinkType,
}

/// Represents different types of link issues
#[derive(Debug)]
enum LinkIssue {
    Broken,
    InvalidSimple,
    InvalidPath,
    Valid,
}

impl<'a> LinkState<'a> {
    pub fn new() -> Self {
        Self {
            active: false,
            text: String::with_capacity(128),
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
        self.text.clear();
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

    pub fn is_simple(&self) -> bool {
        self.active && matches!(self.link_type, LinkType::Autolink | LinkType::Email)
    }

    pub fn should_check(&self) -> bool {
        self.active
            && (matches!(
                self.link_type,
                LinkType::Inline | LinkType::Autolink | LinkType::Email
            ) || self.is_broken())
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.text.clear();
        self.url = CowStr::Borrowed("");
    }

    /// Determine what kind of issue the link has (if any)
    fn classify_issue(&self, file_path: &Path, root: &Path, conf: &LinkCheckerConfig) -> LinkIssue {
        if !self.active {
            return LinkIssue::Valid;
        }

        match () {
            _ if self.is_broken() => LinkIssue::Broken,
            _ if self.is_simple() => {
                if !super::path_checker::check_url(&self.url) {
                    LinkIssue::InvalidSimple
                } else {
                    LinkIssue::Valid
                }
            }
            _ if !super::path_checker::check_path(&self.url, file_path, root, conf) => {
                LinkIssue::InvalidPath
            }
            _ => LinkIssue::Valid,
        }
    }

    pub fn check_and_prompt(
        &mut self,
        file_path: &Path,
        range: std::ops::Range<usize>,
        root: &Path,
        conf: &LinkCheckerConfig,
    ) -> bool {
        let issue = self.classify_issue(file_path, root, conf);
        let prompt_level = conf.prompt_level;

        let has_issue = match issue {
            LinkIssue::Broken => {
                self.log_issue(
                    file_path,
                    range,
                    prompt_level,
                    "broken",
                    &format!(
                        "[{}] is a broken URL (or path).\nWarn: The behavior is not yet stable",
                        self.text
                    ),
                );
                true
            }
            LinkIssue::InvalidSimple => {
                self.log_issue(
                    file_path,
                    range,
                    prompt_level,
                    "invalid",
                    &format!("<{}> isn't a valid URL.", self.text),
                );
                true
            }
            LinkIssue::InvalidPath => {
                self.log_issue(
                    file_path,
                    range,
                    prompt_level,
                    "invalid",
                    &format!("[{}]({}) isn't a valid URL (or path).", self.text, self.url),
                );
                true
            }
            LinkIssue::Valid => false,
        };

        self.reset();
        has_issue
    }

    /// Helper method to log issues with consistent formatting
    fn log_issue(
        &self,
        file_path: &Path,
        range: std::ops::Range<usize>,
        level: log::Level,
        issue_type: &str,
        message: &str,
    ) {
        log::log!(
            level,
            "[{}][{}][{}] {}",
            file_path.display(),
            super::format_range(&range),
            issue_type,
            message
        );
    }
}

impl<'a> Default for LinkState<'a> {
    fn default() -> Self {
        Self::new()
    }
}
