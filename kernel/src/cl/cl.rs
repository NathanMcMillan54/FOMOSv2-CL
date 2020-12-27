/* kernel/src/cl/cl.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * This file starts the command line and it's main processes
 */

use fk_std::{printfk, printf};
use libc::{c_char};

extern "C" {
    fn cl_input() -> &'static str;
}

pub fn cl_main() {
    printfk!(">>> \0");
    unsafe { cl_input() };
}

#[no_mangle]
pub fn command_exists(command: &str) {
    loop {  }
}
