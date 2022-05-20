use core::arch::{asm, global_asm};

use cortex_m::interrupt;

use super::{SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::arch::cortex_m::rt::exception;
use crate::isr_sprintln;
use crate::task::scheduler;

// 导入汇编代码
// global_asm!(include_str!("port.S"));

pub(crate) static mut SYSTICKS: u64 = 0;

#[export_name = "print_sp"]
unsafe fn print_sp(sp: usize) {
    isr_sprintln!("sp {:#08x}", sp);
}
#[exception]
unsafe fn SVCall() {
    isr_sprintln!("SVCall");
    asm!(
        "
    ldr r3, =CURRENT_TASK_PTR
    ldr r1, [r3]
    ldr r0, [r1]

    ldmia r0!, {{r4-r11, r14}}
    msr psp, r0
    isb
    # svcall异常返回，使用psp指向的地址出栈，也就是回到任务栈里了
    mov r0, #0
    msr basepri, r0
    cpsie i //使能全局中断
    cpsie f //使能全局异常
    bx r14
    nop
    "
    )
}
#[exception]
unsafe fn PendSV() {
    isr_sprintln!("PendSV begin");

    asm!(
        "
        cpsid i
        dsb
        isb
        bl print_sp
        mrs r0, psp
        isb
        ldr r3, =CURRENT_TASK_PTR
        ldr r2, [r3]
    
        tst r14, #0x10
        it eq
        vstmdbeq r0!, {{s16-s31}}
    
        stmdb r0!, {{r4-r11, r14}}
        str r0, [r2]
    
        stmdb sp!, {{r0, r3}}

        
        
        bl switch_context
       
        cpsie i
        ldmia sp!, {{r0, r3}}
    
        ldr r3, =CURRENT_TASK_PTR
        ldr r1, [r3]
        ldr r0, [r1]
    
        ldmia r0!, {{r4-r11, r14}}
    
        tst r14, #0x10
        it eq
        vldmiaeq r0!, {{s16-s31}}
    
        msr psp, r0
        isb
    
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
        scheduler::systick();
        isr_sprintln!("SysTick");
    });
}

/// 软中断切换任务
#[export_name = "switch_context"]
unsafe fn switch_context(sp: usize) {
    isr_sprintln!("switch_context {:#08x}", sp);
    scheduler::schedule();
}

/// 所有任务的退出函数，调用exit函数即可
pub(crate) unsafe extern "C" fn task_exit() {
    scheduler::exit_current_task();
}
