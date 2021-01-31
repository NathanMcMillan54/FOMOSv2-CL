extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/clear.c");
    cc::Build::new()
        .file("src/clear.c")
        .compile("clear");

    println!("caro:rerun-if-changed=src/cl/command.c");
    cc::Build::new()
        .file("src/cl/command.c")
        .compile("command");
}
