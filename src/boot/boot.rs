#[path="../setup/mod.rs"]
mod setup;
use crate::strt_setup;

#[path="../../drivers/mod.rs"]
mod drivers;
// use crate::start_loading_drivers;
use crate::boot::boot::drivers::start_loading_drivers;

use core::ptr;

pub(crate) fn boot() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Booting... \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    // anything you want to run on startup put in here
    let mut a = 1;
    let mut b = 2;
    a + b;
    strt_setup();
    start_loading_drivers();
}
