//! 芯片移植模块

include!("env.rs");
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
