#[repr(C)]
struct ArmShutdown {
    pub csr: u32,
    pub rvr: u32,
    pub cvr: u32,
    pub calib: u32,
}

pub extern "C" fn arm_shutdown() {
    let armshutdown = 0x84000008 as *mut ArmShutdown;
    let shutdown = unsafe { (*armshutdown).cvr };
}
