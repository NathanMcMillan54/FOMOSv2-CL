extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src/cpu.S");
    cc::Build::new()
        .file("src/cpu.S")
        .compile("src/cpu");
}
