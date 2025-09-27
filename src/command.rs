use clap::{Arg, Command};

pub fn make_app() -> Command {
    Command::new("mdbook-betterlink")
        .about("Plugin that provides better hyperlink effects for mdbook.")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}
