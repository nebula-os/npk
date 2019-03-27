use crate::directory::Directory;
use crate::package::manifest::MANIFEST_DEFAULT_FILE;
use crate::package::PACKAGE_EXTENSION;
use clap::ArgMatches;
use std::path::{Path, PathBuf};
use std::thread::current;

pub struct PackCommand {}

impl PackCommand {
    pub fn new() -> Self {
        PackCommand {}
    }
}

impl<'a, 'b> crate::commands::Command<'a, 'b> for PackCommand {
    fn clap_command() -> clap::App<'a, 'b> {
        clap::SubCommand::with_name("pack")
            .version(crate::get_version_str())
            .author(crate::get_author())
            .alias("package")
            .arg(clap::Arg::with_name("out")
                .help("Directory, path or name for the resulting package")
                .short("o")
                .long("out")
                .value_name("out")
            )
            .arg(clap::Arg::with_name("manifest")
                .help("Directory, path or name of the manifest")
                .short("m")
                .long("manifest")
                .value_name("manifest")
            )
            .about("Pack a Nebula package. Downloads the sources, generates hashes, builds, makes a package file.")
            .after_help(
                include_str!("pack.txt")
            )
            .into()
    }

    fn handle_matches(matches: &ArgMatches) {
        let current_dir: PathBuf =
            std::env::current_dir().expect("Could not get current directory");

        // Get a manifest directory
        let manifest_path: PathBuf = matches
            .value_of("manifest")
            .map(|v| {
                v.parse()
                    .expect("Provided manifest path has incorrect format")
            })
            .unwrap_or(current_dir.clone());
        let package_directory = Directory::at(&manifest_path).expect(&format!(
            "Cannot find \"{}\" at {:?}",
            MANIFEST_DEFAULT_FILE, manifest_path
        ));
        let name = &package_directory.manifest.info.name;
        let package_file = format!("{}.{}", name, PACKAGE_EXTENSION);

        // Get a package path
        let mut path: PathBuf = matches
            .value_of("out")
            .map(|out| {
                let out_path: PathBuf = out
                    .parse::<PathBuf>()
                    .expect("Provided package path is incorrect");

                if out_path.is_dir() {
                    current_dir.join(package_file.clone())
                } else if out_path
                    .extension()
                    .expect("Path doesn't have an extension")
                    == PACKAGE_EXTENSION
                {
                    out_path
                } else {
                    panic!("Provided package path is incorrect");
                }
            })
            .unwrap_or(package_directory.root.join(package_file.clone()));

        // Display some data
        println!(
            "Calling pack on manifest {:#?} with out {:?}",
            package_directory.manifest, path
        );
    }
}
