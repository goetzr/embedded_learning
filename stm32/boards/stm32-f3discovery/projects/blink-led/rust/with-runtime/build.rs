use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("Failed to get OUT_DIR environment variable"));
    File::create(out.join("memory.x"))
        .expect("Failed to create memory.x file")
        .write_all(include_bytes!("memory.x"))
        .expect("Failed to write memory.x file");

    println!("cargo:rerun-if-changed=memory.x");
}
