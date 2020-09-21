mod load_drivers;
use load_drivers::load_drivers;

pub(crate) fn start_loading_drivers() {
    load_drivers();
}