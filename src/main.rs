#![no_std]
#![no_main]

mod panic;

extern crate arch;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop { }
}
