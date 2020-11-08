extern crate cc;
use std::*;
use std::io::{Result, Write};
use std::fs::*;

static CONFIGARCH: &str = "arm";
static ARCHTYPE: &str = CONFIGARCH;

fn compile(file: &str, name: &str) {
    let file_path: &str = stringify!("arch/" + ARCHTYPE + file);
    let name_path: &str = stringify!("arch/" + ARCHTYPE + name);
    cc::Build::new()
        .file(file_path)
        .compile(&name_path);
}

fn arm_specific() {

}

fn x86_specific() {

}

fn finish_build() {
    // Finish any non-arch specific building
}

fn check_arch() -> std::io::Result<()> {
    if fs::metadata("configs/arch.json").is_ok() {
        print!("configs/arch.json exists \n");
    } else {
        print!("configs/arch.json does not exists \n");
        let mut configs_arch_file = File::create("configs/arch.json")?;
        let default_arch = br#"
            {
                "arch": "arm"
            }"#;
        configs_arch_file.write_all(default_arch)?;
        print!("Created configs/arch.json \nSet default arch: arm \n");
    }
    Ok(())
}

fn main() {
    print!("Building FOMOSv2-CL v2.3.5... \n");
    print!("Checking CPU arch type... \n");
    check_arch();
    if ARCHTYPE == "arm" {
        arm_specific();
    } else if ARCHTYPE == "x86" {
        x86_specific()
    } else {
        arm_specific();
    }
    finish_build();
}
