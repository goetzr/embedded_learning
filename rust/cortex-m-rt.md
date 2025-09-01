# cortex-m-rt Crate Notes

## Target Runtime Libraries

- The common source code for the runtime library for each target platform is in the `asm.S` assembly source file. Two functions are defined:
  
  - Reset
  - HardFaultTrampoline
  
- The `assemble.sh` bash script uses `arm-none-eabi-gcc` to build `asm.S` once for each target platform. The system `ar` is used to package each resulting object file into a static library. The resulting static libraries, which are the target runtime libraries, are stored in the `bin` directory.
- The CI script in `ci/script.sh` calls the `check-blobs.sh` script, which calculates the checksum of each runtime library, then calls the `assemble.sh` script to re-build the runtime libraries and re-calculate their checksums. A report is generated noting any differences in the checksums.

### Reset

- Runs the user-defined pre-init function marked with the `pre_init` attribute.
- Zero-initialize the `.bss` section.
- Copies the initialized data stored in the `.data` section from flash to RAM.
- Enables the FPU if present on the target device (this is specified in the `assemble.sh` script).
- Calls the user-defined entry point function marked with the `entry` attribute.

### HardFaultTrampoline

Calls the user-defined hard fault exception handler, which is named `HardFault` and marked with the `exception` attribute. This function is responsible for getting a pointer to the exception frame and passing it to the user-defined hard fault exception handler.

Page 40 in the Cortex-M4 User Guide states:

"In parallel to the stacking operation, the processor performs a vector fetch that reads the
exception handler start address from the vector table. When stacking is complete, the processor
starts executing the exception handler. At the same time, the processor writes an
EXC_RETURN value to the LR. This indicates which stack pointer corresponds to the stack
frame and what operation mode the processor was in before the entry occurred."

## Linker Script (link.x.in) and Rust Library (src/lib.rs)

