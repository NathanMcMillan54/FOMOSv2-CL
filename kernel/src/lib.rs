#![no_std]
#![no_main]
#![feature(lang_items)]
#![crate_type = "staticlib"]

extern crate fk_std;

// Files
mod cl;

pub fn main_loop() {
    cl::cl::cl_main();
}
