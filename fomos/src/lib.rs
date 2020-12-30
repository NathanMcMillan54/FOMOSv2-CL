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

extern crate fk_std;

mod cl;

extern "C" { pub fn clearScreen(); }
extern "C" { pub fn readFile(file: &str) -> &str; }

pub fn main_loop() {
    loop {
        cl::cl::cl_main();
    }
}
