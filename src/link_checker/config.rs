use log::Level;
use toml::value::Table;

/// It is about the configuration of link checker.
/// We're going to read `preprocessor.betterlink.link_checker` fields in the book's config.
/// For processor or using by command (in the future), it can work.
#[derive(Clone, Debug)]
pub struct LinkCheckerConfig {
    /// Prompt level when bad links are found.
    /// It uses `log::Level` and is based on number parsing.
    /// See [log::Level](https://docs.rs/log/latest/log/enum.Level.html).
    /// Default: `Level::Error` (or 1)
    pub prompt_level: Level,
}

/*
// unused:

fn get_bool_config(table: &Table, key: &str, default: bool) -> bool {
    table.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}
*/

fn get_integer_config(table: &Table, key: &str, default: i64) -> i64 {
    table
        .get(key)
        .and_then(|v| v.as_integer())
        .unwrap_or(default)
}

impl LinkCheckerConfig {
    pub fn parse(raw_table: &Table) -> Self {
        Self {
            prompt_level: Self::parse_log_level(get_integer_config(raw_table, "prompt_level", 1)),
        }
    }

    /// Parse the numeric Level
    ///
    /// Supports:
    /// - `1`: `Level::Error`
    /// - `2`: `Level::Warn`
    /// - `3`: `Level::Info`
    /// - `4`: `Level::Debug` (warn in non-debug mode)
    /// - `5`: `Level::Trace` (warn always)
    /// - Others: Warn and use default level.
    pub fn parse_log_level(level: i64) -> Level {
        match level {
            1 => Level::Error,
            2 => Level::Warn,
            3 => Level::Info,
            #[cfg(debug_assertions)]
            4 => Level::Debug,
            #[cfg(not(debug_assertions))]
            4 => {
                log::warn!("Debug-level prompts will be not visible in non-debug mode.");
                Level::Debug
            }
            5 => {
                log::warn!("Trace-level prompts will be not visible.");
                Level::Trace
            }
            _ => {
                log::error!("Invalid log level! Will use default: Level::Error.");
                Level::Error
            }
        }
    }
}

impl Default for LinkCheckerConfig {
    fn default() -> Self {
        Self {
            prompt_level: Level::Error,
        }
    }
}
