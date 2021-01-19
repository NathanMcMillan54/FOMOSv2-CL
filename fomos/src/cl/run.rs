use fk_std::{printfk, printf, c_char};
use builtin_commands::print_main;

#[no_mangle]
pub extern "C" fn run_command(commad: &str, arguments: &str) {
    printfk!("\nArgument[s]:\n\0");
    print_main(arguments);
    printfk!("\nCommand:\n\0");
    print_main(commad);
}
