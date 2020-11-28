extern crate libc;
use libc::{c_char, c_int, strcat};

pub const SPACE: &'static str = "\n\0";

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

extern "C" {
    fn add_str(str1: *mut *const c_char, str2: *mut *const c_char) -> c_int;
}

pub fn print(arg: &str) -> i32 {
    const TEXT: &'static str = arg;
    unsafe { printf(TEXT.as_ptr() + SPACE.as_ptr() as *const _ ) }
}
