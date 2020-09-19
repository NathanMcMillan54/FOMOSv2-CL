// this will run the command line
#![feature(global_asm)]

use core::ptr;

global_asm!(include_str!("start.s"));

pub fn run_cl() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let load_fomos_done = b"Loading FOMOSv2-CL done \n";
    for byte in load_fomos_done {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    let load_cl_done = b"Loading CL... \n";
    for byte in load_cl_done {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
