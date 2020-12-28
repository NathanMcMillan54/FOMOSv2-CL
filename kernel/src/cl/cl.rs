/* kernel/src/cl/cl.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * This file starts the command line and it's main processes
 */

use fk_std::{printfk, printf};

extern "C" {
    fn cl_input() -> &'static str;
}

pub fn cl_main() {
    printfk!(">>> \0");
    unsafe { cl_input() };
}
