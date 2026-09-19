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

// CH57x 家族(CH583 / CH572)的最小 BSP:轮询控制台 + PA 口输出。
// 没有独立板 feature —— 这两个芯片的 UART/PA 寄存器同址同偏移,一份够用。
#[cfg(any(feature = "ch583", feature = "ch572"))]
pub mod ch57x;
