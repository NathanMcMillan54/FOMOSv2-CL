#![crate_type = "staticlib"]
#![feature(lang_items)]
#![no_main]
#![no_std]

#[path = "../arch/arch.rs"]
mod arch;

mod fomos_err;
mod setup;
mod lang;
mod memorymap;

#[no_mangle]
pub extern fn power_err() {
    // Find what CPU FOMOS is running on then try
    // to tell the user there was an error then
    // try restart the device
}

#[no_mangle]
pub extern fn init() {
    setup::setup::finish_setup_linux();
    setup::setup::setup_fomos();
    loop {  }
    unsafe { arch::shutdown(); }
}
