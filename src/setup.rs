// these functions are from linux/src/setup.rs
extern "C" {
    fn arm_linux();
    fn x86_linux();
}

pub fn setup_linux() {
    unsafe { arm_linux(); }
    unsafe { x86_linux(); }
}
