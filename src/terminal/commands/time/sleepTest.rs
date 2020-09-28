// this is just a example for sleeping 5 seconds
use std::process;

fn main() {
    let mut i = 0;
    let x = 1;
    loop {
        i += x;
        print!("{} \n", i);
        if i == 150000 {
            process::exit(0);
        }
    }
}

// 1 second = 30000 'x'
// 3000*5=150000 (5 seconds)
