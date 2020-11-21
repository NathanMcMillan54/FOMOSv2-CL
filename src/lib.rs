/* lib.rs
 *
 * Build:
 *
 * cargo build
 *
 * */

#![no_main]
#![no_std]
#![feature(lang_items)]
#![crate_type = "staticlib"]

mod lang;

extern crate arch;
extern crate kernel;

#[no_mangle]
pub extern "C" fn init_main() {
    loop {  }
}

pub fn main() {
    unsafe { init_main(); }
}
