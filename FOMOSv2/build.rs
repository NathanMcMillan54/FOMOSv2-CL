extern crate cc;
use std::*;
use std::fs::*;

struct DefaultArch {
    arch: arm
}

fn compile(file: &str, name: &str) {
    cc::Build::new()
        .file(file)
        .compile(name);
}

fn arm_specific() {
    compile("", "");
}

fn x86_specific() {
    compile("", "");
}

fn check_arch() -> std::io::Result<()> {
    if fs::metadata("configs/arch.json").is_ok() {
        print!("configs/arch.json exists \n");
    } else {
        print!("configs/arch.json does not exists \n");
        let mut configs_arch_file = File::create("configs/arch.json");
        configs_arch_file.write(DefaultArch);
        configs_arch_file.sync_all()?;
        print!("Created configs/arch.json \nSet default arch: arm \n");
    }
    Ok(())
}

fn main() {
    print!("Building FOMOSv2-CL v2.3.5... \n");
    print!("Checking CPU arch type... \n");
    check_arch();
}
