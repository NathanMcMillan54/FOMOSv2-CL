extern crate libc;
use libc::{c_char, c_int};

use super::super::printfk::printfk::{print};

extern "C" {
    fn scanf(format: *const libc::c_char, ...) -> c_int;
}

pub fn read(arg: i32) {
    const INPUT: &'static str = arg;
    unsafe { scanf(INPUT.as_ptr() as *const _); }
    print(INPUT);
}
