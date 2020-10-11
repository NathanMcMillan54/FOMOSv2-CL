ENTRY(_start)
SECTIONS
{
    . = 0x000060160;
    .text.boot : { *(.text.boot) }
    .text : { *(.text) }
    .data : { *(.data) }
    .rodata : { *(.rodata) }
    .bss : { *(.bss) }

    . = ALIGN(8);
    . = . + 0x60160;
    LD_STACK_PTR = .;
}
