use core::arch;
mod arm;
mod x86;

// Power
extern "C" {
    fn arm_shutdown();
    fn x86_shutdown();
}

pub fn check_arch() {

}

pub unsafe fn shutdown() {
    #[cfg(target_arch = "arm")]
    arm_shutdown();

    // Shutdown for the cpu
}

pub unsafe fn early_print(argument: &[u8]) {
    #[cfg(target_arch = "arm")]
    arm::io::early_print::arm_early_print(argument);
    /* __________________________________________ */
    #[cfg(target_arch = "x86")]
    x86::io::early_print::x86_early_print(argument);
}
