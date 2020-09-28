use std::process;

pub(crate) fn sleep_s() {
    // sleep in seconds
    // 1 second = 30000 increase of a number in loop
    let argument;
    let mut i = 0;
    let x = 1;
    if argument < 60 {
        loop {
            i += x;
            if i == 30000 {
                process::exit(0);
            } else {
                // nothing
            }
        }
    }
}

pub(crate) fn sleep_m() {
    // sleep in minutes
    // 1 minute = 1800000 increase of a number in loop
    let argument;
    let mut i = 0;
    let x = 1;
    if argument < 60  {
        loop {
            i += x;
            if i == 1800000 {
                process::exit(0);
            } else {
                // nothing
            }
        }
    }
}

pub(crate) fn sleep_h() {
    // sleep in hours
    // 1 hour = 108000000 increase of a number in a loop
    let argument;
    let mut i = 0;
    let x = 1;
    if argument < 60 {
        loop {
            i += x;
            if i == 108000000 {
                process::exit(0);
            } else {
                // nothing
            }
        }
    }
}
