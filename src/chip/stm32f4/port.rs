use core::arch::{asm, global_asm};

use super::{SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::arch::cortex_m::rt::exception;
use crate::isr_sprintln;
use crate::task::scheduler;

// // 导入汇编代码
// global_asm!(include_str!("switch.S"));

pub(crate) static mut SYSTICKS: u64 = 0;

#[exception]
unsafe fn SVCall() {
    isr_sprintln!("SVCall");
    asm!(
        "
    ldr r3, = CURRENT_TASK_PTR
    ldr r1, [r3]
    ldr r0, [r1]
    ldmia r0!, {{r4-r11}}
    msr psp, r0
    isb 
    cpsie i // 开中断
    cpsie f // 开异常
    dsb
    isb
    # 返回任务模式，sp使用psp里的地址
    orr r14, #0xd 
    # svcall异常返回，使用psp指向的地址出栈，也就是回到任务栈里了
    bx r14
    "
    )
}
#[exception]
unsafe fn PendSV() {
    isr_sprintln!("PendSV");
    asm!(
        "
    mrs r0, psp
    isb
    ldr r3, = CURRENT_TASK_PTR
    ldr r2, [r3]

    stmdb r0!, {{r4-r11}}
    str r0, [r2]

    stmdb sp!, {{r3,r14}}
    cpsid i // 关中断
    dsb
    isb
    bl switch_context
    cpsie i // 开中断
    dsb
    isb
    ldmia sp!, {{r3,r14}}

    ldr r1, [r3]
    ldr r0, [r1]
    ldmia r0!, {{r4-r11}}
    msr psp, r0
    isb
    bx r14
    nop
    "
    )
}

/// 系统节拍器中断
#[exception]
unsafe fn SysTick() {
    isr_sprintln!("SysTick");
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
