fn arm() {
    // something that only runs on arm
}

fn x86_64() {
    // something that only runs on x86_64
}

pub(crate) fn check_cpu_type() {
    arm();
    x86_64();
}
