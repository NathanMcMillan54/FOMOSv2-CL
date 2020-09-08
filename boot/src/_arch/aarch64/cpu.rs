global_asm!(include_str!("cpu.S"));

/// Pause execution on the core.
#[inline(always)]
pub fn wait_forever() -> ! {
    unsafe {
        loop {
            asm!("wfe" :::: "volatile")
        }
    }
}
