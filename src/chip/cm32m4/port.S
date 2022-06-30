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
  RESTORE_CONTEXT_SOFT_IRQ
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
    .word     0
    .word     0
    /* CM32M4xxR External Interrupts */
    .word		WWDG_IRQHandler					/* 19: Window WatchDog Interrupt */
    .word		PVD_IRQHandler					/* 20: PVD through EXTI Line detection Interrupt */
    .word		TAMPER_IRQHandler				/* 21: Tamper Interrupt */
    .word		RTC_IRQHandler					/* 22: RTC global Interrupt */
    .word		FLASH_IRQHandler				/* 23: FLASH global Interrupt */
    .word		RCC_IRQHandler					/* 24: RCC global Interrupt */
    .word		EXTI0_IRQHandler				/* 25: EXTI Line0 Interrupt */
    .word		EXTI1_IRQHandler				/* 26: EXTI Line1 Interrupt */
    .word		EXTI2_IRQHandler				/* 27: EXTI Line2 Interrupt */
    .word		EXTI3_IRQHandler				/* 28: EXTI Line3 Interrupt */
    .word		EXTI4_IRQHandler				/* 29: EXTI Line4 Interrupt */
    .word		DMA1_Channel1_IRQHandler		/* 30: DMA1 Channel 1 global Interrupt */
    .word		DMA1_Channel2_IRQHandler		/* 31: DMA1 Channel 2 global Interrupt */
    .word		DMA1_Channel3_IRQHandler		/* 32: DMA1 Channel 3 global Interrupt */
    .word		DMA1_Channel4_IRQHandler		/* 33: DMA1 Channel 4 global Interrupt */
    .word		DMA1_Channel5_IRQHandler		/* 34: DMA1 Channel 5 global Interrupt */
    .word		DMA1_Channel6_IRQHandler		/* 35: DMA1 Channel 6 global Interrupt */
    .word		DMA1_Channel7_IRQHandler		/* 36: DMA1 Channel 7 global Interrupt */
    .word		ADC1_2_IRQHandler				/* 37: ADC1 and ADC2 global Interrupt */
    .word		USB_HP_CAN1_TX_IRQHandler		/* 38: USB Device High Priority or CAN1 TX Interrupts */
    .word		USB_LP_CAN1_RX0_IRQHandler		/* 39: USB Device Low Priority or CAN1 RX0 Interrupts */
    .word		CAN1_RX1_IRQHandler				/* 40: CAN1 RX1 Interrupt */
    .word		CAN1_SCE_IRQHandler				/* 41: CAN1 SCE Interrupt */
    .word		EXTI9_5_IRQHandler				/* 42: External Line[9:5] Interrupts */
    .word		TIM1_BRK_IRQHandler				/* 43: TIM1 Break Interrupt */
    .word		TIM1_UP_IRQHandler				/* 44: TIM1 Update Interrupt */
    .word		TIM1_TRG_COM_IRQHandler			/* 45: TIM1 Trigger and Commutation Interrupt */
    .word		TIM1_CC_IRQHandler				/* 46: TIM1 Capture Compare Interrupt */
    .word		TIM2_IRQHandler					/* 47: TIM2 global Interrupt */
    .word		TIM3_IRQHandler					/* 48: TIM3 global Interrupt */
    .word		TIM4_IRQHandler					/* 49: TIM4 global Interrupt */
    .word		I2C1_EV_IRQHandler				/* 50: I2C1 Event Interrupt */
	  .word		I2C1_ER_IRQHandler				/* 51: I2C1 Error Interrupt */
    .word		I2C2_EV_IRQHandler				/* 52: I2C2 Event Interrupt */
	  .word		I2C2_ER_IRQHandler				/* 53: I2C2 Error Interrupt */
    .word		SPI1_IRQHandler					/* 54: SPI1 global Interrupt */
	  .word		SPI2_I2S2_IRQHandler			/* 55: SPI2/I2S2 global Interrupt */
    .word		USART1_IRQHandler				/* 56: USART1 global Interrupt */
	  .word		USART2_IRQHandler				/* 57: USART2 global Interrupt */
    .word		USART3_IRQHandler				/* 58: USART3 global Interrupt */
	  .word		EXTI15_10_IRQHandler			/* 59: External Line[15:10] Interrupts */
    .word		RTCAlarm_IRQHandler				/* 60: RTC Alarm through EXTI Line Interrupt */
	  .word     0			/* 61: Reserved */
	  .word		TIM8_BRK_IRQHandler				/* 62: TIM8 Break Interrupt */
    .word		TIM8_UP_IRQHandler				/* 63: TIM8 Update Interrupt */
	  .word		TIM8_TRG_COM_IRQHandler			/* 64: TIM8 Trigger and Commutation Interrupt */
    .word		TIM8_CC_IRQHandler				/* 65: TIM8 Capture Compare Interrupt */
	  .word		ADC3_4_IRQHandler				/* 66: ADC3 and ADC4 global Interrupt */
	  .word		  0			/* 67: Reserved */
	  .word     0			/* 68: Reserved */
	  .word		TIM5_IRQHandler					/* 69: TIM5 global Interrupt */
    .word		SPI3_I2S3_IRQHandler			/* 70: SPI3/I2S3 global Interrupt */
	  .word		UART4_IRQHandler				/* 71: UART4 global Interrupt */
    .word		UART5_IRQHandler				/* 72: UART5 global Interrupt */
	  .word		TIM6_IRQHandler					/* 73: TIM6 global Interrupt */
    .word		TIM7_IRQHandler					/* 74: TIM7 global Interrupt */
	  .word		DMA2_Channel1_IRQHandler		/* 75: DMA2 Channel 1 global Interrupt */
    .word		DMA2_Channel2_IRQHandler		/* 76: DMA2 Channel 2 global Interrupt */
	  .word		DMA2_Channel3_IRQHandler		/* 77: DMA2 Channel 3 global Interrupt */
    .word		DMA2_Channel4_IRQHandler		/* 78: DMA2 Channel 4 global Interrupt */
	  .word		DMA2_Channel5_IRQHandler		/* 79: DMA2 Channel 5 global Interrupt */
	  .word     0			/* 80: Reserved */
	  .word     0			/* 81: Reserved */
    .word		CAN2_TX_IRQHandler				/* 82: CAN2 TX Interrupt */
	  .word		CAN2_RX0_IRQHandler				/* 83: CAN2 RX0 Interrupt */
    .word		CAN2_RX1_IRQHandler				/* 84: CAN2 RX1 Interrupt */
	  .word		CAN2_SCE_IRQHandler				/* 85: CAN2 SCE Interrupt */
    .word		QSPI_IRQHandler					/* 86: QSPI global Interrupt */
	  .word		DMA2_Channel6_IRQHandler		/* 87: DMA2 Channel 6 global Interrupt */
    .word		DMA2_Channel7_IRQHandler		/* 88: DMA2 Channel 7 global Interrupt */
	  .word		I2C3_EV_IRQHandler				/* 89: I2C3 Event Interrupt */
    .word		I2C3_ER_IRQHandler				/* 90: I2C3 Error Interrupt */
	  .word		I2C4_EV_IRQHandler				/* 91: I2C4 Event Interrupt */
    .word		I2C4_ER_IRQHandler				/* 92: I2C4 Error Interrupt */
	  .word		UART6_IRQHandler				/* 93: UART6 global Interrupt */
    .word		UART7_IRQHandler				/* 94: UART7 global Interrupt */
	  .word		DMA1_Channel8_IRQHandler		/* 95: DMA1 Channel 8 global Interrupt */
    .word		DMA2_Channel8_IRQHandler		/* 96: DMA2 Channel 8 global Interrupt */
	  .word     0			/* 97: Reserved */
	  .word		SAC_IRQHandler					/* 98: SAC global Interrupt */
	  .word   MMU_IRQHandler					/* 99: MMU global Interrupt */
    .word		TSC_IRQHandler					/* 100: TSC global Interrupt */
	  .word		COMP_1_2_3_IRQHandler			/* 101: COMP1 & COMP2 & COMP3 global Interrupt */
    .word		COMP_4_5_6_IRQHandler			/* 102: COMP4 & COMP5 & COMP6 global Interrupt */
	  .word		COMP7_IRQHandler				/* 103: COMP7 global Interrupt */
	  .word		R_SRAM_IRQHandler				/* 104: R-SRAM Error Interrupt */

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
    //1<<9=1 0000 0000
    li t0, 0x100
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

