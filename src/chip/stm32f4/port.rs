use crate::arch::cortex_m::rt::exception;
use crate::port::Portable;
use crate::task::scheduler;

use super::STM32F4Porting;

/// 系统节拍器中断
#[exception]
unsafe fn SysTick() {
    STM32F4Porting::disable_interrupt();
    scheduler::systick();
    STM32F4Porting::enable_interrupt();
}

/// 软中断切换任务
#[export_name = "switch_context"]
unsafe fn switch_context() {
    scheduler::schedule();
}
