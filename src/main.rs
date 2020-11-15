#![no_std]
#![no_main]

// Files
mod panic;

// FOMOS crates
extern crate arch;

extern "C" {
    fn checkArch();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let mut cpu_arch = unsafe { checkArch(); };
    /* if cpu_arch == "arm" {
        arch::arm_test_println();
    } else if cpu_arch == "x86" {
        arch::x86_test_println();
    } */
    unsafe { checkArch(); }
    arch::x86_test_println();
    loop { }
}
