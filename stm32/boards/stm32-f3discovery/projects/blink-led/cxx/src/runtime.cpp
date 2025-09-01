// NOTE: These are symbols defined in the linker script, not variables, so it doesn't matter what type they are declared with here.
//       To get a symbol's value, its address must first be taken, which then must be cast to the appropriate type.
extern char __sidata;
extern char __sdata;
extern char __edata;

extern int main();

extern "C" void Reset() __attribute__ ((section (".text.runtime")));
void Reset() {
    // Initialize SRAM.
    auto sdata = reinterpret_cast<char*>(&__sdata);
    auto edata = reinterpret_cast<char*>(&__edata);
    auto sidata = reinterpret_cast<char*>(&__sidata);
    while (sdata != edata) {
        *sdata++ = *sidata++;
    }
   
    // Call main, which never returns.
    main();
}

extern "C" void DefaultHandler() __attribute__ ((section (".text.runtime")));
void DefaultHandler() {
    while (true);
}