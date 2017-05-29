MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x80000
  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 0x10000
}

SECTIONS
{
  /* Set stack top to end of RAM */
  __StackTop = ORIGIN(RAM) + LENGTH(RAM);

  .text :
  {
    LONG(__StackTop);
    KEEP(*(.reset_handler));

    . = 0xD8;

    *(.text*)
  } > FLASH
}
