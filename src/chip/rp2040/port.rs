use core::arch::global_asm;
use cortex_m::interrupt;

use super::{SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::task::scheduler;
// use crate::{isr_sprint, isr_sprintln, sprintln};
use cortex_m_rt::exception;
pub(crate) static mut SYSTICKS: u64 = 0;

global_asm!(include_str!("port.S"));

/// 系统节拍器中断
#[exception]
unsafe fn SysTick() {
    const TICKS: u32 = SYSTICK_CLOCK_HZ as u32 / TICK_CLOCK_HZ as u32;
    interrupt::free(|_| {
        let tick = core::ptr::read_volatile(&SYSTICKS);
        core::ptr::write_volatile(&mut SYSTICKS, tick + TICKS as u64);
        if scheduler::systick() {
            cortex_m::peripheral::SCB::set_pendsv();
        }
    });
}

/// 软中断切换任务
#[export_name = "switch_context"]
unsafe extern "C" fn switch_context() {
    scheduler::schedule();
}

/// 所有任务的退出函数，调用exit函数即可
pub(crate) unsafe extern "C" fn task_exit() {
    scheduler::exit_current_task();
}
