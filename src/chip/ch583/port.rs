//! CH583 中断胶水:真 tick 处理与切换入口。
//! 与 ch32v103 的差异:yield/抢占请求走 **`SWI_IRQn=14`**(官方三套 RTOS 移植的做法),
//! 不再复用 SysTick 入口/`CTLR.SWIE`——SysTick 入口现在只由真 tick 触发。

use super::Ch583Porting;
use crate::port::Portable;
use crate::task::scheduler;
use core::arch::global_asm;
use riscv::register::mcause;

global_asm!(include_str!("port.S"), options(raw));

/// SysTick 处理(port.S `_start_trap` 经 mcause=12 进入;工作在中断栈)。
/// 只有真 tick 会走这里(软中断换成了 `SWI_IRQn=14`,见模块头注):
/// 清 `SR.CNTIF`(WCH 写 0 清)+ 时间账(`scheduler::systick`),
/// 返回后经 port.S 的公共出口 `switch_context` 完成切换。
///
/// `CNTIF` 仍保留一层判断:万一 SysTick IRQ 被其它路径(或残留)拉起,
/// 没有 CNTIF 就不做时间账。
#[no_mangle]
unsafe extern "C" fn SysTick() {
    // CH583:SR 在 SYSTICK+4(直址;零 PAC 依赖)
    let sr = (super::STK_BASE + 4) as *const u32;
    if unsafe { sr.read_volatile() & 1 } != 0 {
        // 真 tick:清标志 + 时间账
        super::reset_systick();
        if scheduler::systick() {
            // 需要切换——trap 出口统一调度
        }
    }
}

/// 软中断等价入口:port.S 在 SysTick 处理后调用,执行调度与切换
#[no_mangle]
unsafe extern "C" fn switch_context() {
    Ch583Porting::disable_irq(); // 清 SWI_IRQn=14 的 pending(防残留立即重入)
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
