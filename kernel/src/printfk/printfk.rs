extern crate libc;
use libc::{c_char, c_int};

extern "C" {
    pub fn printf(format: *const libc::c_char, ...) -> libc::c_int;
}

pub fn print(arg: &str) -> i32 {
    // let text: &str = arg + "\0";
    unsafe{ printf(arg.as_ptr() as *const _) }
}
