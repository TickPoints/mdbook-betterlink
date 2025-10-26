use std::path::Path;

/// Checks if a URL/path is valid.
///
/// Returns true if:
/// - The URL is a valid absolute URL, or
/// - The path exists as a file within the root directory
pub fn check_path(
    url: &str,
    path: &Path,
    root: &Path,
    conf: &super::config::LinkCheckerConfig,
) -> bool {
    if conf.black_list.contains(url) {
        return false;
    }
    check_url(url)
        || is_valid_link_target(url, path, root).unwrap_or_else(|| {
            log::warn!(
                "The path checker has an internal error that may cause the decision to fail."
            );
            false
        })
}

/// Checks if a string is a valid URL
pub fn check_url(path: &str) -> bool {
    url::Url::parse(path).is_ok()
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
pub fn is_valid_link_target(target: &str, base_path: &Path, root: &Path) -> Option<bool> {
    if target.starts_with('#') {
        return Some(true); // Fragments are always considered valid
    }

    // Split off fragment and query parts
    let path_part = match target.split(['#', '?']).next() {
        Some("") => return Some(true), // Case where only fragment exists (e.g., "#title")
        Some(part) => part,
        None => return None,
    };

    // Handle the path portion
    let full_path = if let Some(relative_path) = path_part.strip_prefix('/') {
        root.join(relative_path)
    } else {
        let base_dir = root.join(base_path.parent()?);
        let joined_path = base_dir.join(path_part);
        match joined_path.canonicalize() {
            Ok(canonicalized_path) => canonicalized_path,
            Err(_) => return Some(false),
        }
    };

    // Check if the path exists and is within the root directory
    Some(full_path.exists() && full_path.starts_with(root))
}
