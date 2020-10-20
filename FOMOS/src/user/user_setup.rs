fn user_name() {
    // stuff for user input
}

fn user_pass() {
    let mut pass_4_digit = true;
    let mut pass_6_digit = false;
    let mut pass_8_digit = false;
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
    let mut user_setup = false;
    if user_setup == false {
        user_name();
        user_pass();
    }
}
