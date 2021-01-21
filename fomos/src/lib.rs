/* fomos/src/lib.rs
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

extern crate builtin_commands;
#[macro_use]
extern crate fk_std;

mod cl;

extern "C" { pub fn clearScreen(); }

pub fn main_loop() {
    loop {
        cl::cl::cl_main();
    }
}
