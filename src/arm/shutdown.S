.globl arm_shutdown
arm_shutdown:
    ldr     x0, =0x84000008
    hvc     #0
