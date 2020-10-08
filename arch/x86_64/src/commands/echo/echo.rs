pub(crate) fn echo(argument: &[u8]) {
    static ECHO_ARGUMENT: &[u8] = argument;
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in ECHO_ARGUMENT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 15;
        }
    }
}
