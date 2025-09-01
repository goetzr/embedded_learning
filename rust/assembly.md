# Call Frame Information (CFI)

CFI provides additional debugging tables necessary to unwind the stack when a standard function prologue is not used. CFI is explained well in [this](https://www.imperialviolet.org/2017/01/18/cfi.html) article.

Use `objdump` to dump the CFI frames in an object file as follows:

```bash
cargo-objdump -- --dwarf=frames
```

-OR-

```bash
rust-objdump --dwarf=frames <path-to-binary>
```
