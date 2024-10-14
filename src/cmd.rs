use clap::{Arg, ArgAction, Command};

pub fn get_args() -> Command {
    Command::new("pobsd-server")
        .about("playonbsd alternative website")
        .version("0.1.0")
        .arg(
            Arg::new("config")
                .action(ArgAction::Set)
                .required(true)
                .long("config")
                .short('c')
                .help("path to the configuration file"),
        )
}
