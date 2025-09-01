MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM   : ORIGIN = 0x20000000, LENGTH = 40K
}

EXTERN(Reset);
ENTRY(Reset);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* Initial Stack Pointer (SP) value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Reset vector */
    KEEP(*(.vector_table.reset_vector));
    __reset_vector = .;

    /* Exceptions */
    KEEP(*(.vector_table.exceptions));
    __eexceptions = .;

    /* Device specific interrupts */
    KEEP(*(.vector_table.interrupts));
  } > FLASH

  /* ### .text */
  PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));
  .text _stext :
  {
    __stext = .;
    *(.text .text.*);

    . = ALIGN(4); /* Pad .text to the alignment to workaround overlapping load section bug in old lld */
    __etext = .;
  } > FLASH

  /* ### .rodata */
  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    __srodata = .;
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  /* ## Sections in RAM */
  /* ### .data */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  } > RAM AT>FLASH
  /* Allow sections from user `memory.x` injected using `INSERT AFTER .data` to
   * use the .data loading mechanism by pushing __edata. Note: do not change
   * output region or load region in those user sections! */
  . = ALIGN(4);
  __edata = .;

  /* LMA of .data */
  __sidata = LOADADDR(.data);

  /* ### .gnu.sgstubs
     This section contains the TrustZone-M veneers put there by the Arm GNU linker. */
  /* Security Attribution Unit blocks must be 32 bytes aligned. */
  /* Note that this pads the FLASH usage to 32 byte alignment. */
  .gnu.sgstubs : ALIGN(32)
  {
    . = ALIGN(32);
    __veneer_base = .;
    *(.gnu.sgstubs*)
    . = ALIGN(32);
    __veneer_limit = .;
  } > FLASH

  /* ### .bss */
  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON); /* Uninitialized C statics */
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  } > RAM
  /* Allow sections from user `memory.x` injected using `INSERT AFTER .bss` to
   * use the .bss zeroing mechanism by pushing __ebss. Note: do not change
   * output region or load region in those user sections! */
  . = ALIGN(4);
  __ebss = .;

  /* ## .got */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

/* Do not exceed this mark in the error messages below                                    | */
/* # Alignment checks */
ASSERT(ORIGIN(FLASH) % 4 == 0, "
ERROR(asm-only): the start of the FLASH region must be 4-byte aligned");

ASSERT(ORIGIN(RAM) % 4 == 0, "
ERROR(asm-only): the start of the RAM region must be 4-byte aligned");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
BUG(asm-only): .data is not 4-byte aligned");

ASSERT(__sidata % 4 == 0, "
BUG(asm-only): the LMA of .data is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
BUG(asm-only): .bss is not 4-byte aligned");

/* # Position checks */

/* ## .vector_table */
ASSERT(__reset_vector == ADDR(.vector_table) + 0x8, "
BUG(asm-only): the reset vector is missing");

ASSERT(__eexceptions == ADDR(.vector_table) + 0x40, "
BUG(asm-only): the exception vectors are missing");

ASSERT(SIZEOF(.vector_table) > 0x40, "
ERROR(asm-only): The interrupt vectors are missing");

/* ## .text */
ASSERT(ADDR(.vector_table) + SIZEOF(.vector_table) <= _stext, "
ERROR(asm-only): The .text section can't be placed inside the .vector_table section
Set _stext to an address greater than the end of .vector_table (See output of `nm`)");

ASSERT(_stext + SIZEOF(.text) < ORIGIN(FLASH) + LENGTH(FLASH), "
ERROR(asm-only): The .text section must be placed inside the FLASH memory.
Set _stext to an address smaller than 'ORIGIN(FLASH) + LENGTH(FLASH)'");

/* # Other checks */
ASSERT(SIZEOF(.got) == 0, "
ERROR(asm-only): .got section detected in the input object files
Dynamic relocations are not supported. If you are linking to C code compiled using
the 'cc' crate then modify your build script to compile the C code _without_
the -fPIC flag. See the documentation of the `cc::Build.pic` method for details.");
/* Do not exceed this mark in the error messages above                                    | */
