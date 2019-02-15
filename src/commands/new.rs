use std::fs::DirBuilder;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::package::manifest::Manifest;
use crate::package::manifest::ManifestInfo;

pub struct NewCommand {}

impl NewCommand {
    pub fn new() -> Self {
        NewCommand {}
    }
}

impl<'a, 'b> crate::commands::Command<'a, 'b> for NewCommand {
    fn clap_command() -> clap::App<'a, 'b> {
        clap::SubCommand::with_name("new")
            .version(crate::get_version_str())
            .author(crate::get_author())
            .alias("create")
            .arg(clap::Arg::with_name("name")
                .required(true)
                .help("Name of the package to be created")
                .index(1)
            )
            .arg(clap::Arg::with_name("directory")
                .help("A directory where the package should be initialized")
                .long("directory")
                .short("d")
                .value_name("directory")
            )
            .about("Create a new package.")
            .after_help(
                include_str!("new.txt")
            )
            .into()
    }

    fn handle_matches(matches: &ArgMatches) {
        let name = matches.value_of("name")
            .unwrap()
            .to_owned();

        // Create a directory
        let current_dir = std::env::current_dir().expect("Could not get current directory");
        let directory: PathBuf = matches.value_of("directory")
            .map(|dir| current_dir.join(dir.parse::<PathBuf>()
                .expect("Provided directory path is incorrect")))
            .unwrap_or(current_dir);

        if directory.exists() {
            panic!("Directory \"{}\" already exists", directory.file_name()
                .unwrap()
                .to_str()
                .unwrap());
        }

        DirBuilder::new()
            .create(&directory)
            .unwrap();

        // Create a manifest
        let manifest = Manifest {
            info: ManifestInfo {
                name: name,
                ..Default::default()
            },
            ..Default::default()
        };
        let manifest_path = directory.join("manifest.toml");
        manifest.to_file(manifest_path);
    }
}
