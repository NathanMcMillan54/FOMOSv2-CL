use fk_std::{printfk};
use super::configSetup;

fn regular_setup() {
    printfk!("Starting setup...\n\0");
}

pub unsafe fn start_setup() {
    configSetup();
    regular_setup();
}
