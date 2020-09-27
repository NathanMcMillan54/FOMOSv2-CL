use core::ptr;

pub(crate) fn login() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Login \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    // this could be the test to see if input works
    // run keyboard
}
