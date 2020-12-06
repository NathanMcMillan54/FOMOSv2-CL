extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=linux/clear.c");
    cc::Build::new()
        .file("linux/clear.c")
        .compile("clear");
}
