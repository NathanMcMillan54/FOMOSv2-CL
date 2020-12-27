#![no_std]

extern crate fk_std;

extern "C" { pub fn filePath(path: &str) -> bool; }
