mod fomos_err;
mod setup;
mod shutdown;
mod panic;

fn check_arch() {

}

#[no_mangle]
pub extern fn shutdown_fomos() {
    unsafe { shutdown::shutdown("arm"); }
}

#[no_mangle]
pub extern fn init() {
    let arch = check_arch();
    check_arch();
    setup::setup::finish_setup_linux();
    setup::setup::setup_fomos();
    unsafe { shutdown::shutdown("arm"); }
}
