use std::fs::File;
use std::io::prelude::*;
use std::io;

/* this makes a file for some reason the edits it,
   it might be important for making files one day
*/
fn main() -> std::io::Result<()> {
    let mut input_file = String::new();
    match io::stdin().read_line(&mut input_file) {
        Ok(n) => {

        }
        Err(error) => println!("error: {}", error),
    }
    let mut file = File::create(input_file)?;
    file.write_all(b"")?;
    Ok(())
}
