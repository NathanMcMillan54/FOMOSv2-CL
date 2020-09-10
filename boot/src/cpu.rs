//! This is for ARM

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;
pub use arch_cpu::*;
