
# Overview

## cortex-m-rt-macros

Provides the `entry`, `exception`, `interrupt`, and `pre_init` macros that must annotate these special functions.

**NOTE:** The `cortex-m-rt-macros` source code is part of the `cortex-m-rt` crate's repository.

## cortex-m-rt

- Provides the runtime library. The runtime library is a static library containing the implementation of the Reset function and the HardFaultTrampoline function. It is hand-written as ARM assembly in Thumb mode.
- Provides a default linker script `link.x.in` that lays out the vector table, code, and data for the binary. The `lib.rs` library file provide default Rust implementations for the exceptions and interrupts.
- Provides a build script `build.rs` that passes the linker script to the linker and links the runtime library into the binary.

## SVD Files

ST Microelectronics provides CMSIS System View Description (SVD) files for each board that define the peripherals provided by each board. A SVD file is an XML file that lists each peripheral, it's associated registers, and the bit fields within each register. SVD files are validated using the CMSIS-SVD schema file.

## stm32-rs

The SVD files for each board supported by the stm32-rs project, fetched from [st.com](st.com), are found zipped up in the `svd/vendor` directory.

The `extract.sh` script in the `svd` directory extracts the zipped SVD files in the `vendor` subdirectory into the `svd` directory for processing.

One crate is generated for each device family. Within this crate, each supported device is implemented in a module that's only generated when that device's specific crate feature is enabled.

## svd2rust

`svd2rust` is a command line tool that transforms SVD files distributed by board vendors into rust crates known as Peripheral Access Crates (PACs). PACs expose a type safe API to access the device peripherals.

`svd2rust` supports the following architectures:

- Cortex-M
- MSP430
- RISCV
- Xtensa LX6

When targeting the Cortex-M architecture, `svd2rust` will generate three files in the current directory:

- build.rs, build script that places device.x somewhere the linker can find.
- device.x, linker script that weakly aliases all the interrupt handlers to the default exception handler (DefaultHandler).
- lib.rs, the generated code.

The resulting crate must provide an opt-in “rt” feature and depend on these crates: cortex-m, cortex-m-rt, and vcell. Furthermore the “device” feature of cortex-m-rt must be enabled when the “rt” feature is enabled. The Cargo.toml of the device crate will look like this:

```toml
[dependencies]
cortex-m = "0.7"
vcell = "0.1.2"

[dependencies.cortex-m-rt]
optional = true
version = "0.6.13"

[features]
rt = ["cortex-m-rt/device"]
```

The `svd2rust` documentation provides a good explanation of how to work with a board's peripherals using the generated rust code [here](https://docs.rs/svd2rust/latest/svd2rust/#peripheral-api).

## svd-rs

Library crate that provides types for describing the parts of an SVD file. It's used by svd-parser when parsing an SVD file.

**NOTE:** This crate lives in the same `svd` workspace as the `svd-parser` crate.

## svd-parser

Library crate that provides a `parse` function to parse an SVD file. The `parse` function takes in the SVD XML and return a `Device` holding the information parsed from the SVD file.

**NOTE:** This crate lives in the same `svd` workspace as the `svd-rs` crate.