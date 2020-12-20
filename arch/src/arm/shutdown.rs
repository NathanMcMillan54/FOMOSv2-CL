/* arch/src/arm/shutdown.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * Shutdown FOMOSv2-CL for arm devices.
 */

use fk_std::{printfk};

pub fn shutdown() -> ! {
    printfk!("\nShutting down FOMOSv2-CL (arm)\n\0");
    loop {  }
}
