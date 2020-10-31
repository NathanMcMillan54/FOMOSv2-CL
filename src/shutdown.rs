#[no_mangle]
pub unsafe extern "C" fn system_off(arch: &str) {
    loop {
        if arch == "arm" {
            unsafe {
                arm_shutdown();
            }
        } else if arch == "x86" {
            unsafe {
                x86_shutdown();
            }
        } else {
            // Default to arm
            arch == "arm";
            arm_shutdown();
        }
    }
}
