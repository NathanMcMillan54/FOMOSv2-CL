use crate::commands::echo::echo::echo;
use core::panic::PanicInfo;

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    echo(b"Panic \n");
    loop {  }
}
