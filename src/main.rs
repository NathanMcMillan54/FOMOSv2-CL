#![feature(asm)]
#![no_std]
#![no_main]

mod setup;
mod panic;

#[no_mangle]
pub unsafe extern "C" fn FOMOS_main() {
    setup::linux_setup();
    setup::fomos_setup();
}

#[no_mangle]
pub unsafe extern "C" fn system_off() {
    asm!(
    .equ PSCI_SYSTEM_OFF, 0x84000008
    .globl system_off
    system_off:
        ldr     x0, =PSCI_SYSTEM_OFF
        hvc     #0

    );
}