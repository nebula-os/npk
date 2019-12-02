#![feature(with_options)]

extern crate wasmtime;
extern crate wasmtime_wasi;
extern crate wasmtime_interface_types;
extern crate anyhow;
extern crate wat;

use anyhow::{Context, Result, format_err, bail};
use std::cell::{RefCell, Ref};
use std::fs::{read, Permissions};
use std::rc::Rc;
use std::collections::HashMap;
use std::path::{Path, PathBuf, Component};
use wasmtime_wasi::create_wasi_instance;
use wasmtime_interface_types::{ModuleData, Value};
use std::borrow::Borrow;
use std::ffi::{OsStr, CString};
use wasmtime::{HostRef, Instance, Module, Store, Engine};

type FnRef = usize;

fn main() -> Result<()> {
    let wasm = wat::parse_file("wasi-test/target/wasm32-wasi/release/wasi_test.wasm")?;

    // Instantiate engine and store.
    let engine = HostRef::new(Engine::default());
    let store = HostRef::new(Store::new(&engine));

    // Module registry
    let mut module_registry = HashMap::new();

    // Load module data
    let module_data = ModuleData::new(&wasm)?;

    // Load wasi
    let mut dir = preopen_dir(".")?;
    let env = compute_argv("wasi-test/target/wasm32-wasi/release/wasi_test.wasm", &[]);
    let wasi = HostRef::new(create_wasi_instance(&store, &[(".".to_owned(), dir)], &env, &[])?);
    module_registry.insert("wasi_unstable".to_owned(), wasi);

    // Load the module
    println!("Instantiating a module...");
    let (instance, module, _) = instantiate_module(&store, &module_registry, wasm.clone())
        .with_context(|| format!("Failed to process the module"))?;

    // Module functions
    println!("Exports:");
    module.borrow().exports()
        .iter()
        .enumerate()
        .for_each(|(index, export)| {
            println!("{:#?} @ {}", export, index);
        });

    // Functions
    let mul_result = run("mul", &[Value::I32(2i32), Value::I32(6i32)], &module_data, &instance)?;
    println!("Mul(2, 6) = {:?}", mul_result);

    // Double
    let double_result = run("double", &[Value::I32(12i32)], &module_data, &instance)?;
    println!("Double(12) = {:?}", double_result);

    // Main
    let main_result = run("main", &[], &module_data, &instance)?;
    println!("Main() = {:?}", main_result);

    Ok(())
}

fn compute_argv(argv0: &str, arg_arg: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    // Add argv[0], which is the program name. Only include the base name of the
    // main wasm module, to avoid leaking path information.
    result.push(
        Path::new(argv0)
            .components()
            .next_back()
            .map(Component::as_os_str)
            .and_then(OsStr::to_str)
            .unwrap_or("")
            .to_owned(),
    );

    // Add the remaining arguments.
    for arg in arg_arg {
        result.push(arg.to_owned());
    }

    println!("Resulting env: {:#?}", result);

    result
}

fn preopen_dir<P: AsRef<Path>>(path: P) -> Result<std::fs::File> {
    std::fs::OpenOptions::new().read(true).create_new(true).append(true).open(path.as_ref()).with_context(|| format!("Failed to pre-open a directory {:?}", path.as_ref().to_str().unwrap()))
}

fn run<S>(name: S, params: &[Value], data: &ModuleData, instance: &HostRef<Instance>) -> Result<Vec<Value>> where S: Into<String> + Copy {
    let results = data
        .invoke_export(instance, &name.into(), &params)
        .with_context(|| format!("Failed to invoke `{}`", name.into()))?;

    Ok(results)
}

fn instantiate_module(
    store: &HostRef<Store>,
    module_registry: &HashMap<String, HostRef<Instance>>,
    wasm: Vec<u8>
) -> Result<(HostRef<Instance>, HostRef<Module>, Vec<u8>)> {
    // Read the wasm module binary either as `*.wat` or a raw binary
    let data = wasm;

    let module = HostRef::new(Module::new(store, &data)?);

    // Resolve import using module_registry.
    let imports = module
        .borrow()
        .imports()
        .iter()
        .map(|i| {
            let module_name = i.module().as_str();
            if let Some(instance) = module_registry.get(module_name) {
                let field_name = i.name().as_str();
                if let Some(export) = instance.borrow().find_export_by_name(field_name) {
                    Ok(export.clone())
                } else {
                    bail!(
                        "Import {} was not found in module {}",
                        field_name,
                        module_name
                    )
                }
            } else {
                bail!("Import module {} was not found", module_name)
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let instance = HostRef::new(Instance::new(store, &module, &imports)?);

    Ok((instance, module, data))
}
