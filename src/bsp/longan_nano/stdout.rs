//! Stdout based on the UART hooked up to the debug connector

use core::fmt::{self, Write};
use gd32vf103xx_hal::serial::{Config, Parity, StopBits};
use gd32vf103xx_hal::{
    afio::Afio,
    gpio::{
        gpioa::{PA10, PA9},
        Active,
    },
    pac::USART0,
    prelude::*,
    rcu::Rcu,
    serial::{Serial, Tx},
    time::Bps,
};
use nb::block;
use riscv::interrupt;

static mut STDOUT: Option<SerialWrapper> = None;

struct SerialWrapper(Tx<USART0>);

impl fmt::Write for SerialWrapper {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            if *byte == '\n' as u8 {
                let res = block!(self.0.write('\r' as u8));

                if res.is_err() {
                    return Err(::core::fmt::Error);
                }
            }

            let res = block!(self.0.write(*byte));

            if res.is_err() {
                return Err(::core::fmt::Error);
            }
        }
        Ok(())
    }
}

/// Configures stdout
pub fn configure<X, Y>(
    uart: USART0,
    tx: PA9<X>,
    rx: PA10<Y>,
    baud_rate: Bps,
    afio: &mut Afio,
    rcu: &mut Rcu,
) where
    X: Active,
    Y: Active,
{
    let tx = tx.into_alternate_push_pull();
    let rx = rx.into_floating_input();
    let config = Config {
        baudrate: baud_rate,
        parity: Parity::ParityNone,
        stopbits: StopBits::STOP1,
    };
    let serial = Serial::new(uart, (tx, rx), config, afio, rcu);
    let (tx, _) = serial.split();
    interrupt::free(|_| unsafe {
        STDOUT.replace(SerialWrapper(tx));
    })
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
