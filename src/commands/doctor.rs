use crate::environment::Environment;
use clap::ArgMatches;
use num_cpus::{get, get_physical};

pub struct DoctorCommand {}

impl DoctorCommand {
    pub fn new() -> Self {
        DoctorCommand {}
    }
}

impl<'a, 'b> crate::commands::Command<'a, 'b> for DoctorCommand {
    fn clap_command() -> clap::App<'a, 'b> {
        clap::SubCommand::with_name("doctor")
            .version(crate::get_version_str())
            .author(crate::get_author())
            .alias("doc")
            .alias("diagnose")
            .about("Diagnose a system for potential problems")
            .after_help(include_str!("doctor.txt"))
            .into()
    }

    fn handle_matches(matches: &ArgMatches) {
        // Get current environment
        let env = Environment::current();

        println!("Architecture: {:?}", env.arch);
        println!("Processors: {} ({} physical)", env.cpus, env.cpus_physical);
        println!(
            "User (Group): {} ({})",
            env.user.name().to_str().unwrap(),
            env.group.name().to_str().unwrap()
        );
        println!("Home: {:?}", env.home_directory);
        println!("Packages: {:?}", env.packages_directory);
    }
}
