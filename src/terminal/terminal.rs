use core::ptr;


pub(crate)fn terminal() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"\nCL \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    loop {

    }
}