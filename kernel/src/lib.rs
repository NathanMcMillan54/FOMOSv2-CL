#![no_std]
#![no_main]
#![feature(lang_items)]

// Files
mod fomos_err;
mod user;

// FOMOSv2 crates
// extern crate arch;

pub fn init(arch: &str) {
    let mut cpu = arch;
    if cpu == "arm" {
        // arm stuff
    } else if cpu == "x86" {
        // x86 stuff
    } else {
        // Try to restart device
    }
    loop { }
}
