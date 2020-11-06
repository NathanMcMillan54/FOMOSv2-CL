#![crate_type = "staticlib"]
#![no_main]
#![no_std]

mod fomos_err;
mod setup;
mod panic;

#[no_mangle]
pub extern fn power_err() {
    // Find what CPU FOMOS is running on then try
    // to tell the user there was an error then
    // try restart the device

}

#[no_mangle]
pub extern fn shutdown() {
    // Find what CPU FOMOS is running on then try
    // to shutdown the device
    unsafe {    }
}

#[no_mangle]
pub extern fn init() {
    setup::setup::finish_setup_linux();
    setup::setup::setup_fomos();
    loop {  }
}
