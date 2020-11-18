#![no_std]
#![no_main]

// Files
mod panic;

// FOMOSv2 crates
extern crate arch;


pub fn x86_start() {
    arch::x86_test_println();
}

pub fn arm_start() {
    arch::arm_test_println();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    x86_start();
    loop { }
}
