// start all assembly files (setup.s, start.s)
    .syntax unified
.global main
main:
    push    {ip, lr}
    ldr     r0, =message
    bl      printf
    mov     r0, #0    @ Return 0.
    pop     {ip, pc}

message:
    .asciz "Starting setup...\n"
