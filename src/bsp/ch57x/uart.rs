//! CH57x(WCH CH583 / CH572)轮询 UART 控制台(仅 TX;RX 留给后续 BSP)。
//!
//! 寄存器与波特率公式**照抄官方 EVT 源码**:
//! - 基址 `0x40003400`,偏移 `IER @+1 / FCR @+2 / LCR @+3 / LSR @+5(bit6=发送全空) /
//!   THR @+8 / R16_DL @+0x0C / R8_DIV @+0x0E`
//!   (官方 `CH583SFR.h`、`CH572SFR.h`;两芯片同址同偏移,故一份驱动够用)。
//! - 初始化序列同官方 `UART1_DefInit()` / `UART_DefInit()`:
//!   `FCR=(2<<6)|TX_CLR|RX_CLR|FIFO_EN`、`LCR=0x03`(8 位字长)、`IER=0x40`(TXD 使能)、
//!   `DIV=1`。
//! - 波特率:`DL = round(clk / (8*baud))`——官方写法 `x = 10*clk/8/baud; x = (x+5)/10`。
//!
//! ⚠️ `clk_hz` 必须传**复位后的实际主频假设**:本驱动不碰 SAM 解锁/PLL 时钟树,
//! 真实主频由 `examples/board_check_ch57x.rs` 用"逐个假设各打一行标记、哪行可读即该值"
//! 的办法先测出来(这也是上板第一步要回答的问题)。

const UART_BASE: usize = 0x4000_3400;

#[inline]
fn r8(off: usize) -> *mut u8 {
    (UART_BASE + off) as *mut u8
}
#[inline]
fn r16(off: usize) -> *mut u16 {
    (UART_BASE + off) as *mut u16
}

/// 按给定主频假设配置控制台(8-N-1,波特率由调用方给)
pub fn configure(clk_hz: u32, baud: u32) {
    let dl = (clk_hz / (8 * baud)) as u16;
    unsafe {
        r16(0x0C).write_volatile(dl); // R16_DL:除数锁存
        r8(0x0E).write_volatile(1); // R8_DIV:预分频(官方惯用法 1)
        r8(0x02).write_volatile(0x80 | 0x04 | 0x02 | 0x01); // FCR:触发点 2|清 TX/RX FIFO|使能
        r8(0x03).write_volatile(0x03); // LCR:8 位字长
        r8(0x01).write_volatile(0x40); // IER:使能 TXD 引脚
    }
}

/// 写单字节(等"发送全空";`\n` 前补 `\r` 由 `write_str` 负责)
pub fn putc(b: u8) {
    let lsr = r8(0x05) as *const u8;
    let thr = r8(0x08);
    unsafe {
        while lsr.read_volatile() & 0x40 == 0 {}
        thr.write_volatile(b);
    }
}

/// 写字符串(把 `\n` 展开成 `\r\n`)
pub fn write_str(s: &str) {
    for &b in s.as_bytes() {
        if b == b'\n' {
            putc(b'\r');
        }
        putc(b);
    }
}

/// `core::fmt::Write` 出口:`write!(Console, ...)` 用
pub struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write_str(s);
        Ok(())
    }
}
