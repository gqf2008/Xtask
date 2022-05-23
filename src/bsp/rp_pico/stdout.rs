use core::fmt::{self, Write};
use cortex_m::interrupt;

use hal::pac;
use rp2040_hal as hal;

use hal::gpio::pin::bank0::{Gpio0, Gpio1};
type UartPins = (
    hal::gpio::Pin<Gpio0, hal::gpio::Function<hal::gpio::Uart>>,
    hal::gpio::Pin<Gpio1, hal::gpio::Function<hal::gpio::Uart>>,
);

type Uart = hal::uart::UartPeripheral<hal::uart::Enabled, pac::UART0, UartPins>;

static mut STDOUT: Option<Stdout> = None;

pub struct Stdout(Uart);

pub fn use_uart0(uart: Uart) {
    interrupt::free(|_| unsafe {
        STDOUT.replace(Stdout(uart));
    })
}

impl Write for Stdout {
    fn write_str(&mut self, data: &str) -> core::fmt::Result {
        self.0.write_full_blocking(data.as_bytes());
        Ok(())
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
        $crate::sync::free(|_|$crate::bsp::greenpill::stdout::write_str(concat!(file!(),":",line!()," ",$s, "\n")))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::sync::free(|_|$crate::bsp::greenpill::stdout::write_fmt(format_args!(concat!(file!(),":",line!()," ",$s, "\n"), $($tt)*)))
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
