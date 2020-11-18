.global _start
_start:
    mov $1, %rax
    mov $1, %rdi
    mov $start_message, %rsi
    mov $start_message_len, %rdx
    syscall
    jmp .
    start_message: .ascii "x86 FOMOSv2-CL v2.3.5\n"
    .equ start_message_len, . - start_message
