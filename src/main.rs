mod setup;
mod shutdown;

pub unsafe fn main() {
    setup::linux_setup();
    setup::fomos_setup();
    shutdown::system_off("arm");
}

