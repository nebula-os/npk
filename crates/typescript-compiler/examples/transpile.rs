extern crate typescript_compiler;

use typescript_compiler::{Compiler, CompilerOptions, TranspileOptions};

fn main() {
    let mut compiler = Compiler::new().unwrap();
    let transpiled = compiler
        .transpile(
            "let a: string = 3".to_owned(),
            TranspileOptions {
                compilerOptions: Some(CompilerOptions {
                    target: Some("es2016".to_owned()),
                }),
            },
        )
        .unwrap();
    println!("Transpiled: {}", transpiled);
}
