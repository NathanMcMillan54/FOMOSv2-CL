use fk_std::{printfk, add_str, printf};

fn command_exists(command: &str) -> bool {
    printfk!("/lib/");
    add_str(command, "\0");
    unsafe { printf(command.as_ptr() as *const _); }
    return true;
}

pub fn run_command(command: &str) {
    command_exists(command);
}
