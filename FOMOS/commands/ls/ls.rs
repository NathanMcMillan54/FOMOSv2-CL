use std::fs;
use std::io;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let paths = fs::read_dir(input).unwrap();

            for path in paths {
                println!("{}", path.unwrap().path().display())
            }
            Ok(())
        }
        Err(error) => println!("error: {}", error),
    }
}
