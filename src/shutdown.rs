extern "C" {
    fn arm_shutdown();
}

extern "C" {
    fn x86_shutdown();
}

pub unsafe fn shutdown(arch: &str) {
    loop {
        if arch == "arm" {
            arm_shutdown();
        } else if arch == "x86" {
            x86_shutdown();
        } else {
            // Default to arm
            arch == "arm";
            arm_shutdown();
        }
    }
}
