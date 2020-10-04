#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

global_asm!(include_str!("boot/arm-boot.s"));

mod panic;
mod drivers;
mod setup;
mod commands;
mod shutdown;

mod boot;
use crate::boot::boot::boot;

mod terminal;
use crate::terminal::terminal::terminal;


#[no_mangle]
pub extern "C" fn not_main() {
    boot();
    terminal();
}
