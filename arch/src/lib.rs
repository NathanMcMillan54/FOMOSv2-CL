#![no_std]

pub mod arm;
pub mod x86;

extern "C" {
    fn arm_shutdown();
    fn x86_shutdown();
}

pub fn shutdown() {
    #[cfg(target_arch = "arm")]
    unsafe { arm_shutdown(); }

    #[cfg(target_arch = "x86")]
    unsafe { x86_shutdown(); }
}

pub fn x86_test_println() {
    x86::io::vga_buffer::test_print();
}

pub fn arm_test_println() {

}

