extern crate cc;

fn main() {
    println!("carg:rerun-if-changed=src/files.c");
    cc::Build::new()
        .file("src/files.c")
        .compile("files");
}