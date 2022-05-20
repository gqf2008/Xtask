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
        ldmia r0!, {{r4-r11}}
        //栈顶写到psp，进入用户任务后从这里开始cpu自动恢复剩下的寄存器（pc值等）
        msr psp, r0
        isb
        # 屏蔽中断为0
        mov r0, #0
        msr basepri, r0
        // 退出中断函数，使得sp=psp，进入用户模式（thumb）
        // 进入用户任务后，其他寄存器自动出栈，恢复pc值等
        orr r14, #0xd
        bx r14
    "
    )
}
#[exception]
unsafe fn PendSV() {
    asm!(
        "
        // 硬件自动压栈部分xPSR,r15(PC),r14(LR),r12,r3,r2,r1,r0
        // 将当前任务psp保存到r0，开始保存上文
        mrs r0, psp
        isb
        
        // 手动压栈
        stmdb r0!, {{r4-r11}}
        // 加载用户任务地址[CURRENT_TASK_PTR]->[TASK]->[SP]
        ldr r3, =CURRENT_TASK_PTR
        // 任务地址存入r2
         ldr r2, [r3]
        // 保存栈顶到任务第一个sp字段，即任务栈顶指针地址
        str r0, [r2]
        // 这里开始操作msp，r14，r3=CURRENT_TASK_PTR入主栈备份
        stmdb sp!, {{r3, r14}}

        ///////////////////////////////////////////////////////////
        // 关全局中断
        cpsid i
        isb
        // 切换任务
        bl switch_context
        // 开全局中断
        cpsie i
        isb
        /////////////////////////////////////////////////////////

        // 恢复下文
        // 从主栈恢复r0，r3
        ldmia sp!, {{r3, r14}}
        // 从r3=CURRENT_TASK_PTR取新任务地址
        ldr r1, [r3]
        // 新任务栈顶指针
        ldr r0, [r1]
        // 恢复手动保存的寄存器
        ldmia r0!, {{r4-r11}}
        // 设置sp=psp
        msr psp, r0
        isb
        // mov r14, #0xfffffffd
        // 回到任务模式，硬件自动恢复pc
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
