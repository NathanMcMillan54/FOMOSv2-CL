use crate::fomos_err::setup_err;

pub(crate) fn setup_fomos() {
    let first_time_setup = false;

    if first_time_setup == true {
        first_time_fomos_setup();
    } else {
        regular_setup();
    }
}


fn first_time_fomos_setup() {
    // Setup FOMOSV2-CL v2.3.5 for the first time
    // Restart device then run regular_setup()
    regular_setup();
}

fn regular_setup() {
    // Setup FOMOSV2-CL v2.3.5
}

