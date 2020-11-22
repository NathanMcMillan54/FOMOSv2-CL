extern crate libc;

const SPACE: &'static str = "\
";

pub fn _print(_args: &[u8]) {
    let print: &[u8] = _args;
    // print = print + SPACE;
    unsafe { libc::printf(print.as_ptr() as *const _); }
}
