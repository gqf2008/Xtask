//! ESP32-C3 移植模块实现(RV32IMC,单核,无 FPU——标准机器模式)。
//!
//! 路线:**esp32c3 PAC 直依赖**(0.3.0,依赖仅 riscv/vcell,零 embedded-hal
//! 依赖——esp32c3-hal 0.13 锁 `embedded-hal =1.0.0-rc.1` 与本仓库图冲突,
//! 0.11 的 alpha.11 可选依赖虽可绕,但 esp-riscv-rt 与 riscv-rt 的 `_start`
//! 符号冲突无法共存——PAC + 仓库自带 riscv-rt 是唯一零冲突组合)。
//!
//! 中断模型(C3 特色:外设中断经**中断矩阵**映射到 CPU 中断 1..31):
//! - **tick = CPU 中断 6** ← `systimer_target0_int_map=6`:SYSTIMER
//!   target0 **周期模式**(16MHz / 16000 = 1kHz);`int_clr` 是 **W1C**
//!   (与 WCH 的写 0 清相反——两家族在这个细节上正好镜像);
//! - **yield = CPU 中断 7** ← `cpu_intr_from_cpu_0_map=7`:SYSTEM 的
//!   `cpu_intr_from_cpu_0` 写 1 触发(**电平源**——ISR 里写 0 清除,
//!   否则立刻重入;等价 CH32 的 SWIE,但清除方向相反);
//! - mcause code = CPU 中断号(直接分发,无需查状态寄存器)。
//!
//! ⚠️ 真机核对点(构建级验证 2026-08-23,板上行为待验):
//! ①启动方式——riscv-rt 直链需要 C3 的 **direct boot** 模式(flash 头部
//!   magic;常规经 ROM bootloader 需要 esp镜像头,本口未做);
//! ②复位默认 CPU 时钟 80MHz(env 按此配;PLL 160M 配好后同步改);
//! ③SYSTIMER 16MHz 时基与周期模式寄存器序列;
//! ④中断矩阵 map/cpu_int_enable 的位语义(5 位 map 值 = CPU 中断号);
//! ⑤flash/RAM 基址(0x42000000 缓存映射 / 0x3FC88000)。
mod port;

