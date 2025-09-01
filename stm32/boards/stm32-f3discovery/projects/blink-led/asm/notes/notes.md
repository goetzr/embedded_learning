# Assembly Notes

## General

- See the GNU as tips PDF located [here](https://course.ece.cmu.edu/~ee349/f-2012/lab2/gas-tips.pdf) that I've downloaded to my `Documents/STMF32Discovery` folder.
- Use the `ldr <reg>, =<value>` pseudo instruction to load a 32-bit value into a register.
- Precede a symbol name with `.L` to make it a local symbol.
- Using registers above `r7` may force the assembler to use 32-bit instructions. For example, `ldr r0, [r1]` is a 16-bit instruction, but `ldr r0, [r10]` is a 32-bit instruction.
- A label is different than a symbol. A label is a symbol followed by a `:`, and holds the address of a place in memory (data or code). A label is by default local, but can be made global by using the `.global` directive. A symbol is a named value, typically used to hold magic numbers. By default a symbol is public. A symbol can be made local by preceding its name with `.L`.
- Much of the GNU as documentation uses `@` when specifying certain arguments to directives. However, `@` is the comment symbol for ARM assembly. Use `%` instead.
- What are the `$a`, `$t`, `$d` symbols? They are defined [here](https://sourceware.org/binutils/docs/as/ARM-Mapping-Symbols.html). They only show up when you accidentally use `nm` instead of `arm-none-eabi-nm`.
- Pass the `--print-map` option to `arm-none-eabi-ld` to debug linker script issues. It shows you exactly where it's placing the different input sections in the output sections.
- What does `*fill*` mean in the map dumped by the linker with the `--print-map` option? This marks gaps created by the linker to align input sections to their desired alignment. Run `arm-none-eabi-objdump --section-headers <input-object-file>` and look at the `Algn` column to see each input section's alignment. See the StackOverflow post [here](https://stackoverflow.com/questions/26552263/what-is-fill-section-shows-in-the-link-map-file).
- The `.align` directive on ARM takes the number of low-order zero bits in the byte alignment. For example, `.align 3` forces an alignment of 2^3 = 8 bytes.
- To get function return working properly, I had to mark each function with `.thumb_func` and use the `bx lr` instruction.

## GDB
- Use `arm-none-eabi-gdb`.
- Commands in order:
  - target extended-remote :3333
  - load
  - b main
  - c

## Call Frame Information (CFI)

- Part of the DWARF debugging specification, which is found [here](https://dwarfstd.org/doc/DWARF4.pdf).
