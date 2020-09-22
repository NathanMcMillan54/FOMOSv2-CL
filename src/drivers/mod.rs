pub mod drivers;
use drivers::load_drivers;

fn command_line() {
    load_drivers();
}
