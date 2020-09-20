// put arm assembly here to run FOMOS.c
// compile this to img

.text
.global main
main:
        push {r7, lr}

        mov r0, #1
        ldr r1, =string
        mov r2, #12
        mov r7, #4
        svc #0

        pop {r7, pc}

.data
string: .asciz "kernel.img \nkernel.s \n"
