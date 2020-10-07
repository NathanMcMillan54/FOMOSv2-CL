ENTRY(mystart)
SECTIONS
{
  . = 0x7c00;
  .text : {
    entry.o(.text)
    *(.text)
    *(.data)
    *(.rodata)
    __bss_start = .;
    *(.bss)
    *(COMMON)
    __bss_end = .;
  }
  .sig : AT(ADDR(.text) + 512 - 2)
  {
      SHORT(0xaa55);
  }
  /DISCARD/ : {
    *(.eh_frame)
  }
  __stack_bottom = .;
  . = . + 0x1000;
  __stack_top = .;
}