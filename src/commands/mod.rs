use clap::ArgMatches;

pub mod add;
pub mod new;

pub trait Command<'a, 'b> {
    fn clap_command() -> clap::App<'a, 'b>;
    fn handle_matches(matches: &ArgMatches);
}