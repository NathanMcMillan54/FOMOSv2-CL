#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(global_asm)]

global_asm!(include_str!("../x86Init.S"));
global_asm!(include_str!("../armInit.S"));

mod lang;

extern crate kernel;

#[no_mangle]
pub extern "C" fn arm_start() {
    kernel::init("arm");
}

#[no_mangle]
pub extern "C" fn x86_start() {
    kernel::init("x86");
}


// If there is no CPU arch specified run _start defaults to arm

#[no_mangle]
pub extern "C" fn _start() {
    arm_start();
    loop {  }
}
