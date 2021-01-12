extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/arm/password.c");
    cc::Build::new()
        .file("src/arm/password.c")
        .compile("arm_password");
}
