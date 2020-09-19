fn shutdown() {
    extern {
        // call _start_sleep form shutdown.s
        fn _start_sleep();
    }
}