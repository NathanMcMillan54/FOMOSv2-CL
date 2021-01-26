#[no_mangle]
pub extern "C" fn help_main(arg: i32) {
    if arg == 0 {
        printfk!("All\n\0");
    } else if arg == 1 {
        printfk!("print\n\0");
    } else if arg == 2 {
        printfk!("Asku\n\0");
    } else if arg == 3 {
        printfk!("Shutdown\n\0");
    } else if arg == 4 {
        printfk!("Help\n\0");
    }
}
