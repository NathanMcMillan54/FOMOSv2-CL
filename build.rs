/* build.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * Build script for FOMOSv2-CL, compiles and makes root directory for FOMOS.
 */

extern crate cc;

extern crate std;
use std::{fs, process, io::Write};
use std::fs::File;

// OS image, fomos image, FOMOS image
fn boot_dir() {

}

// OS info
fn os_dir() -> std::io::Result<()>{
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
    process::Command::new("git")
        .arg("clone")
        .arg("https://github.com/sbFomos/builtin_commands.git")
        .spawn();
}

// Installed libraries
fn lib_dir() {

}

// User[s], name[s], password[s], settings
fn config_dir() -> std::io::Result<File> {
    fs::create_dir("initramfs/configs/boot/");
    fs::create_dir("initramfs/configs/user/");
    let mut cbst = fs::File::create("initramfs/configs/boot/startupTimes")?;
    cbst.write_all(b"0");
    fs::File::create("initramfs/configs/user/password")?;
    fs::File::create("initramfs/configs/user/name")
}

// User directory
fn home_dir() {

}

fn root() {
    fs::create_dir("initramfs/bin/");
    bin_dir();
    fs::create_dir("initramfs/boot/");
    boot_dir();
    fs::create_dir("initramfs/os/");
    os_dir();
    fs::create_dir("initramfs/lib/");
    lib_dir();
    fs::create_dir("initramfs/configs/");
    config_dir();
    fs::create_dir("initramfs/home/");
    home_dir();
}

fn main() {
    println!("cargo:rerun-if-changed=src/setup/config.c");
    cc::Build::new()
        .file("src/setup/config.c")
        .compile("config");

    println!("cargo:rerun-if-changed=src/setup/user/setupPassword.c");
    cc::Build::new()
        .file("src/setup/user/setupPassword.c")
        .compile("setup_password");

    println!("cargo:rerun-if-changed=src/setup/user/setupName.c");
    cc::Build::new()
        .file("src/setup/user/setupName.c")
        .compile("user_name");

    fs::create_dir("initramfs/");
    root();
}
