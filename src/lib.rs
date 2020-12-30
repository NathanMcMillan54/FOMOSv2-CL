/* lib.rs
 *
 * Build/compile:
 * cargo build
 * sh build.sh <arch>
 *
 * Description:
 * This is FOMOSv2-CL's main file, main.c calls init_main() which then starts all the processes
 * FOMOSv2-CL runs.
 * */

#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![crate_type = "staticlib"]

mod lang;
mod memory;
mod power;
use power::{shutdown, restart};

mod setup;
use setup::{setup::start_setup};

#[macro_use]
extern crate fk_std;
use fk_std::{printfk};

extern crate arch;
extern crate fomos;
use fomos::{clearScreen};

#[no_mangle]
pub extern "C" fn init_main() -> ! {
    unsafe { clearScreen() }
    printfk!("FOMOSv2-CL v2.3.5\n\0");

    start_setup();
    fomos::main_loop();

    unsafe { shutdown() }
}

fn main() {
    unsafe { init_main(); }
}
