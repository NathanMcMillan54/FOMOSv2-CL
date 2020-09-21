#![no_std]
#![no_main]
#![feature(global_asm)]

mod panic;

mod boot;
use crate::boot::boot::boot;

#[no_mangle]
pub extern "C" fn not_main() {
    boot();
}
