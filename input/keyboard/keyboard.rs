fn check_for_event() {
    global_asm!(include_str!("keyboard.s"));
}

fn keyboard_loop() {
    loop {
        check_for_event();
    }
}