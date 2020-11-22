#![no_std]

use core::arch;

pub mod arm;
pub mod x86;

pub fn check_arch() {
    #[cfg(any(target_arch = "arm"))] {
        return unsafe {  }
    }
}

pub fn shutdown() {

}

pub fn x86_test_println() {
    x86::io::vga_buffer::test_print();
}

pub fn arm_test_println() {

}

