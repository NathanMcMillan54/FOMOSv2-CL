use super::{configSetup, first_setup::first_setup};

extern "C" {
    fn printName();
}

#[no_mangle]
pub extern "C" fn regular_setup() {
    printfk!("Starting setup...\n");
    unsafe { printName(); }
}

pub unsafe fn start_setup() {
    configSetup();
}
