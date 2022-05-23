//! 板级支持包实现部分

#[cfg(all(feature = "gd32vf103", feature = "longan_nano"))]
pub mod longan_nano;

#[cfg(all(feature = "stm32f4", feature = "greenpill"))]
pub mod greenpill;

#[cfg(all(feature = "stm32f1", feature = "bluepill"))]
pub mod bluepill;

#[cfg(all(feature = "rp2040", feature = "rp-pico"))]
pub mod rp_pico;
