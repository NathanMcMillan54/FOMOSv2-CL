use core::panic::PanicInfo;
use core::ptr;


#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Panic \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    loop {}
}
