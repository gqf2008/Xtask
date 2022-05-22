use core::arch::asm;

use cortex_m::interrupt;

use super::{SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::task::scheduler;
use crate::{isr_sprint, isr_sprintln, sprintln};
use cortex_m_rt::exception;
pub(crate) static mut SYSTICKS: u64 = 0;

// #[inline]
// pub fn lr() -> u32 {
//     let r;
//     unsafe { asm!("mov {}, lr", out(reg) r, options(nomem, nostack, preserves_flags)) };
//     r
// }
// #[inline]
// pub fn pc() -> u32 {
//     let r;
//     unsafe { asm!("mov {}, pc", out(reg) r, options(nomem, nostack, preserves_flags)) };
//     r
// }
// #[inline]
// pub fn is_enable() -> bool {
//     cortex_m::register::primask::read().is_active()
// }

// #[inline]
// pub fn msp() -> u32 {
//     let r;
//     unsafe { asm!("mrs {}, MSP", out(reg) r, options(nomem, nostack, preserves_flags)) };
//     r
// }

#[exception]
unsafe fn SVCall() {
    asm!(
        "
        ldr r3, =CURRENT_TASK_PTR
        ldr r1, [r3]
        ldr r0, [r1]
        ldmia r0!, {{r4-r11}}
        msr psp, r0
        isb
        mov r14, #0xfffffffd
        bx r14
    "
    )
}

/// 如果由Systict异常触发，那么xPSR、pc、sp等自动保存在主栈中
/// 如果由任务触发，那么xPSR、pc、sp等自动保存在任务栈中
#[exception]
unsafe fn PendSV() {
    /// 不要调用任何函数，否则会改变r14的值，导致无法回到任务栈中
    asm!(
        "
            mrs r0, psp
            isb
            ldr r3, =CURRENT_TASK_PTR
            ldr r2, [r3]
            stmdb r0!, {{r4-r11}}
            str r0, [r2]
            stmdb sp!, {{r3, r14}}
            cpsid i
            cpsid f
            bl switch_context
            cpsie f
            cpsie i
            ldmia sp!, {{r3, r14}}
            ldr r1, [r3]
            ldr r0, [r1]
            ldmia r0!, {{r4-r11}}
            msr psp, r0
            isb
            //恢复msp
            ldr r0, =0xE000ED08 // 向量表地址，将 0xE000ED08 加载到 R0
            ldr r0, [r0] //将 0xE000ED08 中的值，也就是向量表的实际地址加载到 R0
            ldr r0, [r0] //根据向量表实际存储地址，取出向量表中的第一项,向量表第一项存储主堆栈指针MSP的初始值
            msr msp, r0 //将堆栈地址写入主堆栈指针
            bx r14
            "
    );
}

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
