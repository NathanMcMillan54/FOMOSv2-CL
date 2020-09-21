#![no_std]
#![no_main]
#![feature(global_asm)]

// global_asm!(include_str!("../kernel.img"));
global_asm!(include_str!("boot/kernel.img"));

mod panic;

mod boot;
use crate::boot::boot::boot;

mod setup;
use crate::setup::setup::strt_setup;

mod strt_command_line;
use crate::strt_command_line::run_cl::run_cl;

mod command_line;
use crate::command_line::cl::cl;


#[no_mangle]
pub extern "C" fn not_main() {
    boot();
    // if it gets this far everything is probably working
    strt_setup();
    run_cl();
    cl();

}
