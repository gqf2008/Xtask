/*
  The code is based on vendor provided HAL libraries.
  Most code come from Firmware\RISCV\env_Eclipse\start.S
*/
//sw写内存指令
#define STORE    sw
//加载内存指令
#define LOAD     lw

#define LOG_REGBYTES 2
//寄存器字长，4字节
#define REGBYTES (1 << LOG_REGBYTES)

//mstatus状态寄存器内存映射地址
#define CSR_MSTATUS     0x300
//mtvt寄存器，存放向量表地址
#define CSR_MTVT        0x307
//mepc寄存器，存放进入中断服务前PC+4地址，执行mret伪指令后会用它恢复PC到寄存器
#define CSR_MEPC        0x341
//mcause寄存器，存放异常/中断号和原因
#define CSR_MCAUSE      0x342
//mtvt2寄存器，存放中断函数入口地址
#define CSR_MTVT2       0x7EC
//Bumblebee芯片自定义CSR寄存器，实现中断嵌套
#define CSR_JALMNXTI    0x7ED
//Bumblebee芯片自定义CSR寄存器，
#define CSR_PUSHMCAUSE  0x7EE
//Bumblebee芯片自定义CSR寄存器，
#define CSR_PUSHMEPC    0x7EF
//Bumblebee芯片自定义CSR寄存器，
#define CSR_PUSHMSUBM   0x7EB
//Bumblebee芯片自定义CSR寄存器，
#define CSR_MMISC_CTL   0x7d0
//Bumblebee芯片自定义CSR寄存器
#define CSR_MSUBM       0x7c4
//mstatus状态寄存器mie位
#define MSTATUS_MIE     0x00000008

// 关闭全局中断宏
.macro DISABLE_MIE
  csrc CSR_MSTATUS, MSTATUS_MIE
.endm

// 保存上文宏
.macro SAVE_CONTEXT_SOFT_IRQ
  addi sp, sp, -36 * 4 //开辟一块栈空间，SP向下偏移36个字，也就是任务被切走后的任务栈顶
  sw x1, 1 * 4(sp) //ra(return address)
  sw x5, 5 * 4(sp) //t0(temporary)/lr(link register)
  sw x6, 6 * 4(sp) //t1(temporary)
  sw x7, 7 * 4(sp) //t2(temporary)
  sw x8, 8 * 4(sp) //s0/fp(frame pointer)
  sw x9, 9 * 4(sp) //s1(saved register)
  sw x10, 10 * 4(sp) //a0(funcation arguments)/rtval(return value)
  sw x11, 11 * 4(sp) //a1(funcation arguments)/rtval(return value)
  sw x12, 12 * 4(sp) //a2(funcation arguments)
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

  //CSR寄存器压栈，
  csrr t0, mstatus
  sw t0, 32 * 4(sp)
  csrr t0, mepc //当前任务的下一条指令的PC地址，也就是当前PC+4
  sw t0, 33 * 4(sp) //PC地址入栈
  csrr t0, 0x7c4
  sw t0, 34 * 4(sp)
  csrr t0, mcause
  sw t0, 35 * 4(sp)
.endm

.macro RESTORE_CONTEXT_SOFT_IRQ
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
  csrw mstatus, t0 //恢复mstatus
  lw t0, 33 * 4(sp)
  csrw mepc, t0 // 恢复任务PC到mepc寄存器，最后由mret伪指令恢复到PC寄存器
  lw t0, 34 * 4(sp)
  csrw 0x7c4, t0 // 自定义寄存器
  lw t0, 35 * 4(sp)
  csrw mcause, t0 // 恢复mcause
  //SP指针上移，释放当前任务的栈空间，使SP指向任务被切换前的栈顶
  addi sp, sp, 4 * 36 
.endm

.extern CURRENT_TASK_PTR //当前任务指针

