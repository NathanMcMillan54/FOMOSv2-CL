extern crate cc;

fn main() {
    // ARM
    println!("cargo:rerun-if-changed=armInit.S");
    cc::Build::new()
        .file("armInit.S")
        .compile("armInit");


    // x86
    println!("cargo:rerun-if-changed=x86Init.S");
    cc::Build::new()
        .file("x86Init.S")
        .compile("x86Init");
}
