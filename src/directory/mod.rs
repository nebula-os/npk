use std::path::PathBuf;

use crate::package::manifest::Manifest;

#[derive(Debug, Clone)]
pub struct Directory {
    root: PathBuf,
    manifest: Manifest,
}