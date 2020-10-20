use core::ptr;

pub(crate) fn x86() {
    const UART0: *mut u8 = 0xb8000 as *mut u8;
    let out_str = b"ARCH=x86\nStarting FOMOSv2-CL v2.3.5...\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    // detect segmentation fault
    // if there is a segmentation fault then the cpu isn't supported
    // if the cpu isn't supported loop until the cpu crashes
}