pub mod drivers;
use drivers::load_drivers;

pub mod keyboard;
use keyboard::keyboard::arm_keyboard;

pub mod display;
use display::screen::refresh_screen;