// IRQ entry point
.section .text.irq
.option push
.option norelax
.align 2
.option pop
.global _irq_handler
_irq_handler:
  //保存上文到任务堆栈
  SAVE_CONTEXT_SOFT_IRQ
  //把当前任务的SP保存到任务栈
  lw t0, CURRENT_TASK_PTR //CURRENT_TASK_PTR就是当前任务的指针，指向了当前运行任务的地址，这个指针在RUST代码里维护
	sw sp, 0x0(t0)
  //切到中断栈
  csrrw sp, mscratch, sp
  //这个自定义寄存器实现了中断嵌套，跳转到中断向量表地址
  //执行这条指令后全局中断被打开，通过ra地址指向这条指令自己
  //实现了中断嵌套的功能
  csrrw ra, CSR_JALMNXTI, ra
  //退出CSR_JALMNXTI后关闭全局中断
  DISABLE_MIE
  //回到任务栈
  csrrw sp, mscratch, sp
  //这个时候已经切换到新的任务，需要从新任务的栈空间恢复下文，栈顶地址就保存在任务第一个变量里
  lw t0, CURRENT_TASK_PTR
  //这个任务栈地址要么是第一次创建的时候写入任务块的，要么是被切换走的时候写入任务块的
  lw sp, 0x0(t0)
  //恢复下文到CPU寄存器
  RESTORE_CONTEXT_SOFT_IRQ
  //mepc值已经在上面恢复，执行这条伪指令将PC指向新任务断点处
  //同时mie从mpie恢复到中断前的状态
  mret
.section .text.vectors, "ax"
.option push
.option norelax
.align 9
.option pop
//中断向量表
vectors:
    .word     0
    .word     0
    .word     0
    .word     INT_SFT
    .word     0
    .word     0
    .word     0
    .word     INT_TMR
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
.align 6
.option pop
.global _start_trap
//异常函数入口，这个函数在riscv-rt link.x链接脚本中定义
_start_trap:
    //进入异常只需要保存/恢复16个寄存器，
    //因为riscv指令集架构没有要求硬件实现压/出栈
    //但是CH部分芯片执行了硬件压栈
    addi sp, sp, -16*REGBYTES

    STORE ra, 0*REGBYTES(sp)
    STORE t0, 1*REGBYTES(sp)
    STORE t1, 2*REGBYTES(sp)
    STORE t2, 3*REGBYTES(sp)
    STORE t3, 4*REGBYTES(sp)
    STORE t4, 5*REGBYTES(sp)
    STORE t5, 6*REGBYTES(sp)
    STORE t6, 7*REGBYTES(sp)
    STORE a0, 8*REGBYTES(sp)
    STORE a1, 9*REGBYTES(sp)
    STORE a2, 10*REGBYTES(sp)
    STORE a3, 11*REGBYTES(sp)
    STORE a4, 12*REGBYTES(sp)
    STORE a5, 13*REGBYTES(sp)
    STORE a6, 14*REGBYTES(sp)
    STORE a7, 15*REGBYTES(sp)

    add a0, sp, zero
    jal ra, _start_trap_rust

    LOAD ra, 0*REGBYTES(sp)
    LOAD t0, 1*REGBYTES(sp)
    LOAD t1, 2*REGBYTES(sp)
    LOAD t2, 3*REGBYTES(sp)
    LOAD t3, 4*REGBYTES(sp)
    LOAD t4, 5*REGBYTES(sp)
    LOAD t5, 6*REGBYTES(sp)
    LOAD t6, 7*REGBYTES(sp)
    LOAD a0, 8*REGBYTES(sp)
    LOAD a1, 9*REGBYTES(sp)
    LOAD a2, 10*REGBYTES(sp)
    LOAD a3, 11*REGBYTES(sp)
    LOAD a4, 12*REGBYTES(sp)
    LOAD a5, 13*REGBYTES(sp)
    LOAD a6, 14*REGBYTES(sp)
    LOAD a7, 15*REGBYTES(sp)

    addi sp, sp, 16*REGBYTES
    mret
.section .text
.global _setup_interrupts
//配置向量表、异常、中断入口，这个函数在riscv-rt link.x链接脚本中定义
_setup_interrupts:
    // Set the the NMI base to share with mtvec by setting CSR_MMISC_CTL
    li t0, 0x200
    csrs CSR_MMISC_CTL, t0

    // Set the mtvt
    la t0, vectors
    csrw CSR_MTVT, t0

    // Set the mtvt2 and enable it
    la t0, _irq_handler
    csrw CSR_MTVT2, t0
    csrs CSR_MTVT2, 0x1

    // Enable ECLIC and set trap handler
    la t0, _start_trap
    andi t0, t0, -64
    ori t0, t0, 3
    csrw mtvec, t0

    ret

.weak INT_SFT
.weak INT_TMR
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
