extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

#[wasm_bindgen]
pub fn main() {
    // File exists
    println!("File exists: {}", std::path::Path::new("./test.txt").exists());

    // Create a file
    println!("Writing to test.txt");
    std::fs::write("./test.txt", b"Lorem ipsum").unwrap();

    // Read and print it's contents
    println!("Reading...");
    let contents = std::fs::read_to_string("./test.txt").unwrap();
    println!("From file: {}", contents);

    // Delete the file
    std::fs::remove_file("./test.txt").unwrap();
}

#[wasm_bindgen]
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen]
pub fn double(a: i32) -> i32 {
    a * 2
}