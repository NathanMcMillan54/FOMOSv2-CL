use fk_std::{printfk, scanf};

pub fn cl_main() {
    printfk!(">>> \0");
    const INPUT: &str = "b";
    unsafe { scanf(INPUT.as_ptr() as *const _); }
    printfk!(INPUT);
}
