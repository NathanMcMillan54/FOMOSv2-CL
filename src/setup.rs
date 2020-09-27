use core::ptr;

use crate::drivers::display::screen::refresh_screen;
use crate::drivers::keyboard::keyboard::arm_keyboard;
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
    let out_str = b"Setting up drivers... \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    arm_keyboard();
    refresh_screen();
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
