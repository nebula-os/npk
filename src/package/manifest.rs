use anyhow::{Context, Result};
use bincode::{deserialize, serialize};
use quick_js::{Context as ScriptContext, JsValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, RwLock};

pub static MANIFEST_DEFAULT_FILE: &str = "manifest.ts";
pub static MANIFEST_DEFAULT_DEFINITION_FILE: &str = "typings.d.ts";
pub static MANIFEST_TYPESCRIPT_DEFINITION: &str = include_str!("./manifest.d.ts");

pub fn manifest_definition_to_file<P: AsRef<Path>>(path: P) -> Result<()> {
    std::fs::write(
        path,
        MANIFEST_TYPESCRIPT_DEFINITION
            .to_owned()
            .clone()
            .into_bytes(),
    )?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub source: String,
}

impl Manifest {
    pub fn into_bytes(&self) -> Result<Vec<u8>> {
        serialize(self).with_context(|| "Cannot serialize the manifest")
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        deserialize(bytes).with_context(|| "Cannot deserialize the manifest")
    }
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Self> {
        let source = std::fs::read_to_string(path)?;

        Ok(Manifest { source })
    }
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        std::fs::write(path, self.source.clone().into_bytes())?;
        Ok(())
    }
    pub fn evaluate(&self) -> Result<ManifestEvaluated> {
        let context = ScriptContext::builder().memory_limit(100_000).build()?;

        context.eval(&self.source)?;
        let result = if let JsValue::Object(obj) = context
            .eval("manifest()")
            .with_context(|| "Error calling the \"manifest()\" function")?
        {
            Ok(obj)
        } else {
            Err(anyhow!(
                "Calling the \"manifest()\" didn't return an object"
            ))
        }?;
        let manifest_name = match result
            .get("name")
            .with_context(|| "Couldn't find \"name\" in the manifest")?
        {
            JsValue::String(name) => Ok(name.clone()),
            _ => Err(anyhow!("Manifest field \"name\" is not a string")),
        }?;
        let manifest_version = match result
            .get("version")
            .with_context(|| "Couldn't find \"version\" in the manifest")?
        {
            JsValue::String(version) => Ok(version.clone()),
            _ => Err(anyhow!("Manifest field \"version\" is not a string")),
        }?;
        Ok(ManifestEvaluated {
            name: manifest_name,
            version: manifest_version,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestEvaluated {
    pub name: String,
    pub version: String,
}
