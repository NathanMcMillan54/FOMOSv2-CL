extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/clear.c");
    cc::Build::new()
        .file("src/clear.c")
        .compile("clear");

    println!("cargo:rerun-if-changed=src/files.c");
    cc::Build::new()
        .file("../include/src/files.c")
        .compile("files");

    println!("caro:rerun-if-changed=src/cl/input.c");
    cc::Build::new()
        .file("src/cl/command.c")
        .compile("input");
}