// CM32外设预定义中断符号，与中断向量表里的一致，应用层实现后由连接器绑定
// .weak 表示使符号对链接器全局可见，该符号具有较弱的绑定语义
  .weak  WWDG_IRQHandler
  .weak  PVD_IRQHandler
  .weak  TAMPER_IRQHandler
  .weak  RTC_IRQHandler
  .weak  FLASH_IRQHandler
  .weak  RCC_IRQHandler
  .weak  EXTI0_IRQHandler
  .weak  EXTI1_IRQHandler
  .weak  EXTI2_IRQHandler
  .weak  EXTI3_IRQHandler
  .weak  EXTI4_IRQHandler
  .weak  DMA1_Channel1_IRQHandler
  .weak  DMA1_Channel2_IRQHandler
  .weak  DMA1_Channel3_IRQHandler
  .weak  DMA1_Channel4_IRQHandler
  .weak  DMA1_Channel5_IRQHandler
  .weak  DMA1_Channel6_IRQHandler
  .weak  DMA1_Channel7_IRQHandler
  .weak  ADC1_2_IRQHandler
  .weak  USB_HP_CAN1_TX_IRQHandler
  .weak  USB_LP_CAN1_RX0_IRQHandler
  .weak  CAN1_RX1_IRQHandler
  .weak  CAN1_SCE_IRQHandler
  .weak  EXTI9_5_IRQHandler
  .weak  TIM1_BRK_IRQHandler
  .weak  TIM1_UP_IRQHandler
  .weak  TIM1_TRG_COM_IRQHandler
  .weak  TIM1_CC_IRQHandler
  .weak  TIM2_IRQHandler
  .weak  TIM3_IRQHandler
  .weak  TIM4_IRQHandler
  .weak  I2C1_EV_IRQHandler
	.weak  I2C1_ER_IRQHandler
  .weak  I2C2_EV_IRQHandler
	.weak  I2C2_ER_IRQHandler
  .weak  SPI1_IRQHandler
	.weak  SPI2_I2S2_IRQHandler
  .weak  USART1_IRQHandler
	.weak  USART2_IRQHandler
  .weak  USART3_IRQHandler
	.weak  EXTI15_10_IRQHandler
  .weak  RTCAlarm_IRQHandler
	.weak  TIM8_BRK_IRQHandler
  .weak  TIM8_UP_IRQHandler
	.weak  TIM8_TRG_COM_IRQHandler
  .weak  TIM8_CC_IRQHandler
	.weak  ADC3_4_IRQHandler
	.weak  TIM5_IRQHandler
  .weak  SPI3_I2S3_IRQHandler
	.weak  UART4_IRQHandler
  .weak  UART5_IRQHandler
	.weak  TIM6_IRQHandler
  .weak  TIM7_IRQHandler
	.weak  DMA2_Channel1_IRQHandler
  .weak  DMA2_Channel2_IRQHandler
	.weak  DMA2_Channel3_IRQHandler
  .weak  DMA2_Channel4_IRQHandler
	.weak  DMA2_Channel5_IRQHandler
  .weak  CAN2_TX_IRQHandler
	.weak  CAN2_RX0_IRQHandler
  .weak  CAN2_RX1_IRQHandler
	.weak  CAN2_SCE_IRQHandler
  .weak  QSPI_IRQHandler
	.weak  DMA2_Channel6_IRQHandler
  .weak  DMA2_Channel7_IRQHandler
	.weak  I2C3_EV_IRQHandler
  .weak  I2C3_ER_IRQHandler
	.weak  I2C4_EV_IRQHandler
  .weak  I2C4_ER_IRQHandler
	.weak  UART6_IRQHandler
  .weak  UART7_IRQHandler
	.weak  DMA1_Channel8_IRQHandler
  .weak  DMA2_Channel8_IRQHandler
	.weak  SAC_IRQHandler
	.weak  MMU_IRQHandler
  .weak  TSC_IRQHandler
	.weak  COMP_1_2_3_IRQHandler
  .weak  COMP_4_5_6_IRQHandler
	.weak  COMP7_IRQHandler
	.weak  R_SRAM_IRQHandler
