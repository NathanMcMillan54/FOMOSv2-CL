use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("file.txt")?;
    file.write_all(b"ef command")?;
    Ok(())
}
