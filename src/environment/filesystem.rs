use std::fs::create_dir;
use std::path::PathBuf;
use users::os::unix::UserExt;
use users::User;

pub static DEFAULT_PACKAGE_FOLDER: &str = "Packages";
pub static DEFAULT_DATA_FOLDER: &str = "Data";
pub static DEFAULT_APPS_FOLDER: &str = "Apps";

pub fn get_user_home(user: &User) -> PathBuf {
    user.home_dir().to_path_buf()
}

pub fn get_user_packages_directory(user: &User, create: bool) -> PathBuf {
    let packages = get_user_home(user).join(DEFAULT_PACKAGE_FOLDER);

    if create {
        if packages.is_dir() == false {
            if packages.exists() {
                panic!("A file with the packages folder name already exists, please delete or rename it");
            } else {
                create_dir(&packages).unwrap();
            }
        }
    }

    packages
}
