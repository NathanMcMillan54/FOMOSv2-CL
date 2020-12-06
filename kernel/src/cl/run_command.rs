use crate::printfk;
use core::intrinsics::abort;

fn command_exists(command: &str) -> bool {

    return true;
}

pub fn run_command(command: &str) {
    let mut exists: bool = command_exists(command);
    if exists = true {
        printfk!("Command does exist\n\0");
    } else {
        printfk!("Command does not exist\n\0");
        unsafe { abort(); }
    }
}
