use core::panic::{PanicInfo};

#[panic_handler]
pub fn on_panic(_info: &PanicInfo) -> ! {
    loop {  }
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {    }


#[alloc_error_handler]
fn alloc_error_handler(_layout: core::alloc::Layout) -> ! {
    loop {  }
}
