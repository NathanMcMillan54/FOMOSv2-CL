use core::ptr;


// if main.rs got this far then FOMOS should be working good
pub(crate) fn fomos() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Loading FOMOSv2 done \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
