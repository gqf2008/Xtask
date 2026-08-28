//! Cadence UART0 轮询驱动 + semihosting 自退出——QEMU zcu102 的 stdout
//! (`-nographic` 下串口 = 终端;semihosting 例程测完自退,CI 门禁无需
//! timeout 杀进程)。
//!
//! 寄存器(QEMU 源码级确认):CR@+0(bit4=TX_EN)、SR@+0x2C(bit3=TEMPTY)、
//! TXFIFO@+0x30。纯轮询无中断——输出路径与 longan_nano 同构。
//!
//! semihosting:ARM 态 SVC 号 **#0x123456**(Thumb 才是 0xab);SYS_EXIT(0x18)
//! 的 r1 直接放 reason(非指针):ADP_Stopped_ApplicationExit(0x20026)
//! → QEMU exit 0,其他 → exit 1。A32 规则探针实测确认。

use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, Ordering};

/// Cadence UART0 基址
const UART0: usize = super::UART0;

static INITED: AtomicBool = AtomicBool::new(false);

/// 初始化(QEMU 侧 UART 开箱即用,只做 TX 使能 + 幂等标记)
pub fn configure() {
    if INITED.swap(true, Ordering::Relaxed) {
        return;
    }
    unsafe {
        ((UART0 + 0x00) as *mut u32).write_volatile(0x10); // CR.TX_EN
    }
}

/// 写单字节(等 TEMPTY;`\n` 补 `\r`——终端友好)
pub fn putc(b: u8) {
    configure();
    let sr = (UART0 + 0x2C) as *const u32;
    while unsafe { sr.read_volatile() } & 0x08 == 0 {}
    unsafe {
        ((UART0 + 0x30) as *mut u32).write_volatile(b as u32);
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

/// 经静态槽写(宏专用出口;与 qemu_riscv/longan_nano 宏同构)
pub fn writeln_args(args: fmt::Arguments<'_>) {
    use core::fmt::Write as _;
    let _ = writeln!(Stdout, "{}", args);
}

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {{
        $crate::chip::qemu_arm_r52::stdout::writeln_args(format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! sprintln {
    () => { $crate::sprint!("\r") };
    ($($arg:tt)*) => {{
        $crate::chip::qemu_arm_r52::stdout::writeln_args(format_args!($($arg)*))
    }};
}

/// semihosting SYS_EXIT:ok → exit 0(PASS),否则 exit 1(FAIL)
pub fn semihost_exit(ok: bool) -> ! {
    let reason: u32 = if ok {
        0x20026 // ADP_Stopped_ApplicationExit → exit(0)
    } else {
        0x20023 // ADP_Stopped_InternalError → exit(1)
    };
    unsafe {
        core::arch::asm!(
            "mov r0, #0x18", // SYS_EXIT
            "mov r1, {0}",
            "svc #0x123456",
            in(reg) reason,
            options(nostack)
        );
    }
    unreachable!()
}

/// 测试通过自退出
pub fn qemu_exit_pass() -> ! {
    semihost_exit(true)
}

/// 测试失败自退出
pub fn qemu_exit_fail() -> ! {
    semihost_exit(false)
}
