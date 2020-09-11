use std::fs;
use std::io;

fn main() {
    let mut dir_name = String::new();
    match io::stdin().read_line(&mut dir_name) {
        Ok(n) => {
            println!("You entered: {} \n", dir_name);
        }
        Err(error) => println!("error: {}", error),
    }
    let paths = fs::read_dir(dir_name).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
