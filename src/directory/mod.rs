use std::path::PathBuf;

use crate::package::manifest::{Manifest, MANIFEST_DEFAULT_FILE};

#[derive(Debug, Clone)]
pub struct Directory {
    pub root: PathBuf,
    pub manifest: Manifest,
}

impl Directory {
    pub fn at_dir(root: &PathBuf) -> Option<Self> {
        let path = root.clone();

        // Check if the given path is a directory
        if path.is_dir() == false {
            panic!("Given path is not a directory");
        }

        // Check if the manifest file exists
        let manifest_path = path.join(MANIFEST_DEFAULT_FILE);
        if manifest_path.exists() && manifest_path.is_file() {
            let manifest = Manifest::from(&manifest_path)
                .expect(&format!("Couldn't parse \"{}\"", MANIFEST_DEFAULT_FILE));

            Some(Directory {
                root: path.clone(),
                manifest,
            })
        } else {
            None
        }
    }

    pub fn at(path: &PathBuf) -> Option<Self> {
        let path = path.clone();
        let dir: Option<PathBuf> = if path.is_dir() {
            Some(path)
        } else if path.file_name().unwrap() == MANIFEST_DEFAULT_FILE {
            Some(path.parent().unwrap().to_path_buf())
        } else {
            None
        };

        if let Some(dir) = &dir {
            Self::at_dir(&dir)
        } else {
            None
        }
    }
}
