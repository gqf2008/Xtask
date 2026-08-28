//! 芯片移植模块

include!("env.rs");
#[cfg(feature = "cm32m4")]
pub mod cm32m4;
#[cfg(feature = "ch32v307")]
pub mod ch32v307;
#[cfg(feature = "ch32v203")]
pub mod ch32v203;
#[cfg(feature = "ch32v103")]
pub mod ch32v103;
#[cfg(feature = "esp32c3")]
pub mod esp32c3;
#[cfg(feature = "qemu_riscv")]
pub mod qemu_riscv;
#[cfg(feature = "qemu_arm_r52")]
pub mod qemu_arm_r52;
#[cfg(feature = "gd32vf103")]
pub mod gd32vf103;
#[cfg(feature = "rp2040")]
pub mod rp2040;
#[cfg(feature = "stm32f1")]
pub mod stm32f1;
#[cfg(feature = "stm32f4")]
pub mod stm32f4;
#[cfg(feature = "stm32h7")]
pub mod stm32h7;
