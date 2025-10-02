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
}

impl ProcessorConfig {
    pub fn parse(raw_table: &Table) -> Self {
        Self {
            add_link_for_chinese: raw_table
                .get("add_link_for_chinese")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            add_link_for_chinese: false,
        }
    }
}
