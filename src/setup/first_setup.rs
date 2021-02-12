use arch::ARCH;
use super::{configSetup, user::user};

#[no_mangle]
pub extern "C" fn first_setup() {
    printfk!("Starting first setup...\n");
    printfk!(ARCH);
    printfk!("\nSetup 0/1 Done\n");
    unsafe { user::make_user(); }
}
