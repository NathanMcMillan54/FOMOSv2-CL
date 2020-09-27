use core::ptr;
// make support for US QWERTY keyboard first

pub(crate) fn arm_keyboard() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"keyboard.rs \n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    // run keyboard_connected.s
    // this is keyboard.s
    /* unsafe {
        asm!(".text
              .global keyboard

              keyboard:
                  sub sp, sp, #4
                  str lr, [sp, #0]

                  # get ready for input
                  ldr r0, =prompt
                  bl  printf

                  # read the input
                  ldr r0, =format
                  sub sp, sp, #4
                  mov r1, sp
                  bl  scanf
                  ldr r2, [sp, #0]
                  add sp, sp, #4

                  # print the input
                  mov r1, r2
                  bl  printf

                  ldr lr, [sp, #0]
                  add sp, sp, #4
                  mov pc, lr

                  .data

                  format:
                      .asciz "%d"

                  prompt:
                      .asciz " "");
    } */
}
