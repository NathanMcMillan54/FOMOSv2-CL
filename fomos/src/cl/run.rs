use fk_std::{printfk, printf, c_char};
use builtin_commands::print_main;

#[no_mangle]
pub fn run_command(command: c_char, argument: &str) {
    /* printfk!("Running command \0");
    // unsafe { printf(command.as_ptr() as *const _); }
    printfk!(" with argument[s]: \0");
    unsafe { print_main("print_main\n"); }
    // unsafe { printf(command as *const c_char); }
    /* if command == "print" {
        use builtin_commands::print_main;
        unsafe { print_main("Running print command\0"); }
    } else {
        use builtin_commands::print_main;
        // unsafe { print_main("The command wasn't print but still running it anyway\n\0"); }
        unsafe { print_main(argument); }
    } */ */
}
