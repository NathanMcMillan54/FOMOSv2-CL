/* printfk/pfk_macro.rs
 * Macro by Nathan McMillan
 *
 * Description:
 * printfk is a Rust macro for FOMOSv2-CL that replaces the println macro for any parts that run in
 * #![no_std]
 * */

use super::printfk;

#[macro_use]
macro_rules! printfk {
    ($($arg:tt)*) => (printfk::print(format_args!($($arg)*)));
}
