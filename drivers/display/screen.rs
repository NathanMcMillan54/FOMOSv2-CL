fn error() {
    // some error
}

fn is_there_a_screen() {
    // add some stuff to see if the device is connected to a screen
    let mut screen = 0;
    let mut screen_on = 0;

    // if statement see if a screen is plugged in

    if screen = 1 {
        screen_on = 1
    } else {
        error()
    }

    if screen_on = 1 {
        refresh_screen();
    } else {
        // for now
        error();
    }
}

fn refresh_screen() {
    loop {
        // something to refresh screen

    }
}