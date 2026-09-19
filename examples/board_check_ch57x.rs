#![no_std]
#![no_main]

extern crate alloc;

// CH57x(ch583 / ch572)上板自检 —— 把 issue #14 里"只能上板回答"的核对点收敛成一次烧写:
//
//   阶段一:逐个主频假设各打一行标记(115200 8N1)。**哪一行可读,复位主频就是那个值**
//           (本口不做 SAM 解锁/PLL,主频就是复位默认值,只能这样测)。
//   阶段二:按定稿假设打印核对点——启动头魔数回读、SysTick/PFIC 寄存器回读、
//           INTSYSCR(自定义 CSR 0x804)回读、`mcycle` 是否推进(仅供参考)、
//           软中断(SWI_IRQn=14)pending 是否被硬件取走;
//           再起两个同优先级任务互抢 CPU,各自周期打印计数器与 `tick()`——
//           **两个计数器都在涨且 tick 在涨**,就说明时基、上下文切换、时间片在真机成立。
//
// 用法:先按默认 CLK_HZ 烧一次看阶段一 → 把 CLK_HZ 改成可读的那个值 → 重烧看阶段二。
use core::fmt::Write;

use xtask::arch::riscv::rt;
use xtask::bsp::ch57x::{self, uart::Console};
use xtask::prelude::*;

/// 阶段二的主频定稿(首次上板前按阶段一结果改;默认 CH583 官方 `FREQ_SYS=60MHz`,
/// CH572 官方默认 100MHz,见 `src/chip/env.rs`)
const CLK_HZ: u32 = 60_000_000;
const BAUD: u32 = 115200;

/// 与 port.S / mod.rs 一致的直址(零 PAC)
const STK_BASE: usize = 0xE000_F000;
const PFIC_BASE: usize = 0xE000_E000;

/// 本口初始化后 `STK_CTLR` 的期望值:ch583 = INIT|STRE|STCLK|STIE|STE = 0x2F;
/// ch572 的 CTLR 没有 INIT 位 = 0x0F
#[cfg(feature = "ch583")]
const EXP_CTLR: u32 = 0x2F;
#[cfg(feature = "ch572")]
const EXP_CTLR: u32 = 0x0F;

#[rt::entry]
fn main() -> ! {
    extern "C" {
        static _sheap: u8;
        /// 启动头魔数(port.S 里 `.bootvec` 的第 5 字;符号地址即它在 flash 的位置)
        static _boot_magic: u32;
        static _bootvec: u32;
    }

    // ---- 阶段一:复位主频假设扫描 ----
    for hz in [8u32, 16, 24, 32, 48, 60, 100] {
        ch57x::uart::configure(hz * 1_000_000, BAUD);
        let mut c = Console;
        let _ = write!(
            c,
            "\r\n=== [假设] 复位主频 {} MHz —— 这行可读即此值(115200 8N1) ===\r\n",
            hz
        );
    }

    // ---- 阶段二:按定稿假设打印核对点 ----
    ch57x::uart::configure(CLK_HZ, BAUD);
    let mut c = Console;
    let _ = write!(c, "\r\n[board_check] 定稿假设 CLK={} Hz\r\n", CLK_HZ);

    let magic = unsafe { core::ptr::read_volatile(&_boot_magic as *const u32) };
    let vec_addr = unsafe { &_bootvec as *const u32 as usize };
    let _ = write!(
        c,
        "[boot] _bootvec=0x{:08X} 魔数=0x{:08X} {}\r\n",
        vec_addr,
        magic,
        if magic == 0xF3F9_BDA9 {
            "OK(boot option 正确)"
        } else {
            "!! 不是 F3F9BDA9 —— ROM 可能不认这个镜像"
        }
    );

    let ctlr = unsafe { (STK_BASE as *const u32).read_volatile() };
    let sr = unsafe { ((STK_BASE + 4) as *const u32).read_volatile() };
    let ienr0 = unsafe { ((PFIC_BASE + 0x100) as *const u32).read_volatile() };
    let _ = write!(
        c,
        "[stk] CTLR=0x{:08X}(期望 0x{:08X}) SR=0x{:08X} PFIC_IENR[0]=0x{:08X}(bit12 应=1) {}\r\n",
        ctlr,
        EXP_CTLR,
        sr,
        ienr0,
        if ctlr == EXP_CTLR && ienr0 & (1 << 12) != 0 {
            "OK"
        } else {
            "!! 与初始化写入不符"
        }
    );

    // INTSYSCR(自定义 CSR 0x804):官方 RT-Thread 口就靠 csrr 读它判嵌套层级,
    // 本口写 0(关硬件压栈 + 关嵌套)
    let intsyscr: u32;
    unsafe { core::arch::asm!("csrr {}, 0x804", out(reg) intsyscr) };
    let _ = write!(
        c,
        "[csr] intsyscr(0x804)=0x{:08X} {}\r\n",
        intsyscr,
        if intsyscr & 0x3 == 0 {
            "OK(HPE 与嵌套都关着,36 字帧前提成立)"
        } else {
            "!! 非 0 —— 硬件压栈/嵌套没关,36 字帧不成立"
        }
    );

    // mcycle 是否推进(仅参考:delay_us 已改用 SysTick,不依赖它)
    let m0 = xtask::arch::riscv::register::mcycle::read64();
    let mut spin = 0u32;
    while spin < 1000 {
        spin += 1;
        core::hint::spin_loop();
    }
    let m1 = xtask::arch::riscv::register::mcycle::read64();
    let _ = write!(
        c,
        "[misc] mcycle 前后差={}(0 表示没有 Zicntr;本口 delay_us 已改用 SysTick)\r\n",
        m1.wrapping_sub(m0)
    );

    // ---- 起任务:既验证软中断 yield 通路,也验证时基/切换/时间片 ----
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    // 两个 1K 栈任务 + 软件定时器/idle 的实测账见 examples/multitask_ch572.rs
    xtask::init_heap(start_addr, 9 * 1024 + 512);

    TaskBuilder::new().name("probe").spawn(move || {
        let mut c = Console;
        // SWI_IRQn=14 自检:置 pending → 若 trap 侧按 mcause=14 走了切换,硬件取中断时
        // 会把 pending 清掉;清不掉说明这条通路在真机上没生效(PFIC 的 ISR[0] 即 pending 位图)
        let isr14 = |()| unsafe { ((PFIC_BASE) as *const u32).read_volatile() & (1 << 14) };
        let before = isr14(());
        xtask::yield_now(); // 内部 Porting::irq() → 置 IPSR bit14
        let after = isr14(());
        let _ = write!(
            c,
            "[swi] yield 前后 ISR bit14 = {}/{} {}\r\n",
            (before != 0) as u8,
            (after != 0) as u8,
            if before == 0 && after == 0 {
                "OK(pending 被硬件取走 → SWI_IRQn=14 通路成立)"
            } else {
                "!! pending 残留 —— SWI 通路可疑"
            }
        );

        // 与另一个任务互抢:两个计数器都在涨 + tick 在涨 = 时基/切换/时间片成立
        TaskBuilder::new().name("counter_b").spawn(move || {
            let mut n = 0u64;
            let mut c = Console;
            loop {
                n += 1;
                if n % 200_000 == 0 {
                    let _ = write!(c, "[B] n={} tick={}\r\n", n, xtask::tick());
                }
            }
        });
        let mut n = 0u64;
        loop {
            n += 1;
            if n % 200_000 == 0 {
                let _ = write!(c, "[A] n={} tick={}\r\n", n, xtask::tick());
            }
        }
    });

    xtask::start()
}
