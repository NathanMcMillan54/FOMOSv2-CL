static FOMOS: &[u8] = b"FOMOSv2-CL \n";

#[no_mangle]
pub extern "C" fn not_main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in FOMOS.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {  }
}
