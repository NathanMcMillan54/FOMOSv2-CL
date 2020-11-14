#![no_std]
// #![no_main]

mod arm;
mod x86;

extern "C" {
    fn arm_shutdown();
    fn x86_shutdown();
}

pub unsafe fn shutdown() {
    #[cfg(target_arch = "arm")]
    arm_shutdown();

    #[cfg(target_arch = "x86")]
    x86_shutdown();
}

pub unsafe fn early_print(argument: &[u8]) {
    #[cfg(target_arch = "arm")]
    arm::io::early_print::arm_early_print(argument);
    /* __________________________________________ */
    #[cfg(target_arch = "x86")]
    x86::io::early_print::x86_early_print(argument);
}
