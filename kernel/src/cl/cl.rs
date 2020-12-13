use fk_std::{printfk, readfk, scanf, printf};

pub fn cl_main() {
    let mut input = " ";
    unsafe { scanf(input.as_ptr() as *const _); }
    unsafe { printf(input.as_ptr() as *const _); }
}
