extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src/cpu.c");
    cc::Build::new()
        .file("src/cpu.c")
        .compile("cpu");
}
