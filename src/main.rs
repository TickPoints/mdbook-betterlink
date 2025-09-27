pub mod attributes;
mod command;
mod handler;

use std::process;

fn init_logger() {
    let mut logger_builder = env_logger::builder();

    if cfg!(debug_assertions) {
        logger_builder.filter_level(log::LevelFilter::Debug);
    } else {
        logger_builder.filter_level(log::LevelFilter::Info);
    }

    logger_builder.init();

    log::debug!("Inited the logger.");
}

fn main() {
    init_logger();

    let args = command::make_app().get_matches();

    if let Some(_) = args.subcommand_matches("supports") {
        // Because tt is compatible with most backends,
        // Most of the time, exit with 0.
        process::exit(0);
    } else {
        handler::handle();
    }
}
