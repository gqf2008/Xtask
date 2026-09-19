//! CH572(QingKe,**无 A 扩展**;PFIC + WCH 自有 SysTick 模型)移植模块实现。
//! 以 `ch583`(V4A)为模板——两者架构同源(同为 Direct mtvec + 36 字纯软件
//! 全保存帧 + 命令行级同一套 PFIC 使能),差异集中在 SysTick 与两条私有 CSR。
//!
//! 与 `ch583` 的差异(逐条按官方 `openwch/ch570` EVT 源码核对):
//! - **无 A 扩展** → 目标用 `riscv32imc-unknown-none-elf`;内核的原子读改写由
//!   `atomic-polyfill` 兜到 `critical-section`(`src/arch/riscv/critical.rs`),
//!   实测产物 0 条 AMO/LR/SC(见 README「无 A 扩展目标」一条)。
//! - **SysTick 是 32 位**:`CNT`@+8、`CMP`@+16 各只有低字(无 CNTH/CMPHR),
//!   读法与 `ch583` 的 64 位高低重读不同。
//! - **软中断触发位在 `SR`(bit31)而非 `CTLR`**(`SysTick_SR_SWIE`)——但本口
//!   **不用它**:yield/抢占请求走官方三套 RTOS 移植的 `SWI_IRQn=14`(见 `irq()`),
//!   `SWIE` 在初始化与运行期都不碰。`CTLR` 里也**没有 INIT 位**
//!   (官方 `SysTick_Config` 写 `0x0F` = STRE|STCLK|STIE|STE)。
//!   该 IRQ 与 SysTick 一样要在 `IENR[0]` 里**使能**(= bit12|bit14,见
//!   `setup_intrrupt`)——PFIC 的 pending 与 enable 分离,只置 pending 不使能,
//!   请求会挂在一个永不被取走的中断上(而构建/门禁仍全绿)。
//! - **私有 CSR 多一条**:官方 reset 序列为 `0xbc0=0x25` **且 `0xbc1=1`**
//!   (ch583/ch32v* 只有 `0xbc0=0x1f`)。
//! - **12K SRAM / 240K CODE**:任务栈账比 ch583 紧得多(见示例注释)。
//!
//! 与 `ch583` 相同、已按官方源码确认的部分:
//! - PFIC/SysTick 基址同址(`0xE000E000`/`0xE000F000`),`IENR[0]@0x100` 的
//!   bit12|bit14 使能核中断 12 与 14(两个 `PFIC_EnableIRQ`,
//!   `SysTick_IRQn=12` + `SWI_IRQn=14`——后者官方三套移植都有,漏了 yield 失效);
//! - `SR.CNTIF`(bit0)区分真 tick 与软中断请求,**写 0 清**(WCH 惯例,待上板复核);
//! - ROM 启动头:官方 `startup_CH572.S` 的 `.vector` 第 5 字同样是
//!   `0xF3F9BDA9`,用官方 `Link.ld` 现场链接实测**同样落在 flash 0x14**
//!   (`.highcode` 里的 `. = ALIGN(1024)` 因 RAM 基址本就 1024 对齐而无效果),
//!   故与 ch583 共用 `.bootvec` 布局(见 port.S / memory.x)。
//!
//! ⚠️ 真机核对点(构建级验证,板上行为待验):
//! ①`0x00000000` 是否可写 / 是否有别名;
//! ②复位默认主频(env 按官方 `CH57x_common.h` 的 `FREQ_SYS=100MHz` 配置;
//!   本口无 BSP/时钟初始化,SAM 解锁与 PLL 未做,须实测校正);
//! ③`0x804=0` 后异常/中断入口是否确实不再硬件压栈(决定 36 字帧成立);
//! ④置 `SWI_IRQn=14` pending 后是否即刻进 trap、并在 `_start_trap` 里按
//!   mcause=14 走切换(官方三套移植的 yield 路径;`SWIE` 已不使用);
//! ⑤`delay_us` 的实测精度(现用 SysTick 计数,不再依赖 `mcycle`);
//! ⑥BootLoader 是否放行(启动头已复刻,仍需实板确认)。

mod port;

