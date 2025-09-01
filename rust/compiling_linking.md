# Compiling and Linking

To show the rustc invocation, pass the `-v` option to cargo to perform a verbose build. Be sure to execute `cargo clean` first so `rustc` is actually run.

```bash
cargo clean
cargo -v
```

To show the linker invocation and output, execute the following.

```bash
export RUSTC_LOG=rustc_codegen_ssa::back::link=info
cargo clean
cargo build -v
```

This suggestion came from [this](https://github.com/rust-lang/rust/issues/38206) rust issue.

As explained [here](https://longfangsong.github.io/rustc-dev-guide-cn/compiler-debugging.html), the rustc compiler contains lots of debug statements, which may print out logging information at many points during compilation. The `RUSTC_LOG` environment variable is used to specify which modules inside rustc you'd like to enable logging for and at what level. When we set `RUSTC_LOG` to `rustc_codegen_ssa::back::link=info` above, we're telling rustc to enable logging for the `rustc_codegen_ssa::back::link` module at the *info* level and above.

Unfortunately, the default linker, which is the LLD linker shipped with Rust, does not show the contents of the linker script as part of its verbose output. It's sometimes nice to verify that the linker is actually using the linker script you specified by having it print out the contents of the linker script it's using. To do this, we have to use the GCC-ARM linker instead of the LLD linker.

As [this](https://blog.rust-embedded.org/2018-08-2x-psa-cortex-m-breakage/) post explains, the default linker used to be the GCC-ARM linker but was changed to the LLD linker that ships with rust.

To use the GCC-ARM linker, specify the following in the configuration file (`.cargo/config.toml`):

```toml
[target.thumbv7em-none-eabihf]
linker = "/home/rgoetz/tools/gcc-arm-none-eabi-10.3-2021.10/bin/arm-none-eabi-ld"
```

To manually pass extra options to `rustc` on the cargo command line, execute `cargo rustc`. The extra `rustc` options are specified after the `--`.

To keep the intermediate object file around for inspection, pass the `--emit=obj` option to `rustc'.

```bash
cargo rustc -- --emit=obj
```

Cargo always passes `--emit=dep-info,link` to `rustc`, so right now it's not possible using cargo to compile but not link. The arguments to the `--emit` option passed to `rustc` are ORed together. So if I execute:

```bash
cargo rustc -- --emit=obj,asm
```

both the object files and assembly files are generated, along with the final linked executable.

If you ever get confused about where input sections are going, use the ‘-M’ linker option to generate a map file. The map file shows precisely how input sections are mapped to output sections.
