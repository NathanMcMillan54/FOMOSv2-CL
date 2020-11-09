extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=linux_FOMOSv2.c");
    cc::Build::new()
        .file("linux_FOMOSv2.c")
        .compile("linux_FOMSOv2.o");


}