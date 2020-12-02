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

extern crate arch;
extern crate kernel;
extern crate fk_std;

use fk_std::{printf};

#[no_mangle]
pub extern "C" fn init_main() {
    const TEXT: &'static str = "FOMOSv2-CL v2.3.5";
    unsafe { printf(TEXT.as_ptr() as *const _); }
    loop {  }
}

fn main() {
    unsafe { init_main(); }
}
