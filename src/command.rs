use clap::{Arg, Command, command};

/// Parse the command.
///
/// The name `make_app` is really just getting the command parser,
/// but `make_app` is also understandable.
pub fn make_app() -> Command {
    command!()
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Only for mdbook preprocessor"),
        )
        .subcommand(
            Command::new("check")
                .about("Manual do link check"),
        )
}
