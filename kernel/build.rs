extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=string.c");
    cc::Build::new()
        .file("string.c")
        .compile("string");
}