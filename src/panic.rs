use core::panic::PanicInfo;

#[panic_handler]
pub fn on_panic(_info: &PanicInfo) -> ! {
    loop {  }
}

