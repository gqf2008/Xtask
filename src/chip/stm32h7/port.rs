use core::arch::asm;

use cortex_m::interrupt;

use super::{SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::arch::cortex_m::rt::exception;
use crate::task::scheduler;
pub(crate) static mut SYSTICKS: u64 = 0;

#[exception]
unsafe fn SVCall() {
    asm!(
        "
        // 加载任务栈顶地址到r0
        ldr r3, =CURRENT_TASK_PTR
        // 用户任务地址
         ldr r1, [r3]
        // sp地址写入r0，用户任务第一个字段为任务sp地址
        ldr r0, [r1]
        //从任务栈手动恢复任务状态
        ldmia r0!, {r4-r11, r14}
        //栈顶写到psp，进入用户任务后从这里开始cpu自动恢复剩下的寄存器（pc值等）
        msr psp, r0
        isb
        # 屏蔽中断为0
        mov r0, #0
        msr basepri, r0
        // 退出中断函数，使得sp=psp，进入用户模式（thumb）
        // 进入用户任务后，其他寄存器自动出栈，恢复pc值等
        bx r14
    ",
        options(raw)
    )
}
#[exception]
unsafe fn PendSV() {
    asm!(
        "
        mrs r0, psp
        isb
        tst r14, #0x10
        it eq
        vstmdbeq r0!, {s16-s31}
        // 手动压栈
        stmdb r0!, {r4-r11, r14}
        ldr r3, =CURRENT_TASK_PTR
        ldr r2, [r3]
        str r0, [r2]
        stmdb sp!, {r0, r3}
        ///////////////////////////////////////////////////////////
        // 关全局中断
        cpsid i
        cpsid f
        isb
        // 切换任务
        bl switch_context
        // 开全局中断
        cpsie f
        cpsie i
        /////////////////////////////////////////////////////////
        ldmia sp!, {r0, r3}
        ldr r1, [r3]
        ldr r0, [r1]
        ldmia r0!, {r4-r11, r14}
        // FPU处理
        tst r14, #0x10
        it eq
        vldmiaeq r0!, {s16-s31}
        msr psp, r0
        isb
        ldr r0, =0xE000ED08 // 向量表地址，将 0xE000ED08 加载到 R0
        ldr r0, [r0] //将 0xE000ED08 中的值，也就是向量表的实际地址加载到 R0
        ldr r0, [r0] //根据向量表实际存储地址，取出向量表中的第一项,向量表第一项存储主堆栈指针MSP的初始值
        msr msp, r0 //将堆栈地址写入主堆栈指针
        bx r14
        ",options(raw)
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
unsafe fn switch_context() {
    scheduler::schedule();
}

/// 所有任务的退出函数，调用exit函数即可
pub(crate) unsafe extern "C" fn task_exit() {
    scheduler::exit_current_task();
}
