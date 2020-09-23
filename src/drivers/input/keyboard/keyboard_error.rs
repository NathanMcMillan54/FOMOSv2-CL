use core::ptr;

global_asm!(include_str!("keyboard_connect.s");

fn keyboard_error() {
    // if this runs then there is a problem
}
