use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("Opening file: {} \n", input);
        }
        Err(error) => println!("error: {}", error),
    }
    let filename = input;
    // Read only
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
