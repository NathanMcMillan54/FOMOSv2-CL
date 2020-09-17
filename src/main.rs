#![no_std]
#![no_main]
#![feature(global_asm)]

use core::ptr;

mod panic;

mod setup;
use setup::setup;

// FOMOS
mod fomos;
use fomos::fomos;

global_asm!(include_str!("../boot/boot.s"));

#[no_mangle]
pub extern "C" fn not_main() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Loading FOMOSv2-CL... \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    setup();
    fomos();
}
