use super::{SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::arch::cortex_m::rt::exception;
use crate::task::scheduler;

pub(crate) static mut SYSTICKS: u64 = 0;

/// 系统节拍器中断
#[exception]
unsafe fn SysTick() {
    const TICKS: u32 = SYSTICK_CLOCK_HZ as u32 / TICK_CLOCK_HZ as u32;
    let tick = core::ptr::read_volatile(&SYSTICKS);
    core::ptr::write_volatile(&mut SYSTICKS, tick + TICKS as u64);
    scheduler::systick();
}

/// 软中断切换任务
#[export_name = "switch_context"]
unsafe fn switch_context() {
    scheduler::schedule();
}

/// 所有任务的退出函数，调用exit函数即可
pub(crate) unsafe extern "C" fn task_exit() {
    scheduler::exit_current_task();
}
