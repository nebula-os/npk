extern crate anyhow;
extern crate npk_typescript;

use anyhow::Result;
use std::io::{BufRead, Write};
use std::time::Duration;
use typescript_compiler::{Compiler, CompilerOptions, TranspileOptions};

fn main() -> Result<()> {
    println!("Enter \"exit\" to terminate this example");

    let mut compiler = Compiler::new().unwrap();
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let transpiled = compiler.transpile(
            input.to_owned(),
            TranspileOptions {
                compiler_options: Some(CompilerOptions {
                    target: Some("es3".to_owned()),
                    jsx: Some("react".to_owned()),
                    ..Default::default()
                }),
            },
        );
        println!("Transpiled: {:?}", transpiled);
    }

    Ok(())
}
