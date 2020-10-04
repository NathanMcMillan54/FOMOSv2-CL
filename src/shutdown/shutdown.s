// put ARM to sleep mode (shutdown)
// this should put all the cores to sleep
    .syntax unified
.global sleep

sleep:
      push    {ip, lr}             ; print shutdown
      ldr     r0, =shMsg
      bl      printf
      mov     r0, #0    @ Return 0.
      pop     {ip, pc}

      MRC   p15, 0, r0, c0, c0, 5  ; Read MPIDR
      ANDS  r0, r0, #3             ; Mask to leave CPU ID
    loop
      WFINE                        ; If not CPU 0, go into WFI in an endless loop
      BNE   loop


shMsg:
    .asciz "Shutting down... \n"
