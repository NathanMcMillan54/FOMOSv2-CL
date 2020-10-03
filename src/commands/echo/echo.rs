use core::ptr;

pub(crate) fn echo(argument: &str) {

    let echo_x = b" ";
    let echo_y = argument;
    let echo_z = echo_x + echo_y;


    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = echo_z;
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
