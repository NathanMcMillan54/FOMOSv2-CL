mod fomos_err;
mod setup;
mod shutdown;

fn check_arch() {
    // Read arch.json
}

#[no_mangle]
pub extern fn init() {
    check_arch();
    setup::setup::finish_setup_linux();
    setup::setup::setup_fomos();
    unsafe { shutdown::shutdown("arm"); }
}
