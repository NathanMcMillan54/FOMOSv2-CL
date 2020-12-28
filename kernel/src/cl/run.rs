use fk_std::{printfk};

#[no_mangle]
pub fn run_command() {
    printfk!("Running command...\n\0");
}
