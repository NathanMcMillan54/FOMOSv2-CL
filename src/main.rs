#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

// global_asm!(include_str!("../kernel.img"));
global_asm!(include_str!("boot/kernel.s"));

mod panic;

mod boot;
use crate::boot::boot::boot;

mod terminal;
use crate::terminal::terminal::terminal;

mod drivers;
mod setup;
mod commands;


#[no_mangle]
pub extern "C" fn not_main() {
    boot();
    terminal();
}
