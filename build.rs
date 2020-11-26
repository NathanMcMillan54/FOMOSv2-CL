extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=rust-c/std/io.c");
    cc::Build::new()
        .file("rust-c/std/io.c")
        .compile("std_io");
}