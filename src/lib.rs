/* lib.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * This is FOMOSv2-CL's main file, main.c calls init_main() which then starts all the processes
 * FOMOSv2-CL runs.
 * */

#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![crate_type = "staticlib"]

mod lang;
mod memory;

extern crate fk_std;
// use fk_std::{printf, strings::add_str, printfk};

extern crate arch;
extern crate kernel;
use kernel::printfk;

extern "C" {
    fn clearScreen();
}

#[no_mangle]
pub extern "C" fn init_main() {
    unsafe { clearScreen(); }
    printfk!("FOMOSv2-CL v2.3.5\n\0");

    kernel::main_loop();

    #[cfg(target_arch = "arm")]
    arch::arm::shutdown::shutdown();

    #[cfg(target_arch = "x86_64")]
    arch::x86::shutdown::shutdown();
}

fn main() {
    unsafe { init_main(); }
}
