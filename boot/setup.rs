use core::ptr;

fn shutdown() {
    // something to stop FOMOS
    // this will only be called if there is a problem (incorrect login)
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Emergency shutdown \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}

fn next() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Loading next setup task... \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}

fn strt_fomos() {
    // run start.s
}

fn load_drivers() {
    let a = 4;
    let b = 1;
    b + a;
    // something to run all the drivers
}

fn login() {
    let a = 4;
    let b = 1;
    b + a = c;
    if c = 5 {
        next();
    } else {
        shutdown();
    }
}

fn strt_setup() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Starting setup... \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    load_drivers();
    login();
    strt_fomos();
}
