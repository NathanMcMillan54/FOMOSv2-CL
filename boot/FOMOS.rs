#![feature(global_asm)]

use core::ptr;

global_asm!(include_str!("kernel.s"));

fn fomos() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"kernel.img \nkernel.s \nFOMOS.rs \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    // start src/start.s
}
