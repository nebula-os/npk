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
    pub compilerOptions: Option<CompilerOptions>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CompilerOptions {
    pub target: Option<String>,
}
