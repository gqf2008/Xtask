/*
  部分代码来自N200 hal库，Firmware\RISCV\env_Eclipse\start.S
  重写了保存上下文、中断入口函数_irq_handler代码，对大部分指令做了说明
*/

// 保存上文宏
.macro SAVE_CONTEXT_SOFT_IRQ
  // 开辟一块栈空间（栈操作快就是这个道理，加减下SP就行）
  // SP向下偏移36个字，也就是任务被切走后的任务栈顶
  addi sp, sp, -36 * 4 
  // 把x1写入sp偏移4字节的位置，0(sp)位置保留给x0寄存器，很等于0
  sw x1, 1 * 4(sp) //ra(return address)//返回地址
  // 保留x2,x3,x4栈空间，有点浪费～，但是看起来对齐比较舒服
  sw x5, 5 * 4(sp) //t0(temporary)/lr(link register)t[x]临时寄存器
  sw x6, 6 * 4(sp) //t1(temporary)
  sw x7, 7 * 4(sp) //t2(temporary)
  sw x8, 8 * 4(sp) //s0/fp(frame pointer) //s[x]保存寄存器/帧指针
  sw x9, 9 * 4(sp) //s1(saved register)
  sw x10, 10 * 4(sp) //a0(funcation arguments)/rtval(return value)//入参/返回值地址
  sw x11, 11 * 4(sp) //a1(funcation arguments)/rtval(return value)//入参/返回值地址
  sw x12, 12 * 4(sp) //a2(funcation arguments)//a[x]入参地址
  sw x13, 13 * 4(sp) //a3(funcation arguments)
  sw x14, 14 * 4(sp) //a4(funcation arguments)
  sw x15, 15 * 4(sp) //a5(funcation arguments)
  sw x16, 16 * 4(sp) //a6(funcation arguments)
  sw x17, 17 * 4(sp) //a7(funcation arguments)
  sw x18, 18 * 4(sp) //s2(saved register)
  sw x19, 19 * 4(sp) //s3(saved register)
  sw x20, 20 * 4(sp) //s4(saved register)
  sw x21, 21 * 4(sp) //s5(saved register)
  sw x22, 22 * 4(sp) //s6(saved register)
  sw x23, 23 * 4(sp) //s7(saved register)
  sw x24, 24 * 4(sp) //s8(saved register)
  sw x25, 25 * 4(sp) //s9(saved register)
  sw x26, 26 * 4(sp) //s10(saved register)
  sw x27, 27 * 4(sp) //s11(saved register)
  sw x28, 28 * 4(sp) //t3(temporary)
  sw x29, 29 * 4(sp) //t4(temporary)
  sw x30, 30 * 4(sp) //t5(temporary)
  sw x31, 31 * 4(sp) //t6(temporary)

  // CSR寄存器压栈，
  csrr t0, mstatus
  sw t0, 32 * 4(sp)
  csrr t0, mepc // 当前任务的下一条指令的PC地址，也就是当前PC+4
  sw t0, 33 * 4(sp) // PC地址入栈
  csrr t0, 0x7C4 // Bumblebee内核
  sw t0, 34 * 4(sp)
  csrr t0, mcause
  sw t0, 35 * 4(sp)
.endm

// 恢复下文，和上面的上文对齐即可
.macro REsw_CONTEXT_SOFT_IRQ
  lw x1, 1 * 4(sp) // 任务返回地址
  lw x5, 5 * 4(sp)
  lw x6, 6 * 4(sp)
  lw x7, 7 * 4(sp)
  lw x8, 8 * 4(sp)
  lw x9, 9 * 4(sp)
  lw x10, 10 * 4(sp) // a0，任务参数
  lw x11, 11 * 4(sp)
  lw x12, 12 * 4(sp)
  lw x13, 13 * 4(sp)
  lw x14, 14 * 4(sp)
  lw x15, 15 * 4(sp)
  lw x16, 16 * 4(sp)
  lw x17, 17 * 4(sp)
  lw x18, 18 * 4(sp)
  lw x19, 19 * 4(sp)
  lw x20, 20 * 4(sp)
  lw x21, 21 * 4(sp)
  lw x22, 22 * 4(sp)
  lw x23, 23 * 4(sp)
  lw x24, 24 * 4(sp)
  lw x25, 25 * 4(sp)
  lw x26, 26 * 4(sp)
  lw x27, 27 * 4(sp)
  lw x28, 28 * 4(sp)
  lw x29, 29 * 4(sp)
  lw x30, 30 * 4(sp)
  lw x31, 31 * 4(sp)

  // CSR寄存器出栈
  lw t0, 32 * 4(sp)
  csrw mstatus, t0 // 恢复mstatus
  lw t0, 33 * 4(sp)
  csrw mepc, t0 // 恢复任务PC到mepc寄存器，最后由mret伪指令恢复到PC寄存器
  lw t0, 34 * 4(sp)
  csrw 0x7C4, t0 // Bumblebee内核自定义寄存器
  lw t0, 35 * 4(sp)
  csrw mcause, t0 // 恢复mcause
  // SP指针上移，释放当前任务的栈空间，使SP指向任务被切换前的栈顶
  addi sp, sp, 4 * 36 
