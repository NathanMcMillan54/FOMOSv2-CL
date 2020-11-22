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
#![crate_type = "staticlib"]

mod lang;

extern crate arch;
extern crate kernel;

use kernel::*;

extern crate libc;

#[no_mangle]
pub extern "C" fn init_main() {
    // printfk!("FOMOSv2-CL v2.3.5");
    const TEXT: &'static str = "FOMOSv2-CL v2.3.5 Rust libc\n";
    unsafe { libc::printf(TEXT.as_ptr() as *const _); }
    loop {  }
}

fn main() {
    unsafe { init_main(); }
}
