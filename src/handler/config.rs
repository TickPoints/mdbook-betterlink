use crate::link_checker::config::LinkCheckerConfig;
use toml::value::Table;

/// Parsing config.
/// The main parsing work will be divided between `default` and `parse`.
pub fn parse_config(raw_table: Option<&Table>) -> ProcessorConfig {
    if raw_table.is_none() {
        return ProcessorConfig::default();
    }
    ProcessorConfig::parse(raw_table.unwrap())
}

/// We're going to read `preprocessor.betterlink` fields in the book's config.
/// (This behavior occurs before the `handle` function is officially processed.)
/// Once obtained, the following more readable form can be obtained by parsing through the associated method.
#[derive(Clone, Debug)]
pub struct ProcessorConfig {
    /// Add link for Chinese only,
    /// avoid adding unnecessary links to English.
    /// Default: false
    pub add_link_for_chinese: bool,
    /// Debug-compiled programs display the processed contexts during use.
    /// Default: true
    pub display_processed_contexts: bool,
    /// Check links in the processing phase.
    /// Default: true
    pub do_link_check: bool,
    /// Use old `add_a_tag`.
    /// This is because the latest version of the work is still unstable.
    /// Default: true (pre: false)
    pub use_old_tag_adder: bool,

    /// Link Checker Config
    /// Default: ...
    pub link_checker_config: LinkCheckerConfig,
}

fn get_bool_config(table: &Table, key: &str, default: bool) -> bool {
    table.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

impl ProcessorConfig {
    pub fn parse(raw_table: &Table) -> Self {
        Self {
            add_link_for_chinese: get_bool_config(raw_table, "add_link_for_chinese", false),
            display_processed_contexts: get_bool_config(
                raw_table,
                "display_processed_contexts",
                true,
            ),
            do_link_check: get_bool_config(raw_table, "do_link_check", true),
            #[cfg(debug_assertions)]
            use_old_tag_adder: get_bool_config(raw_table, "use_old_tag_adder", false),
            #[cfg(not(debug_assertions))]
            use_old_tag_adder: get_bool_config(raw_table, "use_old_tag_adder", true),
            link_checker_config: raw_table
                .get("link_checker")
                .and_then(|v| v.as_table())
                .map(LinkCheckerConfig::parse)
                .unwrap_or_default(),
        }
    }
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            add_link_for_chinese: false,
            display_processed_contexts: true,
            do_link_check: true,
            #[cfg(debug_assertions)]
            use_old_tag_adder: false,
            #[cfg(not(debug_assertions))]
            use_old_tag_adder: true,
            link_checker_config: LinkCheckerConfig::default(),
        }
    }
}