Before reading on, be sure to read through [this](https://mcyoung.xyz/2021/06/01/linker-script/) article for a review of linker script concepts, specifically VMA / LMA and the placement of output sections in different memory regions.

### Initialization

The `memory.x` file defines the memory regions used by the program. If using a board support crate, it provides the `memory.x` file. Otherwise, you are responsible for providing it. It's included in the linker script using the `INCLUDE` command as follows:

```bash
# Linker script
INCLUDE memory.x
```

The `memory.x` file provided by the `cortex-m-rt` crate is not used by the crate itself. It's provided as an example from which you can create your own `memory.x` file.

The linker script expects the `__RESET_VECTOR` symbol to hold the address of the `Reset` function, which gets placed in the vector table. The `EXTERN` command is used to force the `__RESET_VECTOR` symbol to be entered into the output file as an undefined symbol. This ensures that during the link process the linker will go looking for this symbol in each input object file, exiting with an error if not found.

```bash
# Linker script
EXTERN(__RESET_VECTOR);
```

`__RESET_VECTOR` is defined in `lib.rs` as follows:

```rust
// lib.rs
#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".vector_table.reset_vector")]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
```

Notice that `__RESET_VECTOR` is placed in the `.vector_table.reset_vector` section and that it's declared public with the `no_mangle` attribute so the linker can find it.

`__RESET_VECTOR` is defined as a function pointer pointing to the `Reset` function, which is declared inside an `extern "C"` block elsewhere in the `lib.rs` file as follows:

```rust
// lib.rs
extern "C" {
  fn Reset() -> !;
  
  // Exception handler declarations follow...
}
```

The `Reset` function is defined in the `asm.S` file as described above. The `EXTERN` command is used to force the `Reset` symbol to be entered into the output file as an undefined symbol. This ensures that during the link process the linker will go looking for this symbol in each input object file, exiting with an error if not found.

```bash
# Linker script
EXTERN(Reset);
```

Next the `Reset` function is marked as the program's entry point using the `ENTRY` command. All symbols referenced by the entry point function and recursively by any of the functions it calls are linked into the output file.

```bash
# Linker script
ENTRY(Reset);
```

It's unclear why the `EXTERN` command is necessary for `Reset` if it's marked as the entry point; you'd think the linker would know to go looking for it.

The linker script expects the `__EXCEPTIONS` symbol to be an array holding the address of each exception handler. The `EXTERN` command is used to force the `__EXCEPTIONS` symbol to be entered into the output file as an undefined symbol. This ensures that during the link process the linker will go looking for this symbol in each input object file, exiting with an error if not found.

```bash
# Linker script
EXTERN(__EXCEPTIONS);
```

`__EXCEPTIONS` is defined as an array of `Vector` values in `lib.rs` as follows:

```rust
// lib.rs
#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".vector_table.exceptions")]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector {
        handler: NonMaskableInt,
    },
    // Exception 3: Hard Fault Interrupt.
    Vector {
        handler: HardFaultTrampoline,
    },
    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: MemoryManagement,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector { handler: BusFault },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: UsageFault,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // Exception 7: Secure Fault Interrupt [only on Armv8-M].
    #[cfg(armv8m)]
    Vector {
        handler: SecureFault,
    },
    #[cfg(not(armv8m))]
    Vector { reserved: 0 },
    // 8-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: DebugMonitor,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // 13: Reserved
    Vector { reserved: 0 },
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick },
];
```

Notice that `__EXCEPTIONS` is placed in the `.vector_table.exceptions` section and declared public with the `no_mangle` attribute so the linker can find it.

The `Vector` structure used in the definition of `__EXCEPTIONS` is defined in `lib.rs` as follows:

```rust
// lib.rs
#[doc(hidden)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}
```

This defines a `Vector` value as either a pointer to an unsafe function taking no arguments and returning nothing, or a reserved entry represented as a `usize` value. Each entry in `__EXCEPTIONS` is a `Vector` value that either points to an exception handler or is a reserved entry that's set to 0.

The exception handlers are declared in `lib.rs` as follows:

```rust
// lib.rs
extern "C" {
    fn Reset() -> !;

    fn NonMaskableInt();

    fn HardFaultTrampoline();

    #[cfg(not(armv6m))]
    fn MemoryManagement();

    #[cfg(not(armv6m))]
    fn BusFault();

    #[cfg(not(armv6m))]
    fn UsageFault();

    #[cfg(armv8m)]
    fn SecureFault();

    fn SVCall();

    #[cfg(not(armv6m))]
    fn DebugMonitor();

    fn PendSV();

    fn SysTick();
}
```

The exception handlers are defined in the linker script as follows:

```bash
# Linker script
PROVIDE(NonMaskableInt = DefaultHandler);
EXTERN(HardFaultTrampoline);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);
```

These lines use the `PROVIDE` command to provide a default definition for each exception handler that makes them aliases for the default exception handler. If the user defines any of these exception handlers in their own Rust code, that definition will override the default definition provided here.

The linker script expects `DefaultHandler` to hold the address of the default exception handler. The `EXTERN` command is used to force the `DefaultHandler` symbol to be entered into the output file as an undefined symbol. This ensures that during the link process the linker will go looking for this symbol in each input object file, exiting with an error if not found.

```bash
# Linker script
EXTERN(DefaultHandler);
```

The default definition for the `DefaultHandler` function is provided by the linker script using the `PROVIDE` command. This default definition makes `DefaultHandler` an alias for the `DefaultHandler_` function.

```bash
# Linker script
PROVIDE(DefaultHandler = DefaultHandler_);
```

The `DefaultHandler_` function is defined in `lib.rs` as an infinite loop:

```rust
// lib.rs
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
```

Notice that `DefaultHandler_` is declared public with the `no_mangle` attribute so the linker can find it.

If the user provides their own default handler, which must be named `DefaultHandler` and marked with the `exception` attribute, this overrides the default definition provided by the linker script.

Notice above where the linker script provides default definitions for the exception handlers that the hard fault handler is treated specially. The hard fault handler is set to the `HardFaultTrampoline` function in the `__EXCEPTION` array. Recall that the `HardFaultTrampoline` function is defined in `asm.S`. It's definition gets a pointer to the exception frame and passes it to the `HardFault` function. The `ExceptionFrame` structure defined in `lib.rs` represents an exception frame.

```rust
// lib.rs
/// Registers stacked (pushed onto the stack) during an exception.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}
```

Getters and setters are provided for each of the registers in the exception frame.

Recall that the `EXTERN` command had to be used with the default handler function to ensure that during the link process the linker will go looking for this symbol in each input object file, exiting with an error if not found.

```bash
# Linker script
EXTERN(DefaultHandler);
```

This is necessary for the default handler function because it's not called by any of the code provided by the `cortex-m-rt` crate, so the linker doesn't know to look for it and link it into the output file. However, the hard fault handler `HardFault` is called directly by the `HardFaultTrampoline` function, so no `EXTERN` command is necessary in the linker script.

The `PROVIDE` command is used in the linker script to provide a default definition for the `HardFault` function. This default definition makes the `HardFault` function an alias for the `HardFault_` function.

```bash
# Linker script
PROVIDE(HardFault = HardFault_);
```

The `HardFault_` function is defined in `lib.rs` as an infinite loop:

```rust
// lib.rs
#[allow(unused_variables)]
#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".HardFault.default")]
#[no_mangle]
pub unsafe extern "C" fn HardFault_(ef: &ExceptionFrame) -> ! {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
```

Notice that `HardFault_` is placed in the `.HardFault.default` section and declared public with the `no_mangle` attribute so the linker can find it.

If the user provides their own hard fault handler, which must be named `HardFault` and marked with the `exception` attribute, this overrides the default definition provided by the linker script.

The linker script expects the `__INTERRUPTS` symbol to be an array holding the address of each ISR. The `EXTERN` command is used to force the `__INTERRUPTS` symbol to be entered into the output file as an undefined symbol. This ensures that during the link process the linker will go looking for this symbol in each input object file, exiting with an error if not found.

```bash
# Linker script
EXTERN(__INTERRUPTS);
```

`__INTERRUPTS` is defined as an array of function ponters in `lib.rs` as follows:

```rust
// lib.rs
// If we are not targeting a specific device we bind all the potential device specific interrupts
// to the default handler
#[cfg(all(any(not(feature = "device"), test), not(armv6m)))]
#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".vector_table.interrupts")]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 240] = [{
    extern "C" {
        fn DefaultHandler();
    }

    DefaultHandler
}; 240];
```

Notice that `__INTERRUPTS` is placed in the `.vector_table.interrupts` section and declared public with the `no_mangle` attribute so the linker can find it.

Each entry in the `__INTERRUPTS` array is defaulted to point to the DefaultHandler function.

The `Reset` function defined in `asm.S` expects there to be a function named `__pre_init` that performs any necessary initializations prior to memory being initialized. The `PROVIDE` command is used in the linker script to provide a default implementation of the `__pre_init` function. This default implementation makes the `__pre_init` function an alias for the `DefaultPreInit` function.

```bash
# Linker script
PROVIDE(__pre_init = DefaultPreInit);
```

The `DefaultPreInit` function is defined as an empty function in `lib.rs` as follows:

```rust
// lib.rs
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}
```

Notice that `DefaultPreInit` is declared public with the `no_mangle` attribute so the linker can find it.

The user can override the default implementation of the `__pre_init` function by writing their own pre-init function, which can be named anything as long as it's marked with the `pre_init` attribute.

### Output Sections

The `.text` section, which contains executable code, is defined as follows:

```bash
# Linker script
.vector_table ORIGIN(FLASH) :
  {
    /* Initial Stack Pointer (SP) value */
    LONG(_stack_start);

    /* Reset vector */
    KEEP(*(.vector_table.reset_vector)); /* this is the `__RESET_VECTOR` symbol */
    __reset_vector = .;

    /* Exceptions */
    KEEP(*(.vector_table.exceptions)); /* this is the `__EXCEPTIONS` symbol */
    __eexceptions = .;

    /* Device specific interrupts */
    KEEP(*(.vector_table.interrupts)); /* this is the `__INTERRUPTS` symbol */
  } > FLASH
```

The `LONG` command is used to place a 32-bit value holding the address of the start of the stack as the first entry in the vector table.

The `KEEP` command is used to ensure the custom sections created to hold the reset vector, exception handlers, and ISRs are not discarded by the linker.

The `.data` section, which contains initialized global/static data, is defined as follows:

```bash
# Linker script
.data : ALIGN(4)
{
  . = ALIGN(4);
  __sdata = .;
  *(.data .data.*);
  . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
} > RAM AT>FLASH
. = ALIGN(4);
__edata = .;

/* LMA of .data */
__sidata = LOADADDR(.data);
```

The placement expression **"> RAM AT>FLASH"** means the VMA of the `.data` section is placed in the RAM memory region, but the LMA of the `.data` section is placed in the FLASH memory region. This means the `.data` section is loaded into FLASH, and **must be manually copied** into RAM before any variables in the `.data` section are used.

As discussed above, the `Reset` function defined in `asm.S` performs this copy. The `__sdata` and `__edata` symbols hold the start and end addresses, respectively, of where the `.data` section is expected to be located in RAM at runtime. The `__sidata` symbol holds the address where the `.data` section is loaded into FLASH. The `Reset` function uses `__sidata` as the source of the copy operation and `__sdata` as the destination. `__edata` is used to determine the number of bytes to copy.

The `.bss` section, which contains uninitialized global/static data, is defined as follows:

```bash
# Linker script
.bss (NOLOAD) : ALIGN(4)
{
  . = ALIGN(4);
  __sbss = .;
  *(.bss .bss.*);
  *(COMMON); /* Uninitialized C statics */
  . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
} > RAM
. = ALIGN(4);
__ebss = .;
```

Notice that the `.bss` section is marked as `NOLOAD`. This is important because the contents of the `.bss` section should not be loaded into FLASH. The placement expression **"> RAM"** places the VMA of the `.bss` section in RAM instead of FLASH.

The `.bss` section must be zeroed at runtime prior to accessing any of the variables it contains. As discussed above, the `Reset` function defined in `asm.S` fills the `.bss` section in RAM with zeros.

Static uninitialized variables are placed in the `.uninit` section. See the discussion of static uninitialized variables in `lib.rs` for a good explanation along with a code example. The `.uninit` section is defined as follows:

```bash
# Linker script
.uninit (NOLOAD) : ALIGN(4)
{
  . = ALIGN(4);
  __suninit = .;
  *(.uninit .uninit.*);
  . = ALIGN(4);
  __euninit = .;
} > RAM
```

Notice that the `.uninit` section is marked as `NOLOAD`. The variables in this section are uninitialized, just like the `.bss` section, so there's nothing to load. However, unlike the `.bss` section, the `.uninit` section is not zeroed by the `Reset` function at runtime. In fact, no runtime processing of the `.uninit` section is performed at all. As explained in `lib.rs`, the `.uninit` section is designed to hold large, uninitialized static variables. Initializing them in any way would increase the bootup time unnecessarily.

The heap is placed directly after all other sections in RAM. `__sheap`, which holds the start address of the heap in RAM, is defined as follows:

```bash
# Linker script
PROVIDE(__sheap = __euninit);
```

The `__sheap` symbol is used by the `heap_start` function defined in `lib.rs` to return the address of the start of the heap in RAM.

```rust
// lib.rs
#[inline]
pub fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __sheap: u32;
    }

    unsafe { &mut __sheap }
}
```

The `cortex-m-rt` crate does not use the `heap_start` function. I'm guessing that an allocator must be bolted on top of the runtime to use the heap.

## Build Script

1. Extracts the target triple from the `TARGET` environment variable.
2. Extracts the output directory from the `OUT_DIR` environment variable.
3. If the target architecture starts with **thumbv** (which it always should), copies the architecture-specific runtime library file from the `bin` directory to the output directory, where the linker can find it, renaming it to `libcortex-m-rt.a`. It then writes `cargo:rustc-link-lib=static=cortex-m-rt` to standard out, instructing cargo to link in the runtime library when building.
4. Copies the contents of the linker script `link.x.in` into the file `link.x` in the output directory where the linker can find it. If the `device` feature is enabled, as a board support crate would do, an `INCLUDE(device.x)` linker script command is appended to `link.x` so that the contents of the `device.x` linker script are included when the linker processes the `link.x` file. The `cargo/.config` configuration file passes the `["-C", "link-arg=-Tlink.x"]` codegen option in `rustflags` so that the linker uses the `link.x` linker script file.
5. Writes `cargo:rustc-cfg=...` lines to standard out to define the `cortex_m` and `armv6m` / `armv7m` / `armv8m` conditional compilation definitions as appropriate for the target architecture.
6. Appends an `ASSERT` command to the `link.x` linker script that checks the size of the vector table.
7. Writes the following command to standard out so that the linker can find the runtime library `libcortex-m-rt.a` in the output directory.

```rust
// 'out' holds the path to the output directory.
println!("cargo:rustc-link-search={}", out.display());
```

## Custom Target Specification JSON File

Rust provides a built-in list of target specifications that are viewable via `rustc --print target-list`. A custom target specification JSON file may be written to override and tweak various target-specific options, such as linker scripts, flags, and LLVM options.

The target specification JSON file:

- May live in any directory - most often, committed to version control.
- Any file name you like, so long as it's JSON
- Is given to Rust via the `--target my-target-spec.json`. For example, `cargo build --target ./targets/my-target-spec.json.

See [here](https://book.avr-rust.com/005.1-the-target-specification-json-file.html) for an example.
