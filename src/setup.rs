use core::ptr;

use crate::drivers::drivers::load_drivers;
use crate::user::login::login;

fn setup_user() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Login \n\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    login();
}

fn setup_drivers() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Setting up drivers \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    load_drivers()
}

pub(crate) fn setup() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Setting up FOMOS... \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    setup_drivers();
    setup_user();
}
