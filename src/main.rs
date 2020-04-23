extern crate clap;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
extern crate bincode;
extern crate gaol;
extern crate lz4;
extern crate nix;
extern crate npk_typescript;
extern crate num_cpus;
extern crate quick_js;
extern crate ring;
extern crate target_lexicon;
extern crate toml;

use clap::{App, Arg, SubCommand};

use crate::commands::add::AddCommand;
use crate::commands::doctor::DoctorCommand;
use crate::commands::new::NewCommand;
use crate::commands::pack::PackCommand;

pub mod commands;
pub mod directory;
pub mod environment;
pub mod package;

const VERSION: &str = "0.0.0";

fn main() {
    use commands::Command;

    // Set the CLI up
    let matches = App::new("Nebula Package Keeper")
        .version(get_version_str())
        .author(get_author())
        .about("The package manager for Nebula OS")
        .subcommand(AddCommand::clap_command())
        .subcommand(NewCommand::clap_command())
        .subcommand(DoctorCommand::clap_command())
        .subcommand(PackCommand::clap_command())
        .get_matches();

    // Process the arguments
    if let Some(matches) = matches.subcommand_matches(AddCommand::clap_command().get_name()) {
        AddCommand::handle_matches(matches);
    }
    if let Some(matches) = matches.subcommand_matches(NewCommand::clap_command().get_name()) {
        NewCommand::handle_matches(matches);
    }
    if let Some(matches) = matches.subcommand_matches(DoctorCommand::clap_command().get_name()) {
        DoctorCommand::handle_matches(matches);
    }
    if let Some(matches) = matches.subcommand_matches(PackCommand::clap_command().get_name()) {
        PackCommand::handle_matches(matches);
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
