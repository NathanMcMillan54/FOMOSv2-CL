#![feastur(asm)]
#![feature(global_asm)]

fn main() {
    while 1 {
        global_asm!("hlt");
    }
}