//! CH572 中断胶水:真 tick 处理与切换入口。
//! 与 ch583/port.rs 完全同构——yield/抢占请求走 **`SWI_IRQn=14`**
//! (官方三套 RTOS 移植的做法,见 `Ch572Porting::irq()`),SysTick 入口只由真 tick 触发。

use super::Ch572Porting;
use crate::port::Portable;
use crate::task::scheduler;
use core::arch::global_asm;
use riscv::register::mcause;

global_asm!(include_str!("port.S"), options(raw));

/// SysTick 处理(port.S `_start_trap` 经 mcause=12 进入;工作在中断栈)。
///
/// 一个入口两种来源,靠 `STK_SR.CNTIF` 区分(CH572 的 CNTIF(bit0)与 SWIE(bit31)
/// 同在 `SR`):
/// - **CNTIF=1(真 tick)**:清标志(WCH 惯例写 0 清,待上板复核)+ 时间账
///   (`scheduler::systick`),返回 true 则请求切换(trap 出口统一调度);
/// - **CNTIF=0(SWIE 触发)**:纯调度请求(`Porting::irq()` 的软中断)——
///   不做时间账(否则 yield 一次时间就快进一格)。
///
/// 两条路径都在返回后经 port.S 的公共出口 `switch_context` 完成切换。
#[no_mangle]
unsafe extern "C" fn SysTick() {
    // CH572:SR 在 SYSTICK+4(直址;零 PAC 依赖)
    let sr = (super::STK_BASE + 4) as *const u32;
    if unsafe { sr.read_volatile() & 1 } != 0 {
        // 真 tick:清标志 + 时间账
        super::reset_systick();
        if scheduler::systick() {
            // 需要切换——不需要 SWIE,trap 出口统一调度
        }
    }
    // SWIE 路径:什么都不做,直接调度(port.S 公共出口)
}

/// 软中断等价入口:port.S 在 SysTick 处理后调用,执行调度与切换
#[no_mangle]
unsafe extern "C" fn switch_context() {
    Ch572Porting::disable_irq(); // 防 SWIE(SR.bit31)残留立即重入
    scheduler::schedule();
}

/// 所有任务的退出函数
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn task_exit() {
    scheduler::exit_current_task();
}

/// 供汇编引用的符号占位(避免未使用警告;真正实现在上面)
#[allow(dead_code)]
fn _mcause_helper() {
    let _ = mcause::read();
}
