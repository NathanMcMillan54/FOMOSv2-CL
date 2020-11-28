extern crate no_std_compat;
use no_std_compat::string;
extern crate libc;
use libc::{c_char, c_int};

pub const SPACE: &'static str = "\n\0";

extern "C" {
    fn printf(format: *const libc::c_char, ...) -> libc::c_int;
}

pub fn print(arg: &str) -> i32 {
    const TEXT: &'static str = arg;
    unsafe{ printf(TEXT.as_ptr() as *const _) }
}
