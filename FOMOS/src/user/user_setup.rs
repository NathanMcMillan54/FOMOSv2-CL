fn user_name() {
    // stuff for user input
}

fn user_pass() {
    let pass_4_digit = true;
    let pass_6_digit = false;
    let pass_8_digit = false;
    let mut pass_digit = 0;

    if pass_4_digit == true {
        pass_digit = 4
    } else if pass_6_digit == true {
        pass_digit = 6;
    } else if pass_8_digit == true {
        pass_digit = 8;
    } else {
        // default to 4
        pass_digit = 4;
    }
}

pub(crate) fn setup_user() {
    const USER_MODE: *mut i32 = 0x0000 as *mut i32;
    let cpu_type;
    let arm_USER_MODE = 10000;
    // put x86 user mode here
    let x86_USER_MODE = 0x0000;
    if cpu_type = arm {
        // set USER_MODE to arm user mode
        USER_MODE == arm_USER_MODE;
    } else if cpu_type = x86 {
        USER_MODE == x86_USER_MODE;
    }

    let mut user_setup = false;
    if user_setup == true {
        user_name();
        user_pass();
    }
}
