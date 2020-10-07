#![no_std]
#![no_main]

mod panic;

static FOMOS: &[u8] = b"FOMOSv2-CL v2.2.5";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in FOMOS.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 15;
        }
    }
    loop {  }
}
