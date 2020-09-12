use std::process::Command;

fn shutdown() {
    let output = Command::new("cargo")
        .arg("run")
        .output()
        .expect("Cannot emergency sudo shutdown \n");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}

fn main() {
    let output = Command::new("sh")
        .arg("users/sudo.sh")
        .output()
        .expect("Cannot run sudo command shutting down to avoid sudo command from running \n")
        .expect(shutdown());

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}
