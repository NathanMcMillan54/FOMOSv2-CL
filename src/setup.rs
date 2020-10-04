use core::ptr;

use crate::commands::echo::echo::echo;

use crate::drivers::display::screen::refresh_screen;
use crate::drivers::keyboard::keyboard::arm_keyboard;


fn setup_drivers() {
    echo(b"Setting up drivers... \n");
    arm_keyboard();
    refresh_screen();
}

pub(crate) fn setup() {
    echo(b"Setting up FOMOS... \n");
    setup_drivers();
}
