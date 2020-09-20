// this only works on x86 but it's a good way to satrt i guess

.code16
.global _start
_start:
    cli
    xor %ax, %ax
    mov %ax, %ds
    mov $msg, %si
    mov $0x0e, %ah
loop:
    lodsb
    or %al, %al
    jz halt
    int $0x10
    jmp loop
halt:
    hlt
msg:
    .asciz "Starting... \n"
.org 510
.word 0xaa55

// make this work on ARM
