use core::ptr;

pub fn b4_std_print(argument: &[u8]) {
    const TEXT: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = argument;
    for byte in out_str {
        unsafe {
            ptr::write_volatile(TEXT, *byte);
        }
    }
}