use super::wch_pfic;
use super::{SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::port::Portable;
use crate::prelude::CriticalSection;
use crate::task::Task;
use core::arch::asm;

/// SysTick 寄存器基址(WCH 自有,非 CLINT;CH572 为 **32 位**计数器)
pub(crate) const STK_BASE: usize = 0xE000_F000;

/// PFIC 寄存器基址(CH572 与 CH583/ch32v103/ch32v307 同址)
pub(crate) const PFIC_BASE: usize = 0xE000_E000;

/// 配置 SysTick 与 PFIC(在 start_scheduler 里、restore_ctx 之前调用)。
/// 逐位对齐官方 `RVMSIS/core_riscv.h` 的 `SysTick_Config(uint32_t ticks)`:
/// `CNTL=0; CMP=ticks-1; PFIC_EnableIRQ(SysTick_IRQn); CTLR=STRE|STCLK|STIE|STE`。
#[inline]
pub(crate) fn setup_intrrupt() {
    let stk = STK_BASE as *mut u32;
    unsafe {
        // 1. 停表(CTLR=0)、清 SR(CNTIF 写 0 清;SWIE 本口不用,一并清)、
        //    CNT=0、CMP=TICKS-1
        //    ——CH572 的 CNT/CMP 只有低字(32 位),没有 CNTH/CMPHR
        stk.write_volatile(0);
        stk.add(1).write_volatile(0); // SR
        stk.add(2).write_volatile(0); // CNTL(32 位计数器)
        const TICKS: u32 = (SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u32 - 1;
        stk.add(4).write_volatile(TICKS); // CMPL(32 位比较值)
        // 2. CTLR = STRE(3)|STCLK(HCLK,2)|STIE(1)|STE(0) = 0x0F
        //    ——官方 `SysTick_Config` 同值;**CH572 的 CTLR 没有 INIT 位**,
        //    (SWIE 在 SR.bit31,但本口不用它——yield 走 SWI_IRQn=14,见 irq())
        stk.write_volatile(0x0000_000F);
    }
    // 3. PFIC:使能 **SysTick(bit12) + 软中断(bit14)** —— pending 与 enable 是
    //    两组分离的寄存器:`irq()` 只负责置 pending(`IPSR[0]`),中断要被取走还得
    //    在 `IENR[0]` 里使能该 IRQ(官方 `PFIC_EnableIRQ(SWI_IRQn)` 就是这一步)。
    //    漏掉 bit14 → `irq()` 的 pending 永远不被取走(yield/抢占只剩 tick 兜底),
    //    而构建/门禁/host 测试全绿——2026-09-19 PR #19 独立审查实测。
    let ienr0 = (PFIC_BASE + wch_pfic::IENR_OFFSET) as *mut u32;
    unsafe {
        ienr0.write_volatile(wch_pfic::ienr_enable_mask());
    }
}

/// 清 SysTick 的 CNTIF(写 0)——真 tick 路径调用
#[inline]
pub(crate) fn reset_systick() {
    let stk = STK_BASE as *mut u32;
    unsafe {
        stk.add(1).write_volatile(0); // SR @ +4
    }
}

/// CH572 芯片移植层实现
pub struct Ch572Porting;

// port.S 蹦床 `_task_entry_trampoline` 依赖的 Task 布局偏移(失配编译期炸)
const _: () = assert!(core::mem::offset_of!(Task, sp) == 0);
const _: () = assert!(core::mem::offset_of!(Task, entry) == 8);

impl Portable for Ch572Porting {
    /// 完全内存屏障
    #[inline]
    fn barrier() {
        unsafe {
            // CH572 无 MMU/虚存,通用 fence(iorw) 即可
            core::arch::asm!("fence iorw, iorw");
        }
    }
    /// 临界区保护(本核 mstatus.MIE——单核语义)
    #[inline]
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        riscv::interrupt::free(f)
    }

    /// 开全局中断
    #[inline]
    fn enable_interrupt() {
        unsafe {
            riscv::interrupt::enable();
        }
    }
    /// 关全局中断
    #[inline]
    fn disable_interrupt() {
        unsafe {
            riscv::interrupt::disable();
        }
    }

    /// 启动调度器:配置 SysTick/PFIC → 恢复第一个任务(汇编,不返回)
    fn start_scheduler() -> ! {
        setup_intrrupt();
        log::info!("Start scheduler");
        unsafe { asm!(include_str!("restore_ctx.S"), options(noreturn, raw)) };
    }

    /// 软中断(调度请求):置 PFIC 的 `SWI_IRQn=14` pending
    /// (`IPSR[0] @0xE000E200` bit14)。
    ///
    /// 为什么不用 CH572 自己的 `SR.SWIE`:官方 CH572 SDK 的 SysTick 只**定义**
    /// 不用(全 SDK 零处调用),而官方三套 RTOS 移植的 yield 一律走
    /// `PFIC_SetPendingIRQ(SWI_IRQn)` + `SW_Handler`(CH572 的 `SWI_IRQn=14`);
    /// 走官方路径把"SWIE 到底灵不灵"这个只能上板排除的赌注消掉。
    /// 该 IRQ 的使能位在 `setup_intrrupt()` 里与 SysTick 一并置上(`IENR[0]`
    /// bit14)——置 pending 与使能缺一,本函数就是空操作。
    #[inline]
    fn irq() {
        let ipsr0 = (PFIC_BASE + wch_pfic::IPSR_OFFSET) as *mut u32;
        unsafe {
            ipsr0.write_volatile(wch_pfic::irq_bit(wch_pfic::SWI_IRQ));
        }
    }
    /// 关闭软中断:清 `SWI_IRQn=14` 的 pending(`IPRR[0] @0xE000E280` bit14),
    /// 防残留 pending 让 trap 出口立刻重入。
    #[inline]
    fn disable_irq() {
        let iprr0 = (PFIC_BASE + wch_pfic::IPRR_OFFSET) as *mut u32;
        unsafe {
            iprr0.write_volatile(wch_pfic::irq_bit(wch_pfic::SWI_IRQ));
        }
    }

    /// 读 SysTick 计数器(**32 位**,CH572 无高字——与 ch583 的 64 位读法不同)
    #[inline]
    fn systick() -> u64 {
        let stk = STK_BASE as *mut u32;
        unsafe { stk.add(2).read_volatile() as u64 }
    }

    /// 硬件延时,单位 us —— **用 SysTick 计数器**实现(不再赌 `mcycle`/Zicntr:
    /// CH572 是否实现它没有一手证据,未实现时读出恒为常量 → 死循环;SysTick 是
    /// 内核点拍赖以工作的外设,必然实现)。计数器 32 位、按 CMP 自动重装,
    /// 等待按"到下次重装还剩多少"分块(跨重装差值见 `crate::chip::delay`,有 host 单测)。
    ///
    /// ⚠️ **前置条件:SysTick 已在跑**(即 `start_scheduler()` 之后)。计数器停着时
    /// `elapsed()` 恒为 0,等待不会结束——`start_scheduler()` 之前需要延时用忙等。
    #[inline]
    fn delay_us(us: u64) {
        let period = (SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u64; // = TICKS + 1
        let mut remaining = us * (SYSTICK_CLOCK_HZ as u64) / 1_000_000;
        while remaining > 0 {
            let start = Self::systick();
            let chunk = crate::chip::delay::next_chunk(remaining, start, period);
            while crate::chip::delay::elapsed(start, Self::systick(), period) < chunk {}
            remaining -= chunk;
        }
    }

    /// 任务创建时为 CPU 准备任务现场(36 字帧,与 port.S 的保存宏互为镜像):
    /// [35]=mcause(0x8000_000C)[34]=0(mcause 保留槽)
    /// [33]=mepc=_task_entry_trampoline [32]=mstatus=0x1880(MPP=M|MPIE=1)
    /// [10]=a0=args [1]=ra=task_exit,任务块 sp 指向帧底
    #[inline]
    fn save_context(task: &mut Task) {
        unsafe {
            let sp = task.stack.add(task.stack_size - 1);
            sp.offset(-1).write_volatile(0x8000_000C); // mcause:中断|12(SysTick)
            sp.offset(-2).write_volatile(0); // 保留槽(无 msubm)
            // mepc = 首调蹦床:经标准 jalr 进入 task.entry(mret 直入会被
            // 编译器 outlined 的入口 stub 坑到野跳,见 port.S 蹦床注)
            unsafe extern "C" {
                fn _task_entry_trampoline();
            }
            sp.offset(-3)
                .write_volatile((_task_entry_trampoline as *const ()).addr()); // mepc
            sp.offset(-4).write_volatile(0x0000_1880); // mstatus:MPP=M, MPIE=1
            for i in 0..32usize {
                if i != 1 && i != 10 {
                    sp.offset(i as isize - 36).write_volatile(0);
                }
            }
            sp.offset(-26).write_volatile(task.args.addr()); // a0
            sp.offset(-35)
                .write_volatile((port::task_exit as *const ()).addr()); // ra
            task.sp = sp.offset(-36).addr();
        }
    }
}
