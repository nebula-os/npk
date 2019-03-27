use clap::ArgMatches;

pub mod add;
pub mod doctor;
pub mod new;
pub mod pack;

pub trait Command<'a, 'b> {
    fn clap_command() -> clap::App<'a, 'b>;
    fn handle_matches(matches: &ArgMatches);
}
