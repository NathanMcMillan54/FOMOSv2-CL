// Copied from fk_std src/lib.rs

extern "C" { pub fn printf(format: *const i8, ...) -> i32; }

pub fn print(arg: &str) -> i32 {
    let space = "\n\0";
    let output = arg;
    unsafe { printf(output.as_ptr() as *const _) }
}

#[macro_export]
macro_rules! printfk {
    ($($arg:tt)*) => {$crate::printfk::print($($arg)*)};
}
