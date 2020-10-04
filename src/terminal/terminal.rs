use core::ptr;
use crate::commands::echo::echo::echo;
use crate::shutdown::shutdown::shutdown;


pub(crate)fn terminal() {
    let mut i = 0;
    echo(b"\nCL \n");
    echo(b"FOMOSv2-CL loading done \n");
    echo(b"$>> ");
    loop {
        i = i +1;
        if i == 1000000 {
            shutdown();
        }
    }
}
