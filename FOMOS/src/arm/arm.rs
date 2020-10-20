use core::ptr;

pub(crate) fn arm() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"ARCH=arm\nStarting FOMOSv2-CL v2.3.5...\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }


}
