use crate::printk;

fn update_boot_count() {
    // For now until fs is supported
    let mut boot_count = 0;

    boot_count += 1;
    printk!("Starting {}th boot", boot_count);
}

pub fn setup() {
    update_boot_count();
}

