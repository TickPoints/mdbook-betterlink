use log::{debug, info, warn};
use mdbook::book::Book;
use mdbook::config::Config;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use crate::attributes;

pub struct Handler;

impl Handler {
    pub fn new() -> Self {
        Self
    }
}

fn check_version(current_mdbook_version: &attributes::VersionTuple) {
    use std::cmp::Ordering;
    match attributes::DEPENDENT_VERSION.cmp(current_mdbook_version) {
        Ordering::Equal => info!("The current version is the stable dependent version."),
        _ => warn!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            "betterlink",
            attributes::DEPENDENT_VERSION,
            current_mdbook_version
        ),
    }
}

use super::config::{ProcessorConfig, parse_config};
fn get_processor_config(config: &Config) -> ProcessorConfig {
    parse_config(
        config
            .get("preprocessor.betterlink")
            .and_then(|v| v.as_table()),
    )
}

impl Preprocessor for Handler {
    fn name(&self) -> &str {
        "betterlink"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> mdbook::errors::Result<Book> {
        debug!("Betterlink Preprocessor was started.");

        let current_mdbook_version = attributes::VersionTuple::parse_version(&ctx.mdbook_version);
        check_version(&current_mdbook_version);

        let configs = get_processor_config(&ctx.config);

        Ok(super::book_handler::handle(book, configs, ctx))
    }
}
