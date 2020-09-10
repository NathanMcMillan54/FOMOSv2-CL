use std::io;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("Making directory {} \n", input);
        }
        Err(error) => println!("error: {}", error),
    }
    fs::create_dir(input)?;
    print!("Directory made successfully \n");
    Ok(())
}
