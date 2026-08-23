//! UART0(GPIO0/1)串口输出——Pico 的 STDOUT。
//!
//! 类型上有别于 longan_nano 版(那个是 chip 层 SerialWrapper):rp2040 口
//! 直接持有 UartPeripheral,静态槽 + 宏与 longan_nano 同构。

use core::fmt::Write;
use cortex_m::interrupt;

use hal::pac;
use rp2040_hal as hal;

use hal::gpio::bank0::{Gpio0, Gpio1};
type UartPins = (
    hal::gpio::Pin<Gpio0, hal::gpio::FunctionUart, hal::gpio::PullDown>,
    hal::gpio::Pin<Gpio1, hal::gpio::FunctionUart, hal::gpio::PullDown>,
);

type Uart = hal::uart::UartPeripheral<hal::uart::Enabled, pac::UART0, UartPins>;

static mut STDOUT: Option<Uart> = None;

pub fn use_uart0(uart: Uart) {
    interrupt::free(|_| unsafe {
        STDOUT.replace(uart);
    })
}

/// 经静态槽写(不消费 UART,宏专用出口)
struct StaticWriter;

impl Write for StaticWriter {
    fn write_str(&mut self, data: &str) -> core::fmt::Result {
        unsafe {
            if let Some(uart) = &mut *core::ptr::addr_of_mut!(STDOUT).cast::<Option<Uart>>() {
                uart.write_full_blocking(data.as_bytes());
            }
        }
        Ok(())
    }
}

/// 直接向 STDOUT 写一行(stdout 初始化后可用)
pub fn writeln_str(s: &str) {
    use core::fmt::Write;
    let mut w = StaticWriter;
    let _ = writeln!(w, "{s}");
}

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!(StaticWriterHelper, $($arg)*);
    }};
}

/// 宏辅助(避免在宏内 unsafe)
struct StaticWriterHelper;

impl Write for StaticWriterHelper {
    fn write_str(&mut self, data: &str) -> core::fmt::Result {
        unsafe {
            if let Some(uart) = &mut *core::ptr::addr_of_mut!(STDOUT).cast::<Option<Uart>>() {
                uart.write_full_blocking(data.as_bytes());
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! sprintln {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = writeln!(StaticWriterHelper, $($arg)*);
    }};
}
