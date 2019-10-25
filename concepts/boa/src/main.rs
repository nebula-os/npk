extern crate boa;
extern crate gc;

use crate::boa::exec::Executor;
use boa::builtins::value::{Value, ResultValue, ValueData};
use boa::builtins::function::{Function, NativeFunction, NativeFunctionData};
use boa::exec::Interpreter;
use gc::{GcCell};
use std::borrow::BorrowMut;

static CODE_UNDEFINED: &str = include_str!("../scripts/module_undefined.js");
static CODE_NUMBER: &str = include_str!("../scripts/module_number.js");
static CODE_ARRAY: &str = include_str!("../scripts/module_array.js");
static CODE_OBJECT: &str = include_str!("../scripts/module_object.js");
static CODE_FUNCTION: &str = include_str!("../scripts/module_function.js");

fn main() {
    let mut realm = boa::realm::Realm::create();

    // Set some values
    let global = &realm.global_obj;
    global.set_field_slice("print", Value::new(ValueData::Function(Box::new(GcCell::new(Function::NativeFunc(NativeFunction::new(js_print)))))));

    let mut interpreter = boa::exec::Interpreter::new(realm);

    test_undefined(&mut interpreter);
    test_number(&mut interpreter);
    test_array(&mut interpreter);
    test_object(&mut interpreter);
    test_function(&mut interpreter);
}

// Utilities
fn js_print(this: &Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    println!("Printed from JS: \"{}\"", args[0]);

    ResultValue::Ok(Value::new(ValueData::Undefined))
}

// Undefined
fn test_undefined(mut interpreter: &mut Interpreter) {
    let result = boa::forward_val(&mut interpreter, CODE_UNDEFINED);
    let result = result.unwrap();
    if let boa::builtins::value::ValueData::Undefined = &*result {
        println!("Provided value is undefined");
    }
}

// Number
fn test_number(mut interpreter: &mut Interpreter) {
    let result = boa::forward_val(&mut interpreter, CODE_NUMBER);
    let result = result.unwrap();
    if let boa::builtins::value::ValueData::Number(num) = &*result {
        println!("Provided value is {}", num);
    }
}

// Array
fn test_array(mut interpreter: &mut Interpreter) {
    let result = boa::forward_val(&mut interpreter, CODE_ARRAY);
    let result = result.unwrap();
    if let boa::builtins::value::ValueData::Object(arr) = &*result {
        let arr: &boa::builtins::object::Object = &*arr.borrow();
        for (i, prop) in arr.properties.iter().enumerate() {
            println!("Array has value {} at index {}", prop.1.value.as_ref().unwrap(), prop.0);
        }
    }
}

// Object
fn test_object(mut interpreter: &mut Interpreter) {
    let result = boa::forward_val(&mut interpreter, CODE_OBJECT);
    let result = result.unwrap();
    if let boa::builtins::value::ValueData::Object(obj) = &*result {
        let obj: &boa::builtins::object::Object = &*obj.borrow();
        for (i, prop) in obj.properties.iter().enumerate() {
            println!("Object has value {} at index {}", prop.1.value.as_ref().unwrap(), prop.0);
        }
    }
}

// Function
fn test_function(mut interpreter: &mut Interpreter) {
    let result = boa::forward_val(&mut interpreter, CODE_FUNCTION);
    let result = result.unwrap();
}