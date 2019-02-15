use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use toml::de::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Manifest {
    pub info: ManifestInfo
}

impl Manifest {
    pub fn from<P>(path: P) -> Result<Self, toml::de::Error> where P: AsRef<Path> {
        let mut file = File::open("foo.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let manifest = toml::from_str(&contents);
        manifest
    }

    pub fn to_file<P>(&self, path: P) where P: AsRef<Path> {
        let mut file = File::create(path).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes()).unwrap();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManifestInfo {
    pub name: String
}