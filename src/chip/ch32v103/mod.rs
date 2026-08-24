//! CH32V103 移植模块实现(QingKe V3A(最简三代:PFIC 无字节优先级,SysTick 独立外设) 内核,PFIC + SysTick 模型)。
//!
//! 与 gd32vf103(Bumblebee/ECLIC)的关键差异——本口按**标准 RV32 机器模式**设计:
//!
//! - **HPE 关闭**:QingKe 的硬件序言(隐藏栈自动压 16 个 caller-saved 寄存器)
//!   是 WCH 私有工具链特性;`_setup_interrupts` 显式写 `intsyscr(0x804)=0`,
//!   上下文切换回到**纯软件全保存**模型(与 gd32/Bumblebee 同款 36 字帧,
//!   帧内 0x7C4/msubm 槽保留为 0)。
//! - **mtvec = Direct 模式**:单入口 `_start_trap` 读 `mcause` 分发——
//!   中断号 12 = SysTick(PFIC 核心中断,ARM 兼容位);其余走 `DefaultHandler`
//!   (弱符号,BSP/驱动扩展时再加向量表)。V4 虽支持向量模式,Direct 统一
//!   三代内核(V3A 只有 Direct),少一张对齐表。
//! - **软中断(yield/抢占请求)复用 SysTick 入口**:`STK_CTLR.SWIE`(bit31)
//!   写 1 立即触发 SysTick 中断——ISR 侧读 `STK_SR.CNTIF` 区分真 tick
//!   (计数到点,做时间账)与 SWIE 触发(纯调度请求,不算时间)。
//!   等价 gd32 的 MSIP,但不占用 CLINT(PFIC 无 MSIP 对应物)。
//! - **SysTick@0xE000F000**(WCH 自有,非 CLINT mtime):64 位计数器 +
//!   CMP 比较,`CTLR=0xF`(STE|STIE|STCLK=HCLK|STRE)自动重装;
//!   `SR.CNTIF` **写 0 清零**(ARM 写 1 的反面——已从 WCH EVT 实装取证)。
//!
//! ⚠️ 真机核对点(构建级验证 2026-08-23,板上行为待验):
//! ①flash 基址 0x08000000(ch32-data yaml,与 WCH EVT 链接脚本别名关系);
//! ②复位默认时钟 = HSI 8MHz(env 常数按此配;PLL 配好后同步改);
//! ③`CTLR=0xF` 的自动重装语义与 SR 写 0 清除;
//! ④V4 的 `intsyscr` CSR 编号 0x804(写 0 防御性关闭 HPE)。
mod port;

