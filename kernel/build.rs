extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/clear.c");
    cc::Build::new()
        .file("src/clear.c")
        .compile("clear");

    println!("caro:rerun-if-changed=src/cl/input.c");
    cc::Build::new()
        .file("src/cl/input.c")
        .compile("input");
}
