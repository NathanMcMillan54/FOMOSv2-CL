// call FOMOS.c
    .syntax unified
.global main
main:
    push    {ip, lr}
    ldr     r0, =message
    bl      printf
    mov     r0, #0    @ Return 0.
    pop     {ip, pc}

message:
    .asciz "Starting... \n"
