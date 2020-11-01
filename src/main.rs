mod setup;
mod shutdown;

pub fn main() {
    setup::linux_setup();
    setup::fomos_setup();
    unsafe { shutdown::shutdown("arm"); }
}

