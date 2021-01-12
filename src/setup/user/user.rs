use arch::ARCH;
use arch::arm::{password::password_options, password::short_password};

use fk_std::*;
use crate::power::restart;

pub fn user_name() {

}

pub unsafe fn setup_password() {

}

pub fn make_password() {
    if ARCH == "arm" {
        user_name();
        let arm_pass_options = unsafe { password_options() };
        if arm_pass_options == 2 {
            unsafe { setup_password() }
        } else {
            short_password();
        }
    } else if ARCH == "x86" {
        unsafe { setup_password(); }
    }
}

pub fn make_user() {
    user_name();
    make_password();
    printfk!("Setup 1/1 Done\n\0");
}
