//! NS16550A 轮询驱动——QEMU virt 机的 stdout(`-nographic` 下串口=终端)。
//!
//! 寄存器(标准 16550):THR@+0(发送保持)、LSR@+5(bit5=THRE 可写)。
//! 纯轮询无中断——输出路径与 longan_nano 的 chip stdout 同构。

use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, Ordering};

/// NS16550A 基址(virt 机)
const UART0: usize = 0x1000_0000;

static INITED: AtomicBool = AtomicBool::new(false);

/// 初始化(virt 机 QEMU 侧 UART 开箱即用,这里只做幂等标记 + 换行处理初始化)
pub fn configure() {
    INITED.store(true, Ordering::Relaxed);
}

/// 写单字节(等 THR 空;`\n` 补 `\r`——终端友好)
pub fn putc(b: u8) {
    let thr = UART0 as *mut u8;
    let lsr = (UART0 + 5) as *const u8;
    while unsafe { lsr.read_volatile() } & 0x20 == 0 {}
    unsafe {
        thr.write_volatile(b);
    }
}

/// 写字符串
pub fn write_str(s: &str) {
    for &b in s.as_bytes() {
        if b == b'\n' {
            putc(b'\r');
        }
        putc(b);
    }
}

struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_str(s);
        Ok(())
    }
}

/// 经静态槽写(宏专用出口;与 longan_nano 宏同构)
pub fn writeln_args(args: fmt::Arguments<'_>) {
    use core::fmt::Write as _;
    let _ = writeln!(Stdout, "{}", args);
}

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {{
        $crate::chip::qemu_riscv::stdout::writeln_args(format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! sprintln {
    () => { $crate::sprint!("\r") };
    ($($arg:tt)*) => {{
        $crate::chip::qemu_riscv::stdout::writeln_args(format_args!($($arg)*))
    }};
}

/// 测试自退出:写 SiFive test 设备(PASS=0x5555 → QEMU exit 0)
pub fn qemu_exit_pass() -> ! {
    let fin = super::SIFIVE_TEST as *mut u32;
    unsafe {
        fin.write_volatile(0x5555);
    }
    loop {
        core::hint::spin_loop();
    }
}

/// 测试自退出(FAIL)
pub fn qemu_exit_fail() -> ! {
    let fin = super::SIFIVE_TEST as *mut u32;
    unsafe {
        fin.write_volatile(0x3333);
    }
    loop {
        core::hint::spin_loop();
    }
}
