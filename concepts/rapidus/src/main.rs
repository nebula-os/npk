#![feature(try_trait)]

extern crate rapidus;

mod isolate;

static CODE_UNDEFINED: &str = include_str!("../scripts/module_undefined.js");
static CODE_NUMBER_MAX: &str = include_str!("../scripts/module_number.js");
static CODE_ARRAY: &str = include_str!("../scripts/module_array.js");
static CODE_OBJECT: &str = include_str!("../scripts/module_object.js");

fn main() {
    let mut isolate = isolate::Isolate::new();

    // Undefined test
    let undefined_result = isolate.execute_script(CODE_UNDEFINED).unwrap();
    if rapidus::vm::jsvalue::value::Value::undefined() == undefined_result {
        println!("The value is undefined");
    }

    // Number test
    let number_max_result = isolate.execute_script(CODE_NUMBER_MAX).unwrap();
    if let rapidus::vm::jsvalue::value::Value::Number(num) = number_max_result {
        println!("The number is {}", num);
    }

    // Array test
    let array_result = isolate.execute_script(CODE_ARRAY).unwrap();
    let array = array_result.as_array_mut();
    for (i, prop) in array.elems.iter().enumerate() {
        println!("Array value at {} is {}", i, prop.get_data().unwrap().val);
    }

    // Object test
    let object_result = isolate.execute_script(CODE_OBJECT).unwrap();
    let object = object_result.get_object_info();
    for (i, prop) in object.property.iter().enumerate() {
        println!("Object value at {} is {}", prop.0, prop.1.get_data().unwrap().val);
    }

    // Function test
    // Note: At the moment, Rapidus is not able to exchange functions with the isolate
}