    .text
    .global main

main:
    sub sp, sp, #4
    str lr, [sp, #0]

    # prompt for input
    ldr r0, =prompt
    bl  printf

    # scan input
    ldr r0, =format
    sub sp, sp, #4
    mov r1, sp
    bl  scanf
    ldr r2, [sp, #0]
    add sp, sp, #4

    # print message
    mov r1, r2
    bl  printf

    ldr lr, [sp, #0]
    add sp, sp, #4
    mov pc, lr

    .data

format:
    .asciz "%d"

prompt:
    .asciz "\n"
