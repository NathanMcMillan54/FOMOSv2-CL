#![crate_type = "staticlib"]
#![feature(lang_items)]
#![no_main]
#![no_std]

extern crate arch;

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
    unsafe { arch::early_print(b"Starting FOMOSv2-CL v2.3.5...\n") }
    setup::linux::finish_linux_setup();
    setup::setup::setup_fomos();
    unsafe { arch::shutdown(); }
}
