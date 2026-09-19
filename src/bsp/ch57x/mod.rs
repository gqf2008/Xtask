//! CH57x(WCH CH583 / CH572)最小 BSP。
//!
//! 只做两件事(够上板核对用,不做时钟树/外设全家桶):
//! - `uart`:轮询控制台(TX),寄存器与波特率公式照官方 EVT 源码;
//! - PA 口输出:给 LED / 示波器打点用。
//!
//! 寄存器依据:`PA_DIR @0x400010A0 / PA_PIN @+04 / PA_OUT @+08 / PA_CLR @+0C`
//! (官方 `CH583SFR.h`、`CH572SFR.h`,两芯片同址)。

pub mod uart;

const PA_DIR: usize = 0x4000_10A0;
const PA_OUT: usize = 0x4000_10A8;
const PA_CLR: usize = 0x4000_10AC;

/// 把 `mask` 里的 PA 脚设为输出(0=输入,1=输出)
#[inline]
pub fn pa_enable_output(mask: u32) {
    let dir = PA_DIR as *mut u32;
    unsafe { dir.write_volatile(dir.read_volatile() | mask) }
}

/// PA 脚置高
#[inline]
pub fn pa_set(mask: u32) {
    let out = PA_OUT as *mut u32;
    unsafe { out.write_volatile(out.read_volatile() | mask) }
}

/// PA 脚置低(`PA_CLR` 是写 1 清零)
#[inline]
pub fn pa_clear(mask: u32) {
    let clr = PA_CLR as *mut u32;
    unsafe { clr.write_volatile(mask) }
}
