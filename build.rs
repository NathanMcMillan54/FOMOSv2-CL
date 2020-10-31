use cc;

fn build_arm() {
    cc::Build::new()
        .file("src/arm/shutdown.S")
        .compile("src/arm_shutdown");
}

fn build_x86() {
    cc::Build::new()
        .file("src/x86/shutdown.S")
        .compile("src/x86_shutdown");
}

fn main() {
    // Always default to arm
    let arch = "arm";
    print!("Building FOMOSv2-CL v2.3.5...\n");
    if arch == "arm" {
        print!("ARCH = {}", arch);
        build_arm();
    } else if arch == "x86" {
        print!("ARCH = {}", arch);
        build_x86();
    } else {
        print!("ARCH = ?\nDefaulting to ARCH = arm");
        print!("ARCH = {}", arch);
    }
}
