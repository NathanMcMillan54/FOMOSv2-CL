// this file is full of things to do if FOMOS is running on a bad cpu

pub(crate) fn crash_cpu() {
    // this function will probably
    // increase these variables until the cpu crashes
    let mut i = 0;

    loop {
        i = i + 1;

        if i == 9223372036854775807 {
            // if a gets to this number then it's likely an arm cpu
            // try to load arm FOMOS again
            break;
        }
    }
}

pub(crate) fn check_cpu() {
    // re-run main.rs
}
