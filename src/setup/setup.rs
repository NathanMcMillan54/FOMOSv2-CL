use fk_std::{printfk, printf};
use super::{configSetup, first_setup::first_setup};

#[no_mangle]
pub extern "C" fn regular_setup() {
    printfk!("Starting setup...\n\0");
}

pub unsafe fn start_setup() {
    configSetup();
}
