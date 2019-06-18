extern crate wasmer_runtime;
extern crate wasmer_runtime_core;
extern crate wasmer_wasi;

use wasmer_runtime::default_compiler;
use wasmer_runtime_core::{backend::Compiler, Func};

static WASM: &'static [u8] = include_bytes!("./../wasi-hello-world/main.wasm");

fn main() {
    let module = wasmer_runtime_core::compile_with(WASM, &default_compiler())
        .expect("WASM can't be compiled");

    // Show some module diagnostic info
    let module_info = module.info();
    println!("##--Module Exports--##");
    for export in module_info.exports.keys() {
        println!("{}", export);
    }

    let import_object =
        wasmer_wasi::generate_import_object(vec![], vec![], vec![".".to_string()], vec![]);
    let instance = module
        .instantiate(&import_object)
        .map_err(|err| format!("Can't instantiate the WebAssembly module: {:?}", err))
        .unwrap();

    let start: Func<(), ()> = instance
        .func("_start")
        .map_err(|e| format!("{:?}", e))
        .expect("start function in wasi module");

    println!("##--Execution--##");
    start.call().expect("Can't execute WebAssembly");
}
