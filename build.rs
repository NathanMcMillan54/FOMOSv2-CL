extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
}
