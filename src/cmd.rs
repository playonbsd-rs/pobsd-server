use clap::{Arg, ArgAction, Command};

pub fn get_args() -> Command {
    Command::new("pobsd-server")
        .about("playonbsd alternative website")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("config")
                .action(ArgAction::Set)
                .required(true)
                .long("config")
                .short('c')
                .help("path to the configuration file"),
        )
}
