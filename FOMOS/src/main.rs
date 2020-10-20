mod arm;
mod user;
mod x86;
use crate::arm::arm::arm;
use crate::x86::x86::x86;

fn main() {

    arm();
    x86();
}
