use std::process::Command;

fn main() {
    print!("Making FOMOS image... \n");
    Command::new("dd if=/../FOMOS of=FOMOS.img bs=512 count=sectors")
        .spawn()
        .expect("Could not make Image");
}