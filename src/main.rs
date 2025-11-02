/// About some constants and type definitions related to constants.
pub mod attributes;
/// About the command definition.
pub mod command;
/// About preprocessor control.
pub mod handler;
/// About controls on type checking.
pub mod link_checker;
/// About test.
#[cfg(test)]
mod tests;

use std::process;

fn init_logger() {
    let mut logger_builder = env_logger::builder();

    if cfg!(debug_assertions) {
        logger_builder.filter_level(log::LevelFilter::Debug);
    } else {
        logger_builder.filter_level(log::LevelFilter::Info);
    }

    // Set format
    logger_builder.format_file(false).format_timestamp_secs();

    logger_builder.init();

    log::debug!("Inited the logger.");

    if cfg!(debug_assertions) {
        log::debug!("Already in debug mode.");
    }
}

fn main() {
    init_logger();

    let args = command::make_app().get_matches();

    if args.subcommand_matches("supports").is_some() {
        // Because it is compatible with most backends,
        // Most of the time, exit with 0.
        process::exit(0);
    } else if args.subcommand_matches("check").is_some() {
        todo!();
    } else {
        handler::handle();
    }
}
