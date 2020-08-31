#include "../include/system/config.h"

.globl get_current_el
get_current_el:
    mrs x0, currentel
    mov x1 #2
    lsr x0, x0 #2
    ret

.globl jump_to_el0
jump_to_el0:
    mov x1, x0
    mrs x0, currentel       // check if in el2
    cmp x0 #8
    beq 2

    // ldr x0, =0xe00000
    // msr sp_el0, x0          // init the stack of el0

    // return to el0 from el1
    mov x0 #0x0
    msr spsr_el1, x0        // el0 with DAIF = 0
    msr elr_el1, x1
    eret
2:
    ret

.globl tlb_invalidate
tlb_invalidate:
    dsb ishst               // ensure write has completed
    tlbi vmalle1is          // invalidate tlb, all asid, el1.
    dsb ish                 // ensure completion of tlb invalidation
    isb                     // synchronize context and ensure that no instructions
                            // are fetched using the old translation
    ret

.globl put32
put32:
    str w1, x0
    ret

.globl get32
get32:
    ldr w0,x0
    ret

.globl dummy
dummy:
    ret

.globl load_ttbr0_context
load_ttbr0_context:
    msr ttbr0_el1, x0
    ret

.globl load_ttbr1_context
load_ttbr1_context:
    msr ttbr1_el1, x0
    ret

.globl get_sctlr_el1
get_sctlr_el1:
    mrs x0, sctlr_el1
    ret

.globl get_esr
get_esr:
    mrs x0, esr_el1
    ret

.globl get_spsr
get_spsr:
    mrs x0, spsr_el1
    ret

.globl get_elr
get_elr:
    mrs x0, elr_el1
    ret

.globl get_far
get_far:
    mrs x0, far_el1
    ret
