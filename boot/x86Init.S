.global _start
extern x86_start

_start:
    mov $1, %rax
    mov $1, %rdi
    mov $msg, %rsi
    mov $msg_len, %rdx
    syscall
    jmp .
    message: .ascii "x86 FOMOSv2-CL v2.3.5\n"
    .equ msg_len, . - msg


x86Boot:
    pushad
    call x86_start
    popad
    iret