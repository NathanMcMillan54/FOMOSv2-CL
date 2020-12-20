/* kernel/src/cl/cl.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * This file starts the command line and it's main processes
 */

use fk_std::{printfk, scanf};

pub fn cl_main() {
    printfk!(">>> \0");
    const INPUT: &str = "";
    unsafe { scanf(INPUT.as_ptr() as *const _); }
    printfk!(INPUT);
}
