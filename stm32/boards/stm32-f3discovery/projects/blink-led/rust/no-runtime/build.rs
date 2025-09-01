use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let out_dir_path = env::var("OUT_DIR").expect("OUT_DIR not found");
    let out_dir_path = Path::new(&out_dir_path);
    let linker_script_dst_path = out_dir_path.join("link.ld");
    fs::copy("link.ld", linker_script_dst_path).expect("Copy linker script failed");

    println!("cargo:rerun-if-changed=link.ld");
}