/* src/lang.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * Panic handler.
 */

use core::panic::{PanicInfo};

#[panic_handler]
pub fn on_panic(_info: &PanicInfo) -> ! {
    loop {  }
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {    }
