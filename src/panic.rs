#[panic_handler]
fn on_panic(_info: &core::panic::PanicInfo) -> ! {
    panic!("There was a panic error");
    loop {  }
}
