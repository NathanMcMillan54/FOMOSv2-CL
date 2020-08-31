ENTRY(_Reset)
SECTIONS
{
    . = 0x0
    .text : {
    bin/startup.o (INTERRUPT_VECTOR)
    *(.text)
    }
    .data : { *(.data) }
    .bss : { *(.bss COMMON) }
    . = ALIGN(8);
    . = . + 0x100; /* 4kB of stack memory */
    stack_top = .;
}