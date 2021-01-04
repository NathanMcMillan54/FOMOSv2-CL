use fk_std::{printfk};

#[no_mangle]
pub extern "C" fn first_setup() {
    printfk!("Starting first setup...\n\0");
}