/*
  The code is based on vendor provided HAL libraries.
  Most code come from Firmware\RISCV\env_Eclipse\start.S
*/

#define STORE    sw
#define LOAD     lw
#define LOG_REGBYTES 2
#define REGBYTES (1 << LOG_REGBYTES)

#define CSR_MSTATUS     0x300
#define CSR_MTVT        0x307
#define CSR_MEPC        0x341
#define CSR_MCAUSE      0x342
#define CSR_MTVT2       0x7EC
#define CSR_JALMNXTI    0x7ED
#define CSR_PUSHMCAUSE  0x7EE
#define CSR_PUSHMEPC    0x7EF
#define CSR_PUSHMSUBM   0x7EB
#define CSR_MMISC_CTL   0x7d0
#define CSR_MSUBM       0x7c4

#define MSTATUS_MIE     0x00000008

.macro DISABLE_MIE
  csrc CSR_MSTATUS, MSTATUS_MIE
.endm

.macro SAVE_CONTEXT
  addi sp, sp, -20*REGBYTES

  STORE x1, 0*REGBYTES(sp)
  STORE x4, 1*REGBYTES(sp)
  STORE x5, 2*REGBYTES(sp)
  STORE x6, 3*REGBYTES(sp)
  STORE x7, 4*REGBYTES(sp)
  STORE x10, 5*REGBYTES(sp)
  STORE x11, 6*REGBYTES(sp)
  STORE x12, 7*REGBYTES(sp)
  STORE x13, 8*REGBYTES(sp)
  STORE x14, 9*REGBYTES(sp)
  STORE x15, 10*REGBYTES(sp)
  STORE x16, 11*REGBYTES(sp)
  STORE x17, 12*REGBYTES(sp)
  STORE x28, 13*REGBYTES(sp)
  STORE x29, 14*REGBYTES(sp)
  STORE x30, 15*REGBYTES(sp)
  STORE x31, 16*REGBYTES(sp)
.endm

.macro RESTORE_CONTEXT
  LOAD x1, 0*REGBYTES(sp)
  LOAD x4, 1*REGBYTES(sp)
  LOAD x5, 2*REGBYTES(sp)
  LOAD x6, 3*REGBYTES(sp)
  LOAD x7, 4*REGBYTES(sp)
  LOAD x10, 5*REGBYTES(sp)
  LOAD x11, 6*REGBYTES(sp)
  LOAD x12, 7*REGBYTES(sp)
  LOAD x13, 8*REGBYTES(sp)
  LOAD x14, 9*REGBYTES(sp)
  LOAD x15, 10*REGBYTES(sp)
  LOAD x16, 11*REGBYTES(sp)
  LOAD x17, 12*REGBYTES(sp)
  LOAD x28, 13*REGBYTES(sp)
  LOAD x29, 14*REGBYTES(sp)
  LOAD x30, 15*REGBYTES(sp)
  LOAD x31, 16*REGBYTES(sp)
  // De-allocate the stack space
  addi sp, sp, 20*REGBYTES
.endm

// IRQ entry point
.section .text.irq
.option push
.option norelax
.align 2
.option pop
.global _irq_handler
_irq_handler:
  SAVE_CONTEXT

  // The special CSR read operation, which is actually use mcause as operand to
  // directly store it to memory
  csrrwi  x0, CSR_PUSHMCAUSE, 17
  // The special CSR read operation, which is actually use mepc as operand to
  // directly store it to memory
  csrrwi  x0, CSR_PUSHMEPC, 18
  // The special CSR read operation, which is actually use Msubm as operand to
  // directly store it to memory
  csrrwi  x0, CSR_PUSHMSUBM, 19

  // The special CSR read/write operation, which is actually Claim the CLIC to
  // find its pending highest ID, if the ID is not 0, then automatically enable
  // the mstatus.MIE, and jump to its vector-entry-label, and update the link register.
  // https://www.riscv-mcu.com/column-topic-id-467.html
  csrrw ra, CSR_JALMNXTI, ra

  DISABLE_MIE

  LOAD x5,  19*REGBYTES(sp)
  csrw CSR_MSUBM, x5
  LOAD x5,  18*REGBYTES(sp)
  csrw CSR_MEPC, x5
  LOAD x5,  17*REGBYTES(sp)
  csrw CSR_MCAUSE, x5

  RESTORE_CONTEXT

  mret

.section .text.vectors, "ax"
.option push
.option norelax
.align 9
.option pop

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
_start_trap:
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
