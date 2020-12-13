use fk_std::{printfk};

fn command_exists(command: &str) -> bool {
    printfk!("/lib/\n\0");
    return true;
}

pub fn run_command(command: &str) {
    command_exists(command);
}
