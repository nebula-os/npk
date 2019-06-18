use std::env;

fn main() -> std::io::Result<()> {
    // Print to the stdout
    println!("Hello world");

    // Create a file
    std::fs::write("./test.txt", b"Lorem ipsum")?;
    Ok(())
}
