# Using Board Support Crate

## Install Target Toolchain

Before you can cross compile for another architecture, you must install the architecture in rust. The `rustup` tool is used to do this.

To see the list of all target architectures, including those already installed, run the following command:

```bash
rustup target list
```

To install a new target architecture, run the following command:

```bash
rustup target add <target-name>
```

To install the target architecture for the STM32F303Discovery board, run the following command:

```bash
rustup target add thumbv7em-none-eabihf
```

## cargo.toml Project Configuration File

The dependencies listed below must be specified in your `cargo.toml` file.

```toml
# This is the board support crate.
# This form of specifying the dependency is necessary so we can enable the specified features.
[dependencies.stm32f3]
version = "0.14.0"
features = ["stm32f303", "rt"]

[dependencies]
# This crate provides the core peripherals.
cortex-m = "0.7.4"
# This crate provides the attributes used to mark the entry point and define ISRs.
cortex-m-rt = "0.7.1"
# Your application must provide a panic handler because we're not using the standard library.
# This crate provides a panic handler that executes an infinite loop.
panic-halt = "0.2.0"
```

## .cargo/config.toml Cargo Configuration File

This file provides special instructions to cargo when it's building your application. Create this file with the following contents:

```toml
[build]
# You must tell cargo the target architecture you're building for,
# otherwise it will default to your host's architecture.
target = "thumbv7em-none-eabihf"

rustflags = [
            # You must tell the linker to use the link.x linker script provided by the cortex-m-rt crate,
            # otherwise it will use the linker's default linker script.
            "-C", "link-args=-Tlink.x",
            # Remove this if you don't want the linker to print verbose diagnostic messages.
            # This is usually only necessary when debugging issues with the generated binary.
            # NOTE: The linker output is only shown if RUSTC_LOG is set as shown below.
             "-C", "link-arg=--verbose",
             # Remove this if you don't want the linker to generate a memory map file.
             # This is usually only necessary when debugging issues with the generated binary.
             "-C", "link-args=-Map link.map"]

[target.thumbv7em-none-eabihf]
# The default linker is the lld linker that ships with rust.
# If you want to use a different linker, you have to say so.
linker = "arm-none-eabi-ld"

[env]
# Show linker log statements at info level or above.
# This is usually only necessary when debugging issues with the generated binary.
RUSTC_LOG = "rustc_codegen_ssa::back::link=info"
```

## build.rs Build Script

The build script is run prior to building your application. Create this file with the following conents:

```rust
use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    // Copy the memory.x to the output directory so the linker can find it (it's included in the link.x linker script).
    const MEM_FILE: &'static str = "memory.x";
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not found"));
    let dst_path = out_dir.join(MEM_FILE);
    fs::copy(MEM_FILE, dst_path).expect("Failed to copy memory.x");

    // Add the output directory to the linker search path so the linker can find the memory.x file.
    println!("cargo:rustc-link-search={}", out_dir.display());

    // If the memory.x file is changed, the old one in the output directory should be replaced.
    println!("cargo:rerun-if-changed=memory.x");
}
```

## main.rs Source File

The structure of the `main.rs` file should be as follows:

```rust
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f3::stm32f303;

#[entry]
fn entry() -> ! {
    let peripherals = stm32f303::Peripherals::take().unwrap();
    // use peripherals for setup
    loop {
        // use peripherals for main loop
    }
}
```

## openocd.cfg OpenOCD Configuration file

The OpenOCD configuration file specifies the board/target/interface used to interface with your processor/board.

Use the following contents for the STM32F3Discovery board:

```bash
source [find interface/stlink.cfg]

source [find target/stm32f3x.cfg]
```

## Flashing / Debugging

From the terminal, start openocd in your project's top-level directory (where the openocd.cfg file is located):

```bash
openocd
```

In another terminal, navigate to the target/thumb7em-none-eabihf/debug or target/thumb7em-none-eabihf/release directory, then execute gdb as follows:

```bash
arm-none-eabi-gdb <application-name>
```

In gdb, connect to OpenOCD as follows:

```bash
target extended-remote :3333
```

To flash the image onto the board, execute the `load` command in gdb:

```bash
load
```

After the image is flashed onto the board, use gdb as normal to debug.