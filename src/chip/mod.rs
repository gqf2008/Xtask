//! 芯片移植模块

include!("env.rs");
#[cfg(feature = "gd32vf103")]
pub mod gd32vf103;
#[cfg(feature = "stm32f4")]
pub mod stm32f4;
