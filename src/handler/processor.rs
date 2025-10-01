use log::{debug, info, warn};
use mdbook::book::Book;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use crate::attributes;

pub struct Handler;

impl Handler {
    pub fn new() -> Self {
        Self
    }
}

fn parse_mdbook_version(version: &str) -> attributes::VersionTuple {
    let mut version_strings = [const { String::new() }; 3];
    let mut version_list = [0usize; 3];

    let mut target = 0;
    for c in version.chars() {
        match c {
            'v' | ' ' | '\n' => continue,
            ',' => {
                version_list[target] = version_strings[target].parse().unwrap_or(0);
                target += 1;
            }
            _ => version_strings[target].push(c),
        }
    }

    version_list.into()
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

impl Preprocessor for Handler {
    fn name(&self) -> &str {
        "betterlink"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> mdbook::errors::Result<Book> {
        debug!("Betterlink Preprocessor was started.");

        let current_mdbook_version = parse_mdbook_version(&ctx.mdbook_version);
        check_version(&current_mdbook_version);

        Ok(super::book_handler::handle(book))
    }
}
