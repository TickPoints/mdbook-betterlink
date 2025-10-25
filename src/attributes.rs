use pulldown_cmark::Options;
use std::fmt;

/// A simple version tuple.
/// Contains parsing and generation capabilities.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct VersionTuple(usize, usize, usize);

impl VersionTuple {
    /// Generate a (0,0,0) version tuple.
    pub fn empty() -> Self {
        Self(0, 0, 0)
    }

    /// Parse version number text such as 0,0,1 or 0.0.1.
    pub fn parse_version(version: &str) -> Self {
        version
            .chars()
            .filter(|&c| c.is_ascii_digit() || matches!(c, '.' | ','))
            .collect::<String>()
            .split(&['.', ','][..])
            .filter_map(|s| s.parse::<usize>().ok())
            .take(3)
            .collect::<Vec<_>>()
            .try_into()
            .map(|arr: [usize; 3]| arr.into())
            .unwrap_or_else(|_| [0, 0, 0].into())
    }

    /// Display the version through a format to ({}, {}, {}).
    /// The behavior is the same as what `fmt::Display` does.
    pub fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl From<VersionTuple> for (usize, usize, usize) {
    fn from(v: VersionTuple) -> Self {
        (v.0, v.1, v.2)
    }
}

impl From<[usize; 3]> for VersionTuple {
    fn from(v: [usize; 3]) -> Self {
        VersionTuple(v[0], v[1], v[2])
    }
}

impl From<&str> for VersionTuple {
    fn from(v: &str) -> Self {
        VersionTuple::parse_version(v)
    }
}

impl fmt::Display for VersionTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

/// Constant: The version of the mdbook on which the build (/development) depends.
pub const DEPENDENT_VERSION: VersionTuple = VersionTuple(0, 4, 52);

/// About the default parser config of pulldown_cmark in the crate.
///
/// Supports:
/// - GitHub-compatible footnote syntax.
/// - TeX formulas. (`$` type)
/// - Blockquote tags (`[!NOTE]`, `[!TIP]`, `[!IMPORTANT]`, `[!WARNING]`, `[!CAUTION]`).
pub const DEFAULT_PARSER_OPTIONS: Options = Options::from_bits_truncate(
    Options::ENABLE_FOOTNOTES.bits() |
    Options::ENABLE_MATH.bits() |
    Options::ENABLE_GFM.bits()
);
