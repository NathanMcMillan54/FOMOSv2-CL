.text
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
    .asciz "%d \n"

prompt:
    .asciz "$user"
