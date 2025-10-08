use std::path::PathBuf;

pub fn check_link(context: &str, path: &Option<PathBuf>, root: &PathBuf) {
    let mut in_code_block = false;

    for (line_index, line) in context.lines().enumerate() {
        if line.starts_with("```") || line.starts_with("~~~") {
            in_code_block = !in_code_block;
            continue;
        }

        if in_code_block {
            continue;
        }

        // Detecting Markdown links within a row
        let mut chars = line.chars().peekable();
        let mut buffer = String::new();
        let mut in_link_text = false;
        let mut in_link_url = false;
        let mut link_text = String::new();
        let mut link_url: String;

        while let Some(c) = chars.next() {
            match c {
                '[' if !in_link_text && !in_link_url && chars.peek() != Some(&'`') => {
                    in_link_text = true;
                    buffer.clear();
                }
                ']' if in_link_text => {
                    in_link_text = false;
                    link_text = buffer.clone();
                    buffer.clear();
                }
                '(' if !in_link_text
                    && !in_link_url
                    && !link_text.is_empty()
                    && chars.peek() != Some(&'`') =>
                {
                    in_link_url = true;
                    buffer.clear();
                }
                ')' if in_link_url => {
                    in_link_url = false;
                    link_url = buffer.clone();

                    // Main process
                    if !link_text.is_empty() && !link_url.is_empty() && path.is_some() {
                        let status = check_path(&link_url, path.as_ref().unwrap(), root);
                        if !status {
                            log::error!(
                                "[{0}][{line_index}] {link_url} isn't a valid URL (or path)",
                                path.as_ref().unwrap().display()
                            );
                        }
                    }

                    link_text.clear();
                    link_url.clear();
                    buffer.clear();
                }
                _ if in_link_text || in_link_url => {
                    buffer.push(c);
                }
                _ => {}
            }
        }
    }
}

pub fn check_path(url: &str, path: &PathBuf, root: &PathBuf) -> bool {
    if check_url(url) {
        return true;
    }
    let realpath = path.join(url);
    if !realpath.is_file() {
        return false;
    }
    if !realpath.starts_with(root) {
        return false;
    }
    return true;
}

pub fn check_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}
