pub(crate) fn x86_early_print(argument: &[u8]) {
    static TEXT: &[u8] = argument;
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in TEXT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 15;
        }
    }
}