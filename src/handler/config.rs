use toml::value::Table;

pub fn parse_config(raw_table: Option<&Table>) -> ProcessorConfig {
    if raw_table.is_none() {
        return ProcessorConfig::default();
    }
    ProcessorConfig::parse(raw_table.unwrap())
}

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
        }
    }
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            add_link_for_chinese: false,
            display_processed_contexts: true,
            do_link_check: true,
        }
    }
}
