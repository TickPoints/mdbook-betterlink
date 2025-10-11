pub mod attributes;
mod command;
mod handler;
pub mod link_checker;

use std::process;

fn init_logger() {
    let mut logger_builder = env_logger::builder();

    if cfg!(debug_assertions) {
        logger_builder.filter_level(log::LevelFilter::Debug);
        log::error!("Already in debug mode.");
    } else {
        logger_builder.filter_level(log::LevelFilter::Info);
    }
    
    // Set format
    logger
        .format_file(false)
        .format_timestamp_secs();

    logger_builder.init();

    log::debug!("Inited the logger.");
}

fn main() {
    init_logger();

    let args = command::make_app().get_matches();

    if args.subcommand_matches("supports").is_some() {
        // Because it is compatible with most backends,
        // Most of the time, exit with 0.
        process::exit(0);
    } else {
        handler::handle();
    }
}
