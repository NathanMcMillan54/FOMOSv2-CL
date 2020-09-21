extern "C" {
    fn sleep() -> u64;
}

unsafe fn shutdown() {
    sleep();
}
