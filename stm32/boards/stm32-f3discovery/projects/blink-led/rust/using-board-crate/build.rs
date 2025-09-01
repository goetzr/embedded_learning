use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    const MEM_FILE: &'static str = "memory.x";
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not found"));
    let dst_path = out_dir.join(MEM_FILE);
    fs::copy(MEM_FILE, dst_path).expect("Failed to copy memory.x");

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");
}