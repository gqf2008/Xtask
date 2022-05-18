// 调度器启动时恢复第一个任务到CPU寄存器

.extern CURRENT_TASK_PTR #全局任务指针
.extern switch_context //任务切换函数

// 恢复第一个任务
.global SVCall 
SVCall:
    ldr r3, = CURRENT_TASK_PTR
    ldr r1, [r3]
    ldr r0, [r1]
    ldmia r0!, {r4-r11}
    msr psp, r0
    isb 
    cpsie i # 开中断
    cpsie f # 开异常
    dsb
    isb
    orr r14, #0xd
    bx r14

// 上下文切换
.global PendSV
PendSV:
    msr r0, psp
    isb
    ldr r3, = CURRENT_TASK_PTR
    ldr r2, [r3]

    stmdb r0!, {r4-r11}
    str r0, [r2]

    stmdb sp!, {r3,r14}
    cpsid i # 关中断
    dsb
    isb
    bl switch_context
    cpsie i # 开中断
    dsb
    isb
    ldmia sp!, {r3,r14}

    ldr r1, [r3]
    ldr r0, [r1]
    ldmia r0!, {r4-r11}
    msr psp, r0
    isb
    bx r14
    nop
    