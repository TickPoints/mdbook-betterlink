use clap::{Arg, Command};

/// Parse the command.
///
/// The name `make_app` is really just getting the command parser,
/// but `make_app` is also understandable.
pub fn make_app() -> Command {
    Command::new("mdbook-betterlink")
        .about("Plugin that provides better hyperlink effects for mdbook.")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}
