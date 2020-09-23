pub fn shutdown_arm() {
    global_asm!(r#"
    .global sleep

sleep:
      push    {ip, lr}             ; print shutdown
      ldr     r0, =shMsg
      bl      printf
      mov     r0, #0    @ Return 0.
      pop     {ip, pc}

      MRC   p15, 0, r0, c0, c0, 5  ; Read MPIDR
      ANDS  r0, r0, #3             ; Mask to leave CPU ID
    loop
      WFINE                        ; If not CPU 0, go into WFI in an endless loop
      BNE   loop
    "#);
    #[no_mangle]
    pub unsafe extern "C" fn baz() {}
}

extern "C" {
    fn msg();
    fn sleep();
}

pub fn shutdown_msg() {
    global_asm!("r#
        .syntax unified
    .global msg
    push    {ip, lr}             ; print shutdown
      ldr     r0, =shMsg
      bl      printf
      mov     r0, #0    @ Return 0.
      pop     {ip, pc}

      .asciz 'Shutting down...'
    ")
}
