// this file is full of things to do if FOMOS is running on a unsupported cpu
use crate::arm::arm::arm;
use crate::x86::x86::x86;

pub(crate) fn crash_cpu() {
    // this function will probably
    // increase these variables until the cpu crashes
    let mut i = 0;

    loop {
        i = i + 1;

        if i == 9223372036854775807 {
            arm();
            break;
        } else if i == 999999999999999999999 {
            x86();
            break;
        }
    }
}

pub(crate) fn check_cpu() {
    // re-run main.rs
}
