use crate::environment::arch::get_cpu_architecture;
use crate::environment::filesystem::{get_user_home, get_user_packages_directory};
use crate::environment::user::{get_effective_group, get_effective_user};
use num_cpus::{get, get_physical};
use std::path::PathBuf;
use target_lexicon::Architecture;
use users::{Group, User};

pub mod arch;
pub mod filesystem;
pub mod user;

#[derive(Debug)]
pub struct Environment {
    pub arch: Architecture,
    pub cpus_physical: i32,
    pub cpus: i32,
    pub user: User,
    pub group: Group,
    pub home_directory: PathBuf,
    pub packages_directory: PathBuf,
}

impl Environment {
    pub fn current() -> Self {
        let arch = get_cpu_architecture();
        let cpus_physical = get_physical() as i32;
        let cpus = get() as i32;
        let user = get_effective_user();
        let group = get_effective_group();
        let home_directory = get_user_home(&user);
        let packages_directory = get_user_packages_directory(&user, false);

        Environment {
            arch,
            cpus,
            cpus_physical,
            user,
            group,
            home_directory,
            packages_directory,
        }
    }
}
