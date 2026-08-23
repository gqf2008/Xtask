//! QEMU virt 机中断胶水:tick(mcause=7)与 yield(mcause=3)处理。

use super::QemuRiscvPorting;
use crate::port::Portable;
use crate::task::scheduler;
use core::arch::global_asm;

global_asm!(include_str!("port.S"), options(raw));

/// tick 处理:重装 MTIMECMP + 时间账
#[no_mangle]
unsafe extern "C" fn TickHandler() {
    super::reset_systick();
    scheduler::systick();
    // 需要切换时不必显式请求——trap 出口统一走切换路径
}

/// yield 处理:清 MSIP(电平源,防 mret 后立刻重入);
/// 调度统一走 port.S 公共出口的 switch_context
#[no_mangle]
unsafe extern "C" fn YieldHandler() {
    QemuRiscvPorting::disable_irq(); // MSIP 写 0
}

/// 软中断等价入口:port.S 公共出口调用,执行调度与切换
#[no_mangle]
unsafe extern "C" fn switch_context() {
    scheduler::schedule();
}

/// 所有任务的退出函数
pub(crate) unsafe extern "C" fn task_exit() {
    scheduler::exit_current_task();
}
