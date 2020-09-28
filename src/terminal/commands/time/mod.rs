pub mod sleep;
use sleep::sleep;

fn call_sleep_s() {
    sleep::sleep_s();
}

fn call_sleep_m() {
    sleep::sleep_m();
}

fn call_sleep_h() {
    sleep::sleep_h();
}
