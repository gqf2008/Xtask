use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    #[cfg(feature = "gd32vf103")]
    fs::copy("src/chip/gd32vf103/memory.x", out_dir.join("memory.x")).unwrap();

    #[cfg(feature = "stm32f4")]
    fs::copy("src/chip/stm32f4/memory.x", out_dir.join("memory.x")).unwrap();

    #[cfg(feature = "stm32f1")]
    fs::copy("src/chip/stm32f1/memory.x", out_dir.join("memory.x")).unwrap();

    #[cfg(feature = "rp2040")]
    fs::copy("src/chip/rp2040/memory.x", out_dir.join("memory.x")).unwrap();

    #[cfg(feature = "stm32h7")]
    fs::copy("src/chip/stm32h7/memory.x", out_dir.join("memory.x")).unwrap();

    #[cfg(feature = "ch32v307")]
    fs::copy("src/chip/ch32v307/memory.x", out_dir.join("memory.x")).unwrap();
    #[cfg(feature = "ch32v203")]
    fs::copy("src/chip/ch32v203/memory.x", out_dir.join("memory.x")).unwrap();
    #[cfg(feature = "ch32v103")]
    fs::copy("src/chip/ch32v103/memory.x", out_dir.join("memory.x")).unwrap();
    #[cfg(feature = "esp32c3")]
    fs::copy("src/chip/esp32c3/memory.x", out_dir.join("memory.x")).unwrap();
    #[cfg(feature = "qemu_riscv")]
    fs::copy("src/chip/qemu_riscv/memory.x", out_dir.join("memory.x")).unwrap();
    #[cfg(feature = "qemu_arm_r52")]
    {
        fs::copy("src/chip/qemu_arm_r52/memory.x", out_dir.join("memory.x")).unwrap();
        // R5 无 runtime crate,链接脚本自备(RISC-V 口用 riscv-rt 的 link.x)
        fs::copy("src/chip/qemu_arm_r52/link.x", out_dir.join("link.x")).unwrap();
    }
    #[cfg(feature = "cm32m4")]
    fs::copy("src/chip/cm32m4/memory.x", out_dir.join("memory.x")).unwrap();

    // rerun-if-changed 指向真实被复制的源文件(指错路径时改 memory.x 不触发重建)
    println!("cargo:rerun-if-changed=src/chip/gd32vf103/memory.x");
    println!("cargo:rerun-if-changed=src/chip/rp2040/memory.x");
    println!("cargo:rerun-if-changed=src/chip/stm32f4/memory.x");
    println!("cargo:rerun-if-changed=src/chip/stm32f1/memory.x");
    println!("cargo:rerun-if-changed=src/chip/stm32h7/memory.x");
    println!("cargo:rerun-if-changed=src/chip/cm32m4/memory.x");
    println!("cargo:rerun-if-changed=src/chip/ch32v307/memory.x");
    println!("cargo:rerun-if-changed=src/chip/ch32v203/memory.x");
    println!("cargo:rerun-if-changed=src/chip/ch32v103/memory.x");
    println!("cargo:rerun-if-changed=src/chip/esp32c3/memory.x");
    println!("cargo:rerun-if-changed=src/chip/qemu_riscv/memory.x");
    println!("cargo:rerun-if-changed=src/chip/qemu_arm_r52/memory.x");
    println!("cargo:rerun-if-changed=src/chip/qemu_arm_r52/link.x");
}
