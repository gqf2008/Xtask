
.extern CURRENT_TASK_PTR 
.extern switch_context
# .section .text.exception
# .align 3
# .global SVCall
# SVCall
#     ldr r3, =CURRENT_TASK_PTR
#     ldr r1, [r3]
#     ldr r0, [r1]
#     ldmia r0!, {{r4-r11}}
#     msr psp, r0
#     isb
#     mov r14, #0xfffffffd
#     bx r14

.section .text.exception
.align 3
.global PendSV
PendSV:
    mrs r0, psp
    ldr r3, =CURRENT_TASK_PTR 
    ldr r2, [r3]
    subs r0, #32  
    str r0, [r2] 
    stmia r0!, {{r4-r7}}
    mov r4, r8
    mov r5, r9
    mov r6, r10
    mov r7, r11
    stmia r0!, {{r4-r7}}
    push {{r3,r14}}
    cpsid i
    bl switch_context
    cpsie i
    pop {{r2, r3}} 
    ldr r1, [r2]
    ldr r0, [r1] 
    adds r0, #16 
    ldmia r0!, {{r4-r7}} 
    mov r8, r4
    mov r9, r5
    mov r10, r6
    mov r11, r7
    msr psp, r0   
    subs r0, #32 
    ldmia r0!, {{r4-r7}} 

    ldr r0, =0xE000ED08 // 向量表地址，将 0xE000ED08 加载到 R0
    ldr r0, [r0] //将 0xE000ED08 中的值，也就是向量表的实际地址加载到 R0
    ldr r0, [r0] //根据向量表实际存储地址，取出向量表中的第一项,向量表第一项存储主堆栈指针MSP的初始值
    msr msp, r0 //将堆栈地址写入主堆栈指针
    bx r3
