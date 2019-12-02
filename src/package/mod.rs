use crate::package::manifest::Manifest;
use anyhow::{Result, Context};
use bincode::{deserialize, serialize, Error};

pub mod manifest;

pub static PACKAGE_EXTENSION: &str = "npk";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub manifest: Manifest,
}

impl Package {
    pub fn into_bytes(&self) -> Result<Vec<u8>> {
        serialize(self).with_context(|| "Cannot serialize the package")
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        deserialize(bytes).with_context(|| "Cannot serialize the package")
    }
}
