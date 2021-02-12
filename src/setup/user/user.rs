use arch::ARCH;
use arch::arm::{password::password_options, password::short_password};

use fk_std::*;
use crate::power::restart;

extern "C" {
    fn makePassword();
}

extern "C" {
    fn makeUserName();
}

pub unsafe fn user_name() {
    printfk!("Enter your name: ");
    makeUserName();
}

pub unsafe fn setup_password() {
    printfk!("Enter your password, should be atleast 8 charcaters long: ");
    // Make a password and add it to /config/user/password
    makePassword();
}

fn arm_specific_password() {
    let arm_pass_options = unsafe { password_options() };
    if arm_pass_options == 2 {
        unsafe { setup_password(); }
    } else {
        short_password();
    }
}

pub unsafe fn make_password() {
    printfk!("src/setup/user/user.rs - 36:43\n");
    if ARCH == "arm" {
        // arm_specific_password();
        setup_password();
    } else if ARCH == "x86" {
        setup_password();
    }
}

pub unsafe fn make_user() {
    user_name();
    setup_password();
    printfk!("Setup 1/1 Done\n");
}
