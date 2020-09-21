// in this file it will have all FOMOS commands that will run in files
// for example instead of using:
/*
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Print! \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
*/
// to print things you can just call the echo command. Example:
/*
    let mut print = "Print! \n";
    echo(print)
*/
