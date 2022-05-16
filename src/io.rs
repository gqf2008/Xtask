use crate::port::{Portable, Porting};
use core::fmt::{self, Write};

struct Stdout;

static mut STDOUT: Stdout = Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Porting::printf(s);
        Ok(())
    }
}

#[inline]
pub fn write_str(s: &str) {
    let _ = unsafe { STDOUT.write_str(s) };
}

#[inline]
pub fn write_fmt(args: fmt::Arguments) {
    let _ = unsafe { STDOUT.write_fmt(args) };
}

/// 加了中断保护，禁止在中断服务程序中调用
#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        $crate::sync::free(|_|$crate::io::write_str($s))
    };
    ($($tt:tt)*) => {
        $crate::sync::free(|_|$crate::io::write_fmt(format_args!($($tt)*)))
    };
}

/// 加了中断保护，禁止在中断服务程序中调用
#[macro_export]
macro_rules! sprintln {
    () => {
        $crate::sync::free(|_|$crate::io::write_str("\n"))
    };
    ($s:expr) => {
        $crate::sync::free(|_|$crate::io::write_str(concat!(file!(),":",line!()," ",$s, "\n")))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::sync::free(|_|$crate::io::write_fmt(format_args!(concat!(file!(),":",line!()," ",$s, "\n"), $($tt)*)))
    };
}

/// 在中断服务程序中调用，在用户程序
/// 里调用可能输出不完整，因为随时会被中断
#[macro_export]
macro_rules! isr_sprint {
    ($s:expr) => {
        $crate::io::write_str($s)
    };
    ($($tt:tt)*) => {
        $crate::io::write_fmt(format_args!($($tt)*))
    };
}

/// 在中断服务程序中调用，在用户程序
/// 里调用可能输出不完整，因为随时会被中断
#[macro_export]
macro_rules! isr_sprintln {
    () => {
        $crate::io::write_str("\n")
    };
    ($s:expr) => {
        $crate::io::write_str(concat!(file!(),":",line!()," ",$s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::io::write_fmt(format_args!(concat!(file!(),":",line!()," ",$s, "\n"), $($tt)*))
    };
}