use super::{CPU_CLOCK_HZ, SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::port::Portable;
use crate::prelude::CriticalSection;
use crate::task::Task;
use ch32v1::ch32v103::PFIC;
use core::arch::asm;

/// SysTick 寄存器基址(直址访问——PAC 与直址二选一,这里用 PAC)
pub(crate) const STK_BASE: usize = 0xE000_F000;

/// 配置 SysTick 与 PFIC(在 start_scheduler 里、restore_ctx 之前调用)。
/// V3A 的 PFIC **没有字节优先级寄存器**(无 iprior 系列),只做使能;
/// SysTick 独立外设 @0xE000F000,PAC 只建模 CTLR.ste——全量直址
/// (位布局与 V4 的 PFIC 尾部 STK 兼容:STE0/STIE1/STCLK2/STRE3/SWIE31)
#[inline]
pub(crate) fn setup_intrrupt() {
    let stk = STK_BASE as *mut u32;
    unsafe {
        // 1. 停计数、清 SR.CNTIF(写 0)、CNT/CMP 清零装初值(高字先写)
        stk.write_volatile(0);
        stk.add(1).write_volatile(0); // SR
        stk.add(2).write_volatile(0); // CNTL
        stk.add(3).write_volatile(0); // CNTH
        const TICKS: u32 = (SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u32 - 1;
        stk.add(5).write_volatile(0); // CMPHR
        stk.add(4).write_volatile(TICKS); // CMPLR
        // 2. CTLR = STE|STIE|STCLK(HCLK)|STRE = 0xF(WCH EVT 惯用法)
        stk.write_volatile(0xF);
    }
    // 3. PFIC:使能 SysTick(IENR1 bit12;优先级交给 ITHRESDR 默认阈值)
    let pfic = unsafe { &*PFIC::ptr() };
    pfic.ienr1.write(|w| unsafe { w.bits(1 << 12) });
}

/// 清 SysTick 的 CNTIF(写 0)——真 tick 路径调用(V3A 直址)
#[inline]
pub(crate) fn reset_systick() {
    let stk = STK_BASE as *mut u32;
    unsafe {
        stk.add(1).write_volatile(0);
    }
}

/// CH32V103 芯片移植层实现
pub struct Ch32v103Porting;

// port.S 蹦床 `_task_entry_trampoline` 依赖的 Task 布局偏移(失配编译期炸)
const _: () = assert!(core::mem::offset_of!(Task, sp) == 0);
const _: () = assert!(core::mem::offset_of!(Task, entry) == 8);

impl Portable for Ch32v103Porting {
    /// 完全内存屏障
    #[inline]
    fn barrier() {
        unsafe {
            // V4F 有 MMU,但通用性优先用 fence(iorw)——sfence.vma 仅在
            // 页表维护时需要,内核目前不做虚存管理
            core::arch::asm!("fence iorw, iorw");
        }
    }
    /// 临界区保护(本核 mstatus.MIE——单核语义与 gd32 同)
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

    /// 软中断(调度请求):写 STK_CTLR.SWIE(bit31)触发 SysTick 入口。
    /// ISR 读 SR.CNTIF 区分真 tick 与本请求(见 port.rs 的 SysTick 处理)
    #[inline]
    fn irq() {
        let ctlr = STK_BASE as *mut u32;
        unsafe {
            ctlr.write_volatile(ctlr.read_volatile() | (1 << 31));
        }
    }
    /// 关闭软中断(SWIE 是触发位,自清;防御性清一下)
    #[inline]
    fn disable_irq() {
        let ctlr = STK_BASE as *mut u32;
        unsafe {
            ctlr.write_volatile(ctlr.read_volatile() & !(1 << 31));
        }
    }

    /// 读 SysTick 64 位计数器(高:低:高重读防翻转;V3A 直址)
    #[inline]
    fn systick() -> u64 {
        let stk = STK_BASE as *mut u32;
        loop {
            unsafe {
                let hi = stk.add(3).read_volatile();
                let lo = stk.add(2).read_volatile();
                if hi == stk.add(3).read_volatile() {
                    return ((hi as u64) << 32) | lo as u64;
                }
            }
        }
    }

    /// 硬件延时,单位 us(mcycle——复位默认 HSI 8MHz,PLL 后同步改 env)
    #[inline]
    fn delay_us(us: u64) {
        let t0 = riscv::register::mcycle::read64();
        let clock = (us * (CPU_CLOCK_HZ as u64)) / 1_000_000;
        while riscv::register::mcycle::read64().wrapping_sub(t0) <= clock {}
    }

    /// 任务创建时为 CPU 准备任务现场(36 字帧,与 port.S 的保存宏互为镜像):
    /// [35]=mcause(0x8000_000C)[34]=0(Bumblebee msubm 槽,保留)
    /// [33]=mepc=entry [32]=mstatus=0x1880(MPP=11|MPIE=1)
    /// [10]=a0=args [1]=ra=task_exit,任务块 sp 指向帧底
    #[inline]
    fn save_context(task: &mut Task) {
        unsafe {
            let sp = task.stack.add(task.stack_size - 1);
            sp.offset(-1).write_volatile(0x8000_000C); // mcause:中断|12(SysTick)
            sp.offset(-2).write_volatile(0); // 保留槽(V4F 无 msubm)
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
