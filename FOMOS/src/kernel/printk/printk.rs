extern "C" {
    // call linux_printk() in printk.c
    fn linux_printk(argument: &str) -> &str;
}

pub fn printk(argument: &str) {
    unsafe {
        linux_printk(argument);
    }
}
