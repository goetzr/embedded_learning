# Build Script and Configuration File

It appears that options can be passed to rustc either via the build script (build.rs) or the configuration file (.cargo/config.toml).

For example, to set `cortex_m` for conditional compilation, you can do the following in the build script:

```rust
println!("cargo:rustc-cfg=cortex_m");
```

Alternatively, you can do the following in the configuration file:

```toml
[target.thumbv7em-none-eabihf]
rustflags = ["--cfg", "cortex_m"]
```

To pass the name of the linker script to the linker, you can do the following in the build script:

```rust
println!("cargo:rustc-link-arg=-Tlink.x");
```

Alternatively, you can do the following in the configuration file:

```toml
[target.thumbv7em-none-eabihf]
rustflags = ["-C", "link-arg=-Tlink.x"]
```

It appears that the linker to use can only be set from the configuration file:

```toml
[target.thumbv7em-none-eabihf]
linker = "/home/rgoetz/tools/gcc-arm-none-eabi-10.3-2021.10/bin/arm-none-eabi-ld"
```
