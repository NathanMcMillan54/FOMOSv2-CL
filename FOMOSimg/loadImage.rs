use std::process::Command;

fn main() {
    print!("Loading FOMOS image...");
    let output = Command::new("./FOMOS.img")
        .arg(" ")
        .output()
        .expect("Cannot load FOMOS.img, do it yourself");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}