.endm

// 当前任务全局指针，在RUST代码中定义
.extern CURRENT_TASK_PTR 

// IRQ entry point
.section .text.irq
.option push
.option norelax
.align 2
.option pop
.global _irq_handler
_irq_handler:
  // 保存上文到任务堆栈
  SAVE_CONTEXT_SOFT_IRQ
  // 把当前任务的SP保存到任务栈
  // CURRENT_TASK_PTR是当前任务的指针，指向了当前运行任务的地址，在RUST代码里维护
  lw t0, CURRENT_TASK_PTR 
	sw sp, 0x0(t0)
  // 切到中断栈
  csrrw sp, mscratch, sp
  // Bumblebee内核自定义寄存器实现了中断嵌套，跳转到中断向量表地址
  // 执行这条指令后全局中断被打开mie=1，通过ra地址指向这条指令自己
  // 实现了中断嵌套的功能，所以向量表里的定义的ISR服务函数不需要再
  // 开关中断，中断服务程序不会被其他中断打断（除非发了不可屏蔽异常，
  // 例如硬件错误），中断服务函数执行的CPU被独占着，所以中断服务函数
  // 执行效率决定了多任务的响应能力、吞吐量、CPU利用率等关键指标
  csrrw ra, 0x7ED, ra
  // 退出0x7ED后关闭全局中断mie=0
  // csrc表示清除CSR寄存器指定位的值
  // 这里就表示清除mstatus寄存器mie位的值
  csrc mstatus, 0x00000008
  // 回到任务栈
  csrrw sp, mscratch, sp
  // 这个时候已经切换到新的任务，需要从新任务的栈空间恢复下文，栈顶地址就保存在任务第一个变量里
  lw t0, CURRENT_TASK_PTR
  // 这个任务栈地址要么是第一次创建的时候写入任务块的，要么是被切换走的时候写入任务块的
  lw sp, 0x0(t0)
  // 恢复下文到CPU寄存器
  REsw_CONTEXT_SOFT_IRQ
  // mepc值已经在上面恢复，执行这条伪指令将PC指向新任务断点处
  // 同时mie从mpie恢复到中断前的状态，也就是要恢复到进入中断函数前的中断状态
  mret
.section .text.vectors, "ax"
.option push
.option norelax
.align 9 // Bumblebee内核指令集架构要求至少64字节对齐，这里是2的9次方对齐
.option pop
// 中断向量表
vectors:
    .word     0
    .word     0
    .word     0
    .word     INT_SFT // 中断号=3，可以读取mcause寄存器code域获取
    .word     0
    .word     0
    .word     0
    .word     INT_TMR // 中断号=7，可以读取mcause寄存器code域获取
    .word     0
    .word     0
    .word     0
    .word     0
    .word     0
    .word     0
    .word     0
    .word     0
    .word     0
    .word     INT_BWEI
    .word     INT_PMOVI
    .word     WWDGT
    .word     EXTI_LVD
    .word     TAMPER
    .word     RTC
    .word     FMC
    .word     RCU
    .word     EXTI_LINE0
    .word     EXTI_LINE1
    .word     EXTI_LINE2
    .word     EXTI_LINE3
    .word     EXTI_LINE4
    .word     DMA0_CHANNEL0
    .word     DMA0_CHANNEL1
    .word     DMA0_CHANNEL2
    .word     DMA0_CHANNEL3
    .word     DMA0_CHANNEL4
    .word     DMA0_CHANNEL5
    .word     DMA0_CHANNEL6
    .word     ADC0_1
    .word     CAN0_TX
    .word     CAN0_RX0
    .word     CAN0_RX1
    .word     CAN0_EWMC
    .word     EXTI_LINE9_5
    .word     TIMER0_BRK
    .word     TIMER0_UP
    .word     TIMER0_TRG_CMT
    .word     TIMER0_CHANNEL
    .word     TIMER1
    .word     TIMER2
    .word     TIMER3
    .word     I2C0_EV
    .word     I2C0_ER
    .word     I2C1_EV
    .word     I2C1_ER
    .word     SPI0
    .word     SPI1
    .word     USART0
    .word     USART1
    .word     USART2
    .word     EXTI_LINE15_10
    .word     RTC_ALARM
    .word     USBFS_WKUP
    .word     0
    .word     0
    .word     0
    .word     0
    .word     0
    .word     EXMC // not present in Reference Manual but present in vendor HAL
    .word     0
    .word     TIMER4
    .word     SPI2
    .word     UART3
    .word     UART4
    .word     TIMER5
    .word     TIMER6
    .word     DMA1_CHANNEL0
    .word     DMA1_CHANNEL1
    .word     DMA1_CHANNEL2
    .word     DMA1_CHANNEL3
    .word     DMA1_CHANNEL4
    .word     0
    .word     0
    .word     CAN1_TX
    .word     CAN1_RX0
    .word     CAN1_RX1
    .word     CAN1_EWMC
    .word     USBFS
