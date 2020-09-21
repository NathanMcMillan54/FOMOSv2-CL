extern "C" {
    fn sleep() -> u64;
}

unsafe fn shutdown() {
    // this might be the first time echo command could be used in FOMOS
    // echo("Shutting down... \n");
    sleep();
}
