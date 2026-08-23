//! ESP32-C3 中断胶水:tick(CPU int 6)与 yield(CPU int 7)处理。

use super::Esp32c3Porting;
use crate::port::Portable;
use crate::task::scheduler;
use core::arch::global_asm;

global_asm!(include_str!("port.S"), options(raw));

/// tick 处理(port.S 经 mcause=6 进入;工作在中断栈)。
/// SYSTIMER 周期模式自动重装——这里只 W1C 清标志 + 时间账
#[no_mangle]
unsafe extern "C" fn TickHandler() {
    super::reset_systick();
    scheduler::systick();
    // 需要切换时不必显式请求——trap 出口统一走切换路径
}

/// yield 处理(port.S 经 mcause=7 进入;工作在中断栈)。
/// from_cpu_0 是**电平源**:只做清除(mret 后立刻重入就死循环了);
/// 调度统一走 port.S 公共出口的 switch_context
#[no_mangle]
unsafe extern "C" fn YieldHandler() {
    Esp32c3Porting::disable_irq(); // 清 from_cpu_0(电平)
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
