// put ARM to sleep mode
// this puts all the cores to sleep (should)
global _start_sleep

_start_sleep:
      MRC   p15, 0, r0, c0, c0, 5  ; Read MPIDR
      ANDS  r0, r0, #3             ; Mask to leave CPU ID
    loop
      WFINE                        ; If not CPU 0, go into WFI in an endless loop
      BNE   loop
