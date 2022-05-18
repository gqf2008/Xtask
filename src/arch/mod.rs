//! 这个包是RUST官方嵌入式工作组项目的导出

#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64")))]
pub mod riscv;

#[cfg(target_arch = "arm")]
pub mod cortex_m;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
