fn show_error() {
    // some error
}


global_asm!(include_str!("display.s"));
fn refresh_screen() {
    loop {
        let mut error = 0;
        if error = 1 {
            show_error();
        }
    }
}
