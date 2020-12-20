extern crate cc;

extern crate serde_json;
use serde_json::{de};
extern crate std;
use std::{fs, process, io::Write};

// OS image, kernel image, FOMOS image
fn boot_dir() {

}

// OS info
fn os_dir() -> std::io::Result<()> {
    fs::create_dir("initramfs/os/kernel/");
    fs::create_dir("initramfs/os/kernel/linux");

    fs::create_dir("initramfs/os/FOMOS/");

    let mut offv = fs::File::create("initramfs/os/FOMOS/FOMOSV")?;
    offv.write_all(b"2.3.5");
    let mut offvn = fs::File::create("initramfs/os/FOMOS/FOMOSVN")?;
    offvn.write_all(b"CL (command line)");
    let mut offfn = fs::File::create("initramfs/os/FOMOS/FOMOSFN")?;
    offfn.write_all(b"FOMOSv2-CL v2.3.5");

    let mut oklv = fs::File::create("initramfs/os/kernel/linux/LinuxV")?;
    oklv.write_all(b"5.9.0");
    let mut oklfn = fs::File::create("initramfs/os/kernel/linux/LinuxFN")?;
    oklfn.write_all(b"Linux v5.9")
}

// Installed binaries
fn bin_dir() {
    let commandp = "initramfs/bin/commands/";
    let commandr = "https://github.com/sbFomos/builtin_commands/trunk/";
    let commandn: &str = "";

    let print = "print/";
    for i in 1..2 {
        if i == 1 {
            format!("{}{}", commandn, print);
            format!("{}{}", commandr, print);
            format!("{}{}", commandp, print);
        }
        process::Command::new("svn")
            .arg("checkout")
            .arg(commandr)
            .spawn()
            .expect("Cannot run command");

        fs::rename(commandn, commandp);
        fs::remove_dir(commandn);
    }
}

// Installed libraries
fn lib_dir() {

}

// User stuff
fn home_dir() {

}

fn root() {
    fs::create_dir("initramfs/boot/");
    boot_dir();
    fs::create_dir("initramfs/os/");
    os_dir();
    fs::create_dir("initramfs/bin/");
    bin_dir();
    fs::create_dir("initramfs/lib/");
    lib_dir();
    fs::create_dir("initramfs/home/");
    home_dir();
}

fn main() {
    println!("cargo:rerun-if-changed=src/clear.c");
    cc::Build::new()
        .file("src/clear.c")
        .compile("clear");

    fs::create_dir("initramfs/");
    root();
}
