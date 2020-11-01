mod setup;
mod shutdown;
mod panic;
mod fomos_err;

pub fn main() {
    setup::linux_setup();
    setup::fomos_setup();
    unsafe { shutdown::shutdown("arm"); }
}

