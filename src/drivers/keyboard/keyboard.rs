use core::ptr;
// use cc;
// make support for US QWERTY keyboard first

pub(crate) fn arm_keyboard() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"keyboard.rs \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    // run keyboard_connected.s
    // run keyboard.s
    /* cc::Build::new()
        .file("keyboard.s")
        .compile("keyboard");

    extern {
        fn keyboard();
    }

    pub fn keyboard_input() {
        unsafe {
            keyboard();
        }
    } */
}
