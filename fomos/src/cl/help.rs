/* fomos/src/cl/help.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * The help command for FOMOS
 */

use fk_std::{c_char, printf};

#[no_mangle]
pub extern "C" fn help_main(arg: i32) {
    if arg == 0 {
        unsafe { printf(ASKU.as_ptr() as *const c_char); }
        unsafe { printf(PRINT.as_ptr() as *const c_char); }
        unsafe { printf(SHUTDOWN.as_ptr() as *const c_char); }
        unsafe { printf(HELP.as_ptr() as *const c_char); }
    } else if arg == 1 {
        unsafe { printf(PRINT.as_ptr() as *const c_char); }
    } else if arg == 2 {
        unsafe { printf(ASKU.as_ptr() as *const c_char); }
    } else if arg == 3 {
        unsafe { printf(SHUTDOWN.as_ptr() as *const c_char); }
    } else if arg == 4 {
        unsafe { printf(HELP.as_ptr() as *const c_char); }
    }
}

const ASKU: &'static str = "asku (ask user) is a command that will ask the user for their
password to confirm something. This is used for commands that are important or could effect the
user.\nCommand Example:\n\n    asku u\n\n\0";

const HELP: &'static str = "The help command (this command) is used for showing a description
 or example of a command.\nCommand Example:\n\n    help help\n\n\0";

const PRINT: &'static str = "The print command is used for printing to the command line.\n
Command Example:\n\n    print Hello, World!\n\n\0";

const SHUTDOWN: &'static str = "The shutdown command is used for shutting down your device.This
command runs the asku command because it does effect the user, FOMOSv2-CL v2.3.5 can't save all
 user data and will delete it after shutdown/restart\nCommand Example:\n\n    shutdown 1\n\n\0";
