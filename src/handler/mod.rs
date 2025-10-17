pub mod book_handler;
pub mod config;
pub mod processor;

use mdbook::preprocess::Preprocessor;
use std::io;
use std::process;

/// Exits the process with an error status after logging the error.
///
/// # Arguments
/// * `msg` - The error message to log before exiting
fn exit_with_error(msg: &str) -> ! {
    log::error!("{}", msg);
    process::exit(1)
}

/// Main handler function that processes book data from stdin to stdout
pub fn handle() {
    // Read input data
    let (ctx, book): (mdbook::preprocess::PreprocessorContext, mdbook::book::Book) =
        match serde_json::from_reader(io::stdin()) {
            Ok(data) => data,
            Err(e) => exit_with_error(&format!("Failed to read input: {}", e)),
        };

    // Process the book data
    let handler = processor::Handler::new();
    let processed_book = handler.run(&ctx, book.clone()).unwrap_or_else(|e| {
        log::warn!("Processing failed, using original book: {}", e);
        book
    });

    // Write output data
    serde_json::to_writer(io::stdout(), &processed_book)
        .unwrap_or_else(|e| exit_with_error(&format!("Failed to write output: {}", e)));
}
