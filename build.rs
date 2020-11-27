extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=std/io.c");
    cc::Build::new()
        .file("std/io.c")
        .compile("std_io");
}
