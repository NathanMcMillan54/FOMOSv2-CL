// add stuff to look for event.
// it might be easier to do this in rust but for now it will probably be done is assembly.
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
string: .asciz "From keyboard.s \n"
