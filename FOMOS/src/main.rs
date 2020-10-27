mod arm;
mod kernel;
mod user;
mod x86;
use crate::arm::arm::arm;
use crate::x86::x86::x86;

#[no_mangle]
pub unsafe extern "C" fn FOMOS_main() {
    arm();
    x86();
}
