pub fn vprintk_func(argument: &[u8]) {

}

pub fn printk(argument: &[u8]) {

    let r;
    r = vprintk_func(argument);

    return r;
}
