// put ARM to sleep mode
// this puts all the cores to sleep (should)
.syntax unified
.global main

main:
      push    {ip, lr}             ; print shutdown
      ldr     r0, =printShutdown
      bl      printf
      mov     r0, #0    @ Return 0.
      pop     {ip, pc}

      MRC   p15, 0, r0, c0, c0, 5  ; Read MPIDR
      ANDS  r0, r0, #3             ; Mask to leave CPU ID
    loop
      WFINE                        ; If not CPU 0, go into WFI in an endless loop
      BNE   loop


printShutdown:
    .asciz "Shutting down... \n"
