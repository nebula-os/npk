use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use semver::{Version, VersionReq};
use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use std::iter::Map;
use toml::de::Error;

pub static MANIFEST_DEFAULT_FILE: &str = "manifest.toml";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest {
    pub info: ManifestInfo,
    pub dependencies: Option<BTreeMap<String, ManifestDependency>>,
    pub host_dependencies: Option<BTreeMap<String, ManifestDependency>>,
    pub sources: Option<BTreeMap<String, ManifestSource>>,
}

impl Manifest {
    pub fn from<P>(path: P) -> Result<Self, toml::de::Error>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let manifest = toml::from_str(&contents);

        // Do the validation
        manifest.map(|manifest: Manifest| {
            Manifest::validate(&manifest);
            manifest
        })
    }

    pub fn to_file<P>(&self, path: P, validate: bool)
    where
        P: AsRef<Path>,
    {
        // Validate the values
        if validate {
            self.validate();
        }

        let mut file = File::create(path).unwrap();
        file.write_all(format!("{}", toml::Value::try_from(self).unwrap()).as_bytes())
            .unwrap();
    }

    pub fn validate(&self) {
        // Package name
        if self.info.name.len() < 1 {
            panic!("Name of the package should be at least 1 character long");
        }

        // Package version
        self.info
            .version
            .parse::<Version>()
            .expect("Package version is invalid");

        // Package dependencies
        if let Some(dependencies) = &self.dependencies {
            dependencies.values().for_each(|value| {
                value.validate();
            });
        };

        // Package host dependencies
        if let Some(dependencies) = &self.host_dependencies {
            dependencies.values().for_each(|value| {
                value.validate();
            });
        };
    }
}

// Info
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManifestInfo {
    pub name: String,
    pub version: String,
    pub arch: Vec<String>,
}

// Dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManifestDependency {
    Short(String),
    Long { version: String },
}

impl ManifestDependency {
    pub fn validate(&self) {
        match self {
            ManifestDependency::Short(version) => version
                .parse::<VersionReq>()
                .expect(&format!("Cannot parse version requirement \"{}\"", version)),
            ManifestDependency::Long { version } => version
                .parse::<VersionReq>()
                .expect(&format!("Cannot parse version requirement \"{}\"", version)),
        };
    }
}

impl Default for ManifestDependency {
    fn default() -> Self {
        ManifestDependency::Short(Default::default())
    }
}

// Source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManifestSource {
    Url(String),
    Detailed { url: String },
}
