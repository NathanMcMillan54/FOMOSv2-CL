use crate::restart;

#[no_mangle]
pub extern "C" fn first_setup() {
    printfk!("Starting first setup...\n\0");
    // unsafe { restart() }
}
