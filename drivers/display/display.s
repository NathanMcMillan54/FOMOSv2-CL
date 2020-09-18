// add stuff to only run this if connected to a screen
.globl _start
.extern LD_STACK_PTR

.section ".text.boot"

_start:
    ldr     x30, =LD_STACK_PTR
    mov     sp, x30
    bl      refresh_screen

.equ PSCI_SYSTEM_OFF, 0x84000008
.globl system_off
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0

