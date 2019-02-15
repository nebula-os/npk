extern crate clap;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use clap::{App, Arg, SubCommand};

use crate::commands::add::AddCommand;
use crate::commands::new::NewCommand;

pub mod commands;
pub mod directory;
pub mod package;

const VERSION: &str = "0.0.1";

fn main() {
    use commands::Command;

    // Set the CLI up
    let matches = App::new("Nebula Package Keeper")
        .version(get_version_str())
        .author(get_author())
        .about("The package manager for Nebula OS")
        .subcommand(AddCommand::clap_command())
        .subcommand(NewCommand::clap_command())
        .get_matches();

    // Process the arguments
    if let Some(matches) = matches.subcommand_matches(AddCommand::clap_command().get_name()) {
        AddCommand::handle_matches(matches);
    }
    if let Some(matches) = matches.subcommand_matches(NewCommand::clap_command().get_name()) {
        NewCommand::handle_matches(matches);
    }
}

pub fn get_version() -> semver::Version {
    VERSION.parse().unwrap()
}

pub fn get_version_str() -> &'static str {
    VERSION
}

pub fn get_author() -> &'static str {
    "Aleksey <ahalahan@gmail.com> Halahan"
}
