use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("You entered: {} \n", input);
        }
        Err(error) => println!("error: {}", error),
    }
}
