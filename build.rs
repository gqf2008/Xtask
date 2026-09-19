use std::path::PathBuf;
use std::{env, fs};

/// 芯片接线守卫:`src/chip/<名字>/` 存在、且 `<名字>` feature 被启用时,
/// 该 feature 必须在 `src/port.rs` 的 `chip_portings!` 清单里登记。
///
/// 漏登记的后果不是编译错误而是**静默降级**:`Porting` 落到兜底桩
/// `DefaultPorting`(各方法 `unimplemented!()`),构建与门禁全绿,直到运行期
/// `xtask::start()` 才 panic(2026-09-18 ch583 骨架 PR 实测踩过,见 issue #14 /
/// PR #15)。这里用"目录 ⇄ feature ⇄ 登记表"三方交叉把它变成构建期失败。
///
/// 阳性对照:`Cargo.toml` 开 `--features ch583` 后删掉 `src/port.rs` 里
/// `"ch583" => ...` 那一行,构建必须报本守卫(而不是编过)。
fn check_chip_porting_wiring(manifest_dir: &str) {
    let chip_dir = PathBuf::from(manifest_dir).join("src/chip");
    let port_rs = fs::read_to_string(PathBuf::from(manifest_dir).join("src/port.rs"))
        .expect("读不到 src/port.rs(芯片接线守卫需要它)");
    let mut missing = Vec::new();
    for entry in fs::read_dir(&chip_dir).expect("读不到 src/chip/") {
        let entry = entry.expect("src/chip/ 条目读取失败");
        if !entry.file_type().expect("条目类型读取失败").is_dir() {
            continue; // env.rs / mod.rs 等文件跳过
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        // 该芯片 feature 是否启用(cargo 把启用的 feature 变成 CARGO_FEATURE_*)
        let env_key = format!("CARGO_FEATURE_{}", name.to_uppercase().replace('-', "_"));
        if env::var_os(&env_key).is_none() {
            continue;
        }
        if !port_rs.contains(&format!("\"{name}\" =>")) {
            missing.push(name);
        }
    }
    if !missing.is_empty() {
        panic!(
            "芯片 feature {missing:?} 未在 src/port.rs 的 chip_portings! 清单登记 —— \
             Porting 会静默落到 DefaultPorting(unimplemented!),运行期才 panic"
        );
    }
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    check_chip_porting_wiring(&manifest_dir);

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
    #[cfg(feature = "ch583")]
    fs::copy("src/chip/ch583/memory.x", out_dir.join("memory.x")).unwrap();
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
    // src/port.rs 与 src/chip/* 供上面的接线守卫:改动必须触发本脚本重跑
    println!("cargo:rerun-if-changed=src/port.rs");
    println!("cargo:rerun-if-changed=src/chip");
    println!("cargo:rerun-if-changed=src/chip/gd32vf103/memory.x");
    println!("cargo:rerun-if-changed=src/chip/rp2040/memory.x");
    println!("cargo:rerun-if-changed=src/chip/stm32f4/memory.x");
    println!("cargo:rerun-if-changed=src/chip/stm32f1/memory.x");
    println!("cargo:rerun-if-changed=src/chip/stm32h7/memory.x");
    println!("cargo:rerun-if-changed=src/chip/cm32m4/memory.x");
    println!("cargo:rerun-if-changed=src/chip/ch32v307/memory.x");
    println!("cargo:rerun-if-changed=src/chip/ch32v203/memory.x");
    println!("cargo:rerun-if-changed=src/chip/ch32v103/memory.x");
    println!("cargo:rerun-if-changed=src/chip/ch583/memory.x");
    println!("cargo:rerun-if-changed=src/chip/esp32c3/memory.x");
    println!("cargo:rerun-if-changed=src/chip/qemu_riscv/memory.x");
    println!("cargo:rerun-if-changed=src/chip/qemu_arm_r52/memory.x");
    println!("cargo:rerun-if-changed=src/chip/qemu_arm_r52/link.x");
}
