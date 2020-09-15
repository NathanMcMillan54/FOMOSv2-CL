use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main() -> std::io::Result<()> {
    let mut input_file = String::new();
    match io::stdin().read_line(&mut input_file) {
        Ok(n) => {

        }
        Err(error) => println!("error: {}", error),
    }
    let mut file = File::create(input_file)?;
    file.write_all(b"ef command")?;
    Ok(())
}
