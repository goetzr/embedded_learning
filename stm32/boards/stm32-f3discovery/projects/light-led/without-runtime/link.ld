MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}

EXTERN(Reset);
ENTRY(Reset);

SECTIONS
{
  PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

  .vector_table ORIGIN(FLASH) :
  {
    LONG(_stack_start);
    LONG(Reset);
  } > FLASH

  .text ORIGIN(FLASH) + 0x1020 :
  {
    *(.text .text.*);
    . = ALIGN(4);
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx);
  }
}
