#[macro_use]
extern crate anyhow;
extern crate quick_js;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use anyhow::Result;
use quick_js::Context;
use serde_derive::{Deserialize, Serialize};

pub static WRAPPER_JS: &str = include_str!("../js/dist/main.umd.js");

pub struct Compiler {
    pub context: Context,
}

impl Compiler {
    pub fn new() -> Result<Self> {
        let mut context = Context::builder().memory_limit(100_000_000).build()?;
        context.eval(WRAPPER_JS)?;

        Ok(Compiler { context })
    }

    pub fn transpile(&mut self, source: String, options: TranspileOptions) -> Result<String> {
        Ok(self.context.eval_as(&format!(
            "tscWrapper.transpile(\"{}\", {}).outputText",
            source,
            serde_json::to_string(&options)?
        ))?)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TranspileOptions {
    #[serde(rename = "compilerOptions")]
    pub compiler_options: Option<CompilerOptions>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CompilerOptions {
    pub target: Option<String>,
    #[serde(rename = "emitDeclarationOnly")]
    pub emit_declarations_only: Option<bool>,
    pub declaration: Option<bool>,
    #[serde(rename = "declarationDir")]
    pub declaration_dir: Option<String>,
    #[serde(rename = "declarationMap")]
    pub declaration_map: Option<bool>,
    pub lib: Option<Vec<String>>,
    pub module: Option<String>,
    #[serde(rename = "moduleResolution")]
    pub module_resolution: Option<String>,
    #[serde(rename = "noLib")]
    pub no_lib: Option<bool>,
    pub jsx: Option<String>,
    #[serde(rename = "jsxFactory")]
    pub jsx_factory: Option<String>,
}
