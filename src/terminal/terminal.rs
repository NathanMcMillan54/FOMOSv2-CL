use core::ptr;
use crate::commands::echo::echo::echo;


fn keyboard() {
    loop {
        // input
    }
}

pub(crate)fn terminal() {
    echo(b"\nCL \n");
    echo(b"FOMOSv2-CL loading done \n");
    loop {
        echo(b"$>> ");
        keyboard();
    }
}
