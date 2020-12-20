/* kernel/src/lib.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * This lets FOMOS run it's main and most important processes.
 */

#![no_std]
#![feature(lang_items)]
#![crate_type = "staticlib"]

extern crate fk_std;

// Files
mod cl;

pub fn main_loop() {
    cl::cl::cl_main();
}
