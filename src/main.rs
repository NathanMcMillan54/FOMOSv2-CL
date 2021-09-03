#![no_std]
#![no_main]

#[macro_use] extern crate novusk;

use novusk::kernel::printk::printk;

pub(crate) mod setup;

async fn main_loop() {

}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    printk!("Setting up {} {}...", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    setup::setup();

    main_loop();

    panic!("Nothing to run");
}
