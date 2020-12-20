/* arch/src/x86/shutdown.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * Shutdown FOMOSv2-CL for x86 devices.
 */

use fk_std::{printfk};

pub fn shutdown() -> ! {
    printfk!("\nShutting down FOMOSv2-CL (x86)\n\0");
    loop {  }
}