use super::{CPU_CLOCK_HZ, SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::port::Portable;
use crate::prelude::CriticalSection;
use crate::task::Task;
use core::arch::asm;
use esp32c3::{INTERRUPT_CORE0, SYSTEM, SYSTIMER};

/// tick 用的 CPU 中断号(中断矩阵 1..15 优先级档内任选;6/7 避开
/// esp-idf 惯用的 1-5/19-31 保留区,降低与后续 BSP 的碰撞概率)
pub(crate) const CPU_INT_TICK: u32 = 6;
/// yield 用的 CPU 中断号
pub(crate) const CPU_INT_YIELD: u32 = 7;

/// 配置 SYSTIMER 与中断矩阵(start_scheduler 里、restore_ctx 之前调用)
#[inline]
pub(crate) fn setup_intrrupt() {
    let syst = unsafe { &*SYSTIMER::ptr() };
    // 1. SYSTIMER 使能时钟 + 计数单元 0
    syst.conf.modify(|_r, w| {
        w.clk_en().set_bit().timer_unit0_work_en().set_bit()
    });
    // 2. target0 周期模式:period = 16MHz/1kHz = 16000 ticks(52 位域)
    //    (周期模式下比较器自动重装,ISR 只需 W1C 清标志)
    syst.target0_conf.write(|w| unsafe {
        w.target0_period_mode()
            .set_bit() // 1=周期模式
            .target0_period()
            .bits((SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u32)
            .target0_timer_unit_sel()
            .clear_bit() // 0=unit0
    });
    // 3. 装载比较器 + 开 target0 中断 + target0 开始工作
    syst.comp0_load.write(|w| w.timer_comp0_load().set_bit()); // write-only
    syst.int_ena.write(|w| w.target0_int_ena().set_bit());
    syst.conf.modify(|_r, w| w.target0_work_en().set_bit());

    // 4. 中断矩阵:SYSTIMER target0 → CPU int 6;from_cpu_0 → CPU int 7
    //    (map 寄存器 5 位值 = 目标 CPU 中断号;写 0 = 不映射)
    let mtx = unsafe { &*INTERRUPT_CORE0::ptr() };
    mtx.systimer_target0_int_map
        .write(|w| unsafe { w.bits(CPU_INT_TICK) });
    mtx.cpu_intr_from_cpu_0_map
        .write(|w| unsafe { w.bits(CPU_INT_YIELD) });
    // cpu_int_type 保持复位值 0(全电平触发——from_cpu_0 的正确形态);
    // 使能两个 CPU 中断(位图)
    mtx.cpu_int_enable
        .modify(|_r, w| unsafe { w.bits((1 << CPU_INT_TICK) | (1 << CPU_INT_YIELD)) });
}

/// 清 tick 标志(int_clr 是 W1C——写 1 清除)
#[inline]
pub(crate) fn reset_systick() {
    let syst = unsafe { &*SYSTIMER::ptr() };
    syst.int_clr.write(|w| w.target0_int_clr().set_bit());
}

// critical-section 1.x 的 RISC-V 裸实现(ARM 侧由 cortex-m 的
// critical-section-single-core feature 提供;PAC 引用了 critical-section,
// 这里用本核 mstatus.MIE 实现同一语义——与 Porting::free 一致)
#[no_mangle]
unsafe extern "C" fn _critical_section_1_0_acquire() -> bool {
    let was_enabled = riscv::register::mstatus::read().mie();
    riscv::interrupt::disable();
    was_enabled
}

#[no_mangle]
unsafe extern "C" fn _critical_section_1_0_release(was_enabled: bool) {
    if was_enabled {
        riscv::interrupt::enable();
    }
}

/// ESP32-C3 芯片移植层实现
pub struct Esp32c3Porting;

// port.S 蹦床 `_task_entry_trampoline` 依赖的 Task 布局偏移(失配编译期炸)
const _: () = assert!(core::mem::offset_of!(Task, sp) == 0);
const _: () = assert!(core::mem::offset_of!(Task, entry) == 8);

impl Portable for Esp32c3Porting {
    /// 完全内存屏障
    #[inline]
    fn barrier() {
        unsafe {
            core::arch::asm!("fence iorw, iorw");
        }
    }
    /// 临界区保护(本核 mstatus.MIE)
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

    /// 启动调度器:配置 SYSTIMER/矩阵 → 恢复第一个任务(汇编,不返回)
    fn start_scheduler() -> ! {
        setup_intrrupt();
        log::info!("Start scheduler");
        unsafe { asm!(include_str!("restore_ctx.S"), options(noreturn, raw)) };
    }

    /// 软中断(yield/调度请求):SYSTEM.cpu_intr_from_cpu_0 写 1(电平源,
    /// 置位直到 CPU int 7 的 ISR 写 0 清除)
    #[inline]
    fn irq() {
        let sys = unsafe { &*SYSTEM::ptr() };
        sys.cpu_intr_from_cpu_0.write(|w| w.cpu_intr_from_cpu_0().set_bit());
    }
    /// 清软中断(ISR 入口做;防电平源立刻重入)
    #[inline]
    fn disable_irq() {
        let sys = unsafe { &*SYSTEM::ptr() };
        sys.cpu_intr_from_cpu_0
            .write(|w| w.cpu_intr_from_cpu_0().clear_bit());
    }

    /// 读 SYSTIMER unit0(52 位 @16MHz):update 置位锁存 → 读 hi/lo
    #[inline]
    fn systick() -> u64 {
        let syst = unsafe { &*SYSTIMER::ptr() };
        syst.unit0_op.write(|w| w.timer_unit0_update().set_bit()); // write-only(读回无效)
        // 锁存后读(锁存同步需要几个 APB 周期——构建级按 TRM 序列写,
        // 真机核对点:若读到 0 增加 while 等待 value_valid)
        let hi = syst.unit0_value_hi.read().bits() as u64;
        let lo = syst.unit0_value_lo.read().bits() as u64;
        ((hi & 0xF_FFFF) << 32) | lo // 52 位掩码(高 12 位保留)
    }

    /// 硬件延时(mcycle;复位默认 CPU 80MHz,PLL 后同步改 env)
    #[inline]
    fn delay_us(us: u64) {
        let t0 = riscv::register::mcycle::read64();
        let clock = (us * (CPU_CLOCK_HZ as u64)) / 1_000_000;
        while riscv::register::mcycle::read64().wrapping_sub(t0) <= clock {}
    }

    /// 任务现场(36 字帧,与 port.S 镜像;[35]=mcause 取 CPU_INT_TICK 形态)
    #[inline]
    fn save_context(task: &mut Task) {
        unsafe {
            let sp = task.stack.add(task.stack_size - 1);
            sp.offset(-1)
                .write_volatile(0x8000_0000usize | CPU_INT_TICK as usize); // mcause:中断|6
            sp.offset(-2).write_volatile(0); // 保留槽
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
