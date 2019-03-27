use crate::package::manifest::Manifest;
use bincode::{deserialize, serialize};

pub mod manifest;

pub static PACKAGE_EXTENSION: &str = "npk";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub manifest: Manifest,
}

impl Package {
    pub fn into_bytes(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        deserialize(bytes)
    }
}
