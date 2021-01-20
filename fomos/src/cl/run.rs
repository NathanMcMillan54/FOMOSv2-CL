#[no_mangle]
pub extern "C" fn run_command(mut command: i32, argument: &str) {
    if command == 1 {
        builtin_commands::print_main(argument);
    } else {
        printfk!("Command not found\n\0");
    }
}