/*
    Trap entry point (_start_trap)
*/
.section .trap, "ax"
.option push
.option norelax
.align 6  // ISA要求至少64字节对齐，align表示2的6次方
.option pop
.global _start_trap
// 异常函数入口，这个函数在riscv-rt link.x链接脚本中定义
_start_trap:
    // 进入异常只需要保存/恢复16个寄存器，
    // 因为riscv指令集架构没有要求硬件实现压/出栈
    // 但是CH部分芯片实现了硬件压栈
    addi sp, sp, -16*4

    sw ra, 0*4(sp)
    sw t0, 1*4(sp)
    sw t1, 2*4(sp)
    sw t2, 3*4(sp)
    sw t3, 4*4(sp)
    sw t4, 5*4(sp)
    sw t5, 6*4(sp)
    sw t6, 7*4(sp)
    sw a0, 8*4(sp)
    sw a1, 9*4(sp)
    sw a2, 10*4(sp)
    sw a3, 11*4(sp)
    sw a4, 12*4(sp)
    sw a5, 13*4(sp)
    sw a6, 14*4(sp)
    sw a7, 15*4(sp)

    add a0, sp, zero
    // 无条件跳转到RUST函数，ra被设置为这个函数地址，应该是不会再回来了
    // 除非RUST把ra地址再设置ra+4然后ret才可以回来
    jal ra, _start_trap_rust

    lw ra, 0*4(sp)
    lw t0, 1*4(sp)
    lw t1, 2*4(sp)
    lw t2, 3*4(sp)
    lw t3, 4*4(sp)
    lw t4, 5*4(sp)
    lw t5, 6*4(sp)
    lw t6, 7*4(sp)
    lw a0, 8*4(sp)
    lw a1, 9*4(sp)
    lw a2, 10*4(sp)
    lw a3, 11*4(sp)
    lw a4, 12*4(sp)
    lw a5, 13*4(sp)
    lw a6, 14*4(sp)
    lw a7, 15*4(sp)

    addi sp, sp, 16*4
    mret
.section .text
.global _setup_interrupts
// 配置向量表、异常、中断入口，这个函数在riscv-rt link.x链接脚本中定义
_setup_interrupts:
    // Set the the NMI base to share with mtvec by setting 0x7D0
    // 0x7D0是Bumblebee内核自定义寄存器，0x200是NMI（不可屏蔽中断）基地址，与mtvec共享
    li t0, 0x200
    csrs 0x7D0, t0

    // Set the mtvt，
    // 0x307是Bumblebee内核自定义寄存器，保存向量表入口地址
    la t0, vectors
    csrw 0x307, t0

    // Set the mtvt2 and enable it
    //0x7EC是Bumblebee内核自定义寄存器，保存中断入口地址
    la t0, _irq_handler
    csrw 0x7EC, t0
    csrs 0x7EC, 0x1

    // Enable ECLIC and set trap handler
    // 设置异常入口地址
    la t0, _start_trap
    andi t0, t0, -64
    ori t0, t0, 3
    csrw mtvec, t0

    ret

// GD32外设预定义中断符号，与中断向量表里的一致，应用层实现后由连接器绑定
// .weak 表示使符号对链接器全局可见，该符号具有较弱的绑定语义
.weak INT_BWEI
.weak INT_PMOVI
.weak WWDGT
.weak EXTI_LVD
.weak TAMPER
.weak RTC
.weak FMC
.weak RCU
.weak EXTI_LINE0
.weak EXTI_LINE1
.weak EXTI_LINE2
.weak EXTI_LINE3
.weak EXTI_LINE4
.weak DMA0_CHANNEL0
.weak DMA0_CHANNEL1
.weak DMA0_CHANNEL2
.weak DMA0_CHANNEL3
.weak DMA0_CHANNEL4
.weak DMA0_CHANNEL5
.weak DMA0_CHANNEL6
.weak ADC0_1
.weak CAN0_TX
.weak CAN0_RX0
.weak CAN0_RX1
.weak CAN0_EWMC
.weak EXTI_LINE9_5
.weak TIMER0_BRK
.weak TIMER0_UP
.weak TIMER0_TRG_CMT
.weak TIMER0_CHANNEL
.weak TIMER1
.weak TIMER2
.weak TIMER3
.weak I2C0_EV
.weak I2C0_ER
.weak I2C1_EV
.weak I2C1_ER
.weak SPI0
.weak SPI1
.weak USART0
.weak USART1
.weak USART2
.weak EXTI_LINE15_10
.weak RTC_ALARM
.weak USBFS_WKUP
.weak EXMC
.weak TIMER4
.weak SPI2
.weak UART3
.weak UART4
.weak TIMER5
.weak TIMER6
.weak DMA1_CHANNEL0
.weak DMA1_CHANNEL1
.weak DMA1_CHANNEL2
.weak DMA1_CHANNEL3
.weak DMA1_CHANNEL4
.weak CAN1_TX
.weak CAN1_RX0
.weak CAN1_RX1
.weak CAN1_EWMC
.weak USBFS
