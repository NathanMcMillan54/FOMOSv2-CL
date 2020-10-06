use crate::commands::echo::echo::echo;
use crate::drivers::display::vga_buffer::test_vga;

pub(crate) fn refresh_screen() {
    echo(b"screen.rs \n");
}
