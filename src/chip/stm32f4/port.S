
/*
  调度器启动时恢复第一个任务到CPU寄存器
*/

.extern CURRENT_TASK_PTR # 全局任务指针
.extern switch_context # 任务切换函数

# 恢复第一个任务
.section .text
.global SVCall 
SVCall:
    bl print_sp 
    ldr r3, =CURRENT_TASK_PTR
    ldr r1, [r3]
    ldr r0, [r1]
    bl print_sp 
    ldmia r0!, {{r4-r11, r14}}
    msr psp, r0
    isb
    mov r0, #0
    msr	basepri, r0
    # 返回任务模式，sp使用psp里的地址
    # orr r14, #0xd
    # svcall异常返回，使用psp指向的地址出栈，也就是回到任务栈里了
    bx r14
    .align 4

# 上下文切换
.section .text
.global PendSV
PendSV:
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
    
    cpsid i 
    dsb
    isb
    bl switch_context
   
    cpsie i 
    ldmia sp!, {{r0, r3}}

    ldr r1, [r3]
    ldr r0, [r1]

    ldmia r0!, {{r4-r11, r14}}

    tst r14, #0x10				
    it eq
    vldmiaeq r0!, {{s16-s31}}

    msr psp, r0
    isb
    bx r14
    .align 4
    