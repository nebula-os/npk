#![feature(try_trait)]

extern crate rapidus;

mod isolate;

static CODE: &str = include_str!("module.js");

fn main() {
    println!("Loading:\n{}", CODE);
    let mut isolate = isolate::Isolate::new();
    let result = isolate.execute_script(CODE);
    println!("Result: {:#?}", result);

    let exports = isolate.value("exports");
    println!("Exports: {:#?}", exports);
    println!("Exports -> Value: {:#?}", exports.get_property("value"));
}
