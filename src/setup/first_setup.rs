use arch::ARCH;
use fk_std::printf;
use super::{configSetup, user::make_user};

#[no_mangle]
pub extern "C" fn first_setup() {
    printfk!("Starting first setup...\n\0");
    unsafe { printf(ARCH.as_ptr() as *const _); }
    printfk!("\nSetup 0/1 Done\n\0");
    make_user();
}
