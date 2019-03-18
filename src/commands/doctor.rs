use clap::ArgMatches;

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
            .after_help(
                include_str!("doctor.txt")
            )
            .into()
    }

    fn handle_matches(matches: &ArgMatches) {
        println!("Diagnosing your system...");
    }
}
