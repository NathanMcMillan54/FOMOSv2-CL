extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=input.c");
    cc::Build::new()
        .file("input.c")
        .compile("input");
}
