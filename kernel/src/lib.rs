#![no_std]
#![no_main]
#![feature(lang_items)]

// Files
mod fomos_err;

// FOMOSv2 crates
// extern crate arch;


/* pub fn x86_start() {
    arch::x86_test_println();
}

pub fn arm_start() {
    arch::arm_test_println();
} */

pub fn init(arch: &str) {
    // x86_start();
    loop { }
}
