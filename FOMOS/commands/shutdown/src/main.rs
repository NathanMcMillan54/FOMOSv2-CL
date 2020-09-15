extern crate system_shutdown;

use system_shutdown::shutdown;

fn main() {
    match shutdown() {
        /* get this to run system/sleep.s
         They should only work on ARM CPU's */
        Ok(_) => println!("Shutting down..."),
        Err(error) => eprintln!("Failed to shut down: {}", error),
    }
}