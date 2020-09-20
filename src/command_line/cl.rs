use core::ptr;

fn keyboard_drivers() {
    // make sure keyboard drivers are still working
}

fn screen_drivers() {
    // make sure screen drivers are still working
}

pub(crate) fn cl() {
    // run drivers
    keyboard_drivers();
    screen_drivers();

    // TODO: get input working
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Loading CL done \n\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    loop {
        let mut command = 0;
        let argument = 0;
        if command == 0 {
            // continue
        } else if command == 1 {
            // if command = 1 then a command was entered
            // run the command
        } else {
            const UART0: *mut u8 = 0x0900_0000 as *mut u8;
            let out_str = b"Not a command \n";
            for byte in out_str {
                unsafe {
                    ptr::write_volatile(UART0, *byte);
                }
            }
            // continue
        }
    }
}
