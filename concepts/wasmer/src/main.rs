extern crate wasmer_runtime;
extern crate wasmer_runtime_core;
extern crate wasmer_wasi;

use wasmer_runtime::{
    cache::{Cache, FileSystemCache, WasmHash},
    default_compiler,
    error::CacheError,
};
use wasmer_runtime_core::{backend::Compiler, Func, Module};

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

    // Save/load from cache
    println!("##--Module Cache--##");
    let module = store_module(module).expect("Can't load the stored WebAssembly module");

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

// Cache
// https://docs.rs/wasmer-runtime/0.4.2/wasmer_runtime/cache/struct.FileSystemCache.html
fn store_module(module: Module) -> Result<Module, CacheError> {
    // Create a new file system cache.
    // This is unsafe because we can't ensure that the artifact wasn't
    // corrupted or tampered with.
    let mut fs_cache = unsafe { FileSystemCache::new("/tmp/wasm_cache")? };
    // Compute a key for a given WebAssembly binary
    let key = WasmHash::generate(&[]);
    // Store a module into the cache given a key
    fs_cache.store(key, module.clone())?;
    Ok(module)
}
