#![no_std]


mod arm;
mod kernel;
mod setup;
mod x86;

#[no_mangle]
pub unsafe extern "C" fn FOMOS_main() {
    setup::setup_cpu();
}
