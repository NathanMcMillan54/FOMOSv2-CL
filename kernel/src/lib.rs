#![no_std]
#![no_main]
#![feature(lang_items)]
#![crate_type = "staticlib"]

// Files
mod cl;

pub fn main_loop() {
    loop {
        cl::cl::cl_main();
    }
}
