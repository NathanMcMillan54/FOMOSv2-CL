.extern exit, puts
.data
    msg: .asciz "FOMOSv2-CL"
.text
.global main
main:
    /* r0 is the first argument. */
    ldr r0, msg
    bl puts
    mov r0, 0
    bl exit