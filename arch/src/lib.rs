/* arch/src/lib.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * This file lets FOMOSv2-CL use and run arch specific code.
 */

#![no_std]
#![crate_type = "staticlib"]

extern crate fk_std;

pub mod arm;
pub mod x86;
