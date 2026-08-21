//! Stdout based on the UART hooked up to the debug connector
//!
//! 实现已下沉到芯片层 `chip::gd32vf103::stdout`（全 crate 唯一持有 USART0 的 STDOUT
//! 静态），这里只做 re-export，保持 `bsp::longan_nano::stdout::*` 路径和下面四个宏
//! 的展开不变。此前本文件与芯片层各持有一份相同的 STDOUT 静态：示例配置其中一个、
//! 宏却写另一个，另一条路径上的日志会被静默丢弃。
pub use crate::chip::gd32vf103::stdout::{configure, write_fmt, write_str};

/// 加了中断保护，禁止在中断服务程序中调用
#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        $crate::sync::free(|_|$crate::bsp::longan_nano::stdout::write_str($s))
    };
    ($($tt:tt)*) => {
        $crate::sync::free(|_|$crate::bsp::longan_nano::stdout::write_fmt(format_args!($($tt)*)))
    };
}

/// 加了中断保护，禁止在中断服务程序中调用
#[macro_export]
macro_rules! sprintln {
    () => {
        $crate::sync::free(|_|$crate::bsp::longan_nano::stdout::write_str("\n"))
    };
    ($s:expr) => {
        $crate::sync::free(|_|$crate::bsp::longan_nano::stdout::write_str(concat!($s, "\n")))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::sync::free(|_|$crate::bsp::longan_nano::stdout::write_fmt(format_args!(concat!($s, "\n"), $($tt)*)))
    };
}

/// 在中断服务程序中调用，在用户程序
/// 里调用可能输出不完整，因为随时会被中断
#[macro_export]
macro_rules! isr_sprint {
    ($s:expr) => {
        $crate::bsp::longan_nano::stdout::write_str($s)
    };
    ($($tt:tt)*) => {
        $crate::bsp::longan_nano::stdout::write_fmt(format_args!($($tt)*))
    };
}

/// 在中断服务程序中调用，在用户程序
/// 里调用可能输出不完整，因为随时会被中断
#[macro_export]
macro_rules! isr_sprintln {
    () => {
        $crate::bsp::longan_nano::stdout::write_str("\n")
    };
    ($s:expr) => {
        $crate::bsp::longan_nano::stdout::write_str(concat!(file!(),":",line!()," ",$s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::bsp::longan_nano::stdout::write_fmt(format_args!(concat!(file!(),":",line!()," ",$s, "\n"), $($tt)*))
    };
}
