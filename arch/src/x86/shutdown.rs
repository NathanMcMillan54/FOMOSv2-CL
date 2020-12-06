use fk_std::{printf};

pub fn shutdown() {
    const SHUTDOWNMSG: &'static str = "Shutting down FOMOSv2-CL (x86)\n\0";
    unsafe { printf(SHUTDOWNMSG.as_ptr() as *mut _); }
}