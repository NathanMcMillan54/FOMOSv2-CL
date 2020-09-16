use std::{thread, time};

fn main() {
    let one_second = time::Duration::from_millis(1000);

    print!("Setting up FOMOS 3/4... \n");

    let mut power = 1;
    if power = 0 {
        print!("Power = 0 \n");
        thread::sleep(one_second);
        /* ../system/sleep.s */
    } else if power > 1 {
        print!("Error: power > 1 \n");
        thread::sleep(one_second);
        /* ../system/sleep.s */
    } else if power < 0 {
        print!("Error: power < 0 \n");
        thread::sleep(one_second);
        /* ../system/sleep.s */
    }
    /* Start main.c */
}
