#![no_std]

use core::ptr;

static FOMOS: &[u8] = b"FOMOSv2-CL v2.2.5";

pub(crate) fn echo(argument: &[u8]) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in FOMOS.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 15;
        }
    }

    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"FOMOSv2-CL v2.2.5";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
