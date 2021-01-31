/* fomos/src/cl/cl.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * This file starts the command line and runs commands
 */

use super::cl_input;
use super::help;

pub fn cl_main() {
    printfk!(">>> \0");
    unsafe { cl_input() };
}

#[no_mangle]
pub extern "C" fn run_command(mut command: i32, argument: &str) {
    if command == 1 {
        builtin_commands::print_main(argument);
    } else if command == 2 {
        builtin_commands::asku_main();
    } else if command == 3 {
        builtin_commands::shutdown_main(argument);
    } else if command == 5 {
        builtin_commands::makefile_main(argument);
    }
    else {
        printfk!("Command not found\n\0");
    }
}
