//! Cortex®-A/R 系列 ARMv7-A/R 指令集架构。
//!
//! 与 cortex_m(thumbv7em 等 M 核,依赖 cortex-m-rt 运行时)不同,
//! ARMv7-R(R4/R5 等)生态没有成熟的 `cortex-r-rt` crate——启动代码、
//! 向量表、链接脚本全部由 chip 口自备(port.S + link.x)。因此本模块
//! 不 re-export 运行时 crate,仅作为 arch 层的占位(例程不从这里取
//! `#[entry]`,而是直接定义 `#[no_mangle] extern "C" fn main`)。

// 注:target_arch = "arm" 同时覆盖 M 核与 A/R 核,arch/mod.rs 按
// feature(qemu_arm_r52)分流;M 核口仍走 cortex_m 模块。
