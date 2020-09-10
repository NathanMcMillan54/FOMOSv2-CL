use std::fs;
use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("You entered: {} \n", input);
        }
        Err(error) => println!("error: {}", error),
    }
    let paths = fs::read_dir(input).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}