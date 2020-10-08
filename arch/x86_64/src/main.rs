#![no_std]
#![no_main]

mod panic;
mod commands;
use crate::commands::echo::echo::echo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    echo(b"FOMOSv2-CL v2.2.5");
    loop {  }
}
