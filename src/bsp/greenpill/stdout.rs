use core::fmt::{self, Write};
use cortex_m::interrupt;

use stm32f4xx_hal::pac::USART1;
use stm32f4xx_hal::serial::{self, Tx};

static mut STDOUT: Option<Stdout<Tx<USART1>>> = None;

pub fn use_tx1(tx: Tx<USART1>) {
    interrupt::free(|_| unsafe {
        STDOUT.replace(Stdout(tx));
    })
}

pub struct Stdout<T>(pub T);

impl<T> Write for Stdout<T>
where
    T: embedded_hal::serial::Write<u8, Error = serial::Error>,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| nb::block!(self.0.write(*c)))
            .map_err(|_| core::fmt::Error)
    }
}

#[inline]
pub fn write_str(s: &str) {
    unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_str(s);
        }
    }
}

#[inline]
pub fn write_fmt(args: fmt::Arguments) {
    unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_fmt(args);
        }
    }
}

/// 加了中断保护，禁止在中断服务程序中调用
#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        $crate::sync::free(|_|$crate::bsp::greenpill::stdout::write_str($s))
    };
    ($($tt:tt)*) => {
        $crate::sync::free(|_|$crate::bsp::greenpill::stdout::write_fmt(format_args!($($tt)*)))
    };
}

/// 加了中断保护，禁止在中断服务程序中调用
#[macro_export]
macro_rules! sprintln {
    () => {
        $crate::sync::free(|_|$crate::bsp::greenpill::stdout::write_str("\n"))
    };
    ($s:expr) => {
        $crate::sync::free(|_|$crate::bsp::greenpill::stdout::write_str(concat!($s, "\n")))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::sync::free(|_|$crate::bsp::greenpill::stdout::write_fmt(format_args!(concat!($s, "\n"), $($tt)*)))
    };
}

/// 在中断服务程序中调用，在用户程序
/// 里调用可能输出不完整，因为随时会被中断
#[macro_export]
macro_rules! isr_sprint {
    ($s:expr) => {
        $crate::bsp::greenpill::stdout::write_str($s)
    };
    ($($tt:tt)*) => {
        $crate::bsp::greenpill::stdout::write_fmt(format_args!($($tt)*))
    };
}

/// 在中断服务程序中调用，在用户程序
/// 里调用可能输出不完整，因为随时会被中断
#[macro_export]
macro_rules! isr_sprintln {
    () => {
        $crate::bsp::greenpill::stdout::write_str("\n")
    };
    ($s:expr) => {
        $crate::bsp::greenpill::stdout::write_str(concat!(file!(),":",line!()," ",$s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::bsp::greenpill::stdout::write_fmt(format_args!(concat!(file!(),":",line!()," ",$s, "\n"), $($tt)*))
    };
}
