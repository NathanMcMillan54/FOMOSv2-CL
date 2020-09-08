use std::process::Command;

fn main() {
    println!("Starting FOMOS.img \n");
    // don't actually start setup.c
    // just do this for now unitl the os can be figured out
    print!("Starting setup.. \n");
    let output = Command::new("./FOMOS/setup.c")
        .arg(" ")
        .output()
        .expect("Cannot run setup.c");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}