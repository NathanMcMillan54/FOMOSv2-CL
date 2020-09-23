pub mod drivers;
use drivers::load_drivers;

pub mod keyboard;
use keyboard::keyboard::arm_keyboard;

pub mod display;
use display::screen::refresh_screen;


fn run_drivers() {
    load_drivers();
    arm_keyboard();
    refresh_screen();
}
