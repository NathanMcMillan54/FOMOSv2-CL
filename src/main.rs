#![no_std]
#![no_main]

mod panic;

static BOOTTEXT: &[u8] = b"x86 FOMOSv2-CL v2.3.5";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in BOOTTEXT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xF;
        }
    }

    loop {  }
}
