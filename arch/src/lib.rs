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

#[macro_use]
extern crate fk_std;

pub mod arm;
pub mod x86;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::ARCH;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub use arm::ARCH;
