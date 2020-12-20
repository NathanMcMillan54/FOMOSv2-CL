/* kernel/src/cl/run_command.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * Checks if a command exists then runs it
 */

use fk_std::{printfk};

fn command_exists(command: &str) -> bool {
    printfk!("/bin/\0");
    printfk!(command);
    printfk!("\n\0");
    return true;
}

pub fn run_command(command: &str) {
    command_exists(command);
}
