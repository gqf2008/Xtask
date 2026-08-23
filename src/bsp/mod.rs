//! 板级支持包实现部分

#[cfg(all(feature = "gd32vf103", feature = "longan_nano"))]
pub mod longan_nano;

#[cfg(all(feature = "stm32f4", feature = "greenpill"))]
pub mod greenpill;

#[cfg(all(feature = "stm32f1", feature = "bluepill"))]
pub mod bluepill;

// rp-pico 板 feature 已并入 rp2040(BSP 用仓库自带 bsp_pins!,无官方 rp-pico 依赖)
#[cfg(feature = "rp2040")]
pub mod rp_pico;
