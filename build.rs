use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    #[cfg(feature = "gd32vf103")]
    fs::copy("src/chip/gd32vf103/memory.x", out_dir.join("memory.x")).unwrap();

    #[cfg(feature = "stm32f4")]
    fs::copy("src/chip/stm32f4/memory.x", out_dir.join("memory.x")).unwrap();

    println!("cargo:rerun-if-changed=memory.x");
}
