use fk_std::{printfk};
use fomos::readFile;
use crate::setup::first_setup::first_time_setup;

fn config_setup() {
    printfk!("\nReading configuration files...\n\0");
    let times_started = unsafe { readFile("/configs/boot/startupTimes") };
    if times_started == "0" {
        printfk!("/configs/startupTimes = 0\n\0");
        first_time_setup();
    }
}

pub fn start_setup() {
    config_setup();
}
