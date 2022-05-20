use core::fmt::{self, Write};
use cortex_m::interrupt;

use stm32f1xx_hal::pac::USART1;
use stm32f1xx_hal::serial::Tx;

static mut STDOUT: Option<Stdout<Tx<USART1>>> = None;

pub fn use_tx1(tx: Tx<USART1>) {
    interrupt::free(|_| unsafe {
        STDOUT.replace(Stdout(tx));
    })
}

pub struct Stdout<T>(pub T);

impl<T> Write for Stdout<T>
where
    T: embedded_hal::serial::Write<u8, Error = ::core::convert::Infallible>,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| nb::block!(self.0.write(*c)))
            .map_err(|_| core::fmt::Error)
    }
}

/// Writes string to stdout
pub fn write_str(s: &str) {
    unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_str(s);
        }
    }
}

/// Writes formatted string to stdout
pub fn write_fmt(args: fmt::Arguments) {
    unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_fmt(args);
        }
    }
}

/// Macro for printing to the serial standard output
#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        cortex_m::interrupt::free(|_|$crate::bsp::bluepill::stdout::write_str($s))
    };
    ($($tt:tt)*) => {
        cortex_m::interrupt::free(|_|$crate::bsp::bluepill::stdout::write_fmt(format_args!($($tt)*)))
    };
}

/// Macro for printing to the serial standard output, with a newline.
#[macro_export]
macro_rules! sprintln {
    () => {
        cortex_m::interrupt::free(|_|$crate::bsp::bluepill::stdout::write_str("\n"))
    };
    ($s:expr) => {
        cortex_m::interrupt::free(|_|$crate::bsp::bluepill::stdout::write_str(concat!($s, "\n")))
    };
    ($s:expr, $($tt:tt)*) => {
        cortex_m::interrupt::free(|_|$crate::bsp::bluepill::stdout::write_fmt(format_args!(concat!($s, "\n"), $($tt)*)))
    };
}

/// Macro for printing to the serial standard output
#[macro_export]
macro_rules! isr_sprint {
    ($s:expr) => {
        $crate::bsp::bluepill::stdout::write_str($s)
    };
    ($($tt:tt)*) => {
        $crate::bsp::bluepill::stdout::write_fmt(format_args!($($tt)*))
    };
}

/// Macro for printing to the serial standard output, with a newline.
#[macro_export]
macro_rules! isr_sprintln {
    () => {
        $crate::bsp::bluepill::stdout::write_str("\n")
    };
    ($s:expr) => {
        $crate::bsp::bluepill::stdout::write_str(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::bsp::bluepill::stdout::write_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}
