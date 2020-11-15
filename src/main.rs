#![no_std]
#![no_main]

mod panic;

extern crate arch;

extern "C" {
    fn cpuArch();
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { cpuArch(); }
    loop { }
}
