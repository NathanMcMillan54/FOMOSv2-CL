/* kernel/src/cl/cl.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * This file starts the command line and it's main processes
 */

use include::{filePath};
use fk_std::{printfk, printf};

extern "C" {
    fn cl_input() -> &'static str;
}

pub fn cl_main() {
    printfk!(">>> \0");
    unsafe { cl_input() };
}

#[no_mangle]
pub fn command_exists(command: &str) {
    printfk!("It didn't break!\n\0");
    if unsafe { filePath(command) } == true {
        printfk!("It exists!\n\0");
    } else {
        printfk!("Command not found\n\0");
    }
    printfk!("It still didn't break!\n\0");
    loop {  }
}
