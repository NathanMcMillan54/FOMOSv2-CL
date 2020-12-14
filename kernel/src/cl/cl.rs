use fk_std::{printfk, scanf};

extern "C" {
    fn input();
}

pub fn cl_main() {
    printfk!(">>> \0");
    unsafe { input(); }
}
