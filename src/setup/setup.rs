#![feature(global_asm)]

use core::ptr;

global_asm!(include_str!("setup.s"));

pub(crate) fn strt_setup() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Starting setup... \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    // load stuff
}
