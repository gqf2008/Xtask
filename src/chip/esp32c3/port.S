/*
  ESP32-C3(RV32IMC)上下文切换与中断分发——CH32 口同款:
  mtvec=Direct 单入口分发(mcause=CPU 中断号),纯软件 36 字帧。
  tick=CPU int 6(SYSTIMER target0 周期模式),yield=CPU int 7
  (SYSTEM cpu_intr_from_cpu_0 电平触发)。

  帧布局(36 字,与 mod.rs 的 save_context 互为镜像):
    [0]  x0 保留      [1]  ra          [2..4] x2-x4 保留
    [5..31] x5-x31    [32] mstatus     [33] mepc
    [34] 保留(Bumblebee msubm 槽)  [35] mcause
*/

// 保存上文宏
.macro SAVE_CONTEXT
  addi sp, sp, -36 * 4
  sw x1, 1 * 4(sp)     // ra
  sw x5, 5 * 4(sp)     // t0
  sw x6, 6 * 4(sp)     // t1
  sw x7, 7 * 4(sp)     // t2
  sw x8, 8 * 4(sp)     // s0/fp
  sw x9, 9 * 4(sp)     // s1
  sw x10, 10 * 4(sp)   // a0
  sw x11, 11 * 4(sp)   // a1
  sw x12, 12 * 4(sp)   // a2
  sw x13, 13 * 4(sp)   // a3
  sw x14, 14 * 4(sp)   // a4
  sw x15, 15 * 4(sp)   // a5
  sw x16, 16 * 4(sp)   // a6
  sw x17, 17 * 4(sp)   // a7
  sw x18, 18 * 4(sp)   // s2
  sw x19, 19 * 4(sp)   // s3
  sw x20, 20 * 4(sp)   // s4
  sw x21, 21 * 4(sp)   // s5
  sw x22, 22 * 4(sp)   // s6
  sw x23, 23 * 4(sp)   // s7
  sw x24, 24 * 4(sp)   // s8
  sw x25, 25 * 4(sp)   // s9
  sw x26, 26 * 4(sp)   // s10
  sw x27, 27 * 4(sp)   // s11
  sw x28, 28 * 4(sp)   // t3
  sw x29, 29 * 4(sp)   // t4
  sw x30, 30 * 4(sp)   // t5
  sw x31, 31 * 4(sp)   // t6
  csrr t0, mstatus
  sw t0, 32 * 4(sp)
  csrr t0, mepc        // 被打断处的下一条指令
  sw t0, 33 * 4(sp)
  sw zero, 34 * 4(sp)  // 保留槽(无 msubm)
  csrr t0, mcause
  sw t0, 35 * 4(sp)
.endm

// 恢复下文宏(与保存严格对称)
.macro RESTORE_CONTEXT
  lw x1, 1 * 4(sp)
  lw x5, 5 * 4(sp)
  lw x6, 6 * 4(sp)
  lw x7, 7 * 4(sp)
  lw x8, 8 * 4(sp)
  lw x9, 9 * 4(sp)
  lw x10, 10 * 4(sp)
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
  lw t0, 32 * 4(sp)
  csrw mstatus, t0
  lw t0, 33 * 4(sp)
  csrw mepc, t0
  // 34 槽跳过(保留)
  lw t0, 35 * 4(sp)
  csrw mcause, t0
  addi sp, sp, 4 * 36
.endm

// 当前任务全局指针(RUST 代码维护)
.extern CURRENT_TASK_PTR
// RUST 侧 tick 处理(CPU int 6)
.extern TickHandler
// RUST 侧 yield 处理(CPU int 7:清 from_cpu_0 后调度)
.extern YieldHandler
// 未处理异常/中断的默认处理(riscv-rt 提供,弱符号——应用可强定义覆盖)
.extern DefaultHandler

/*
  统一 trap 入口(mtvec = Direct 模式指向这里):
  中断(mcause bit31=1)号 12 → SysTick(RUST);
  其余中断/全部异常 → DefaultHandler。
  SysTick 返回后(或 DefaultHandler 返回后)统一走切换/恢复路径。
*/
.section .trap, "ax"
.option push
.option norelax
.align 6
.option pop
.global _start_trap
_start_trap:
  // 保存上文到当前任务栈
  SAVE_CONTEXT
  // 任务块首字段存 sp
  lw t0, CURRENT_TASK_PTR
  sw sp, 0x0(t0)
  // 切到中断栈(mscratch 在 restore_ctx.S 初始化)
  csrrw sp, mscratch, sp

  // 分发:中断? (mcause < 0 即 bit31=1)
  csrr t0, mcause
  bltz t0, _int_dispatch

  // ---- 异常路径 ----
_exc_dispatch:
  // 所有异常进 DefaultHandler(构建级:无 FPU/无虚存,异常即致命)
  jal ra, DefaultHandler
  j _switch_and_restore

  // ---- 中断路径(mcause code = CPU 中断号,C3 中断矩阵 1..31)----
_int_dispatch:
  li t1, 0x7fffffff
  and t0, t0, t1        // 取中断号(去 bit31)
  li t1, 6              // tick(SYSTIMER target0)
  beq t0, t1, _isr_tick
  li t1, 7              // yield(from_cpu_0)
  beq t0, t1, _isr_yield
  // 未接入的外设中断 → DefaultHandler(BSP 扩展点:UART 等走矩阵映射)
  jal ra, DefaultHandler
  j _switch_and_restore

_isr_tick:
  jal ra, TickHandler   // RUST:W1C 清 int_raw + 时间账(scheduler::systick)
  j _switch_and_restore

_isr_yield:
  jal ra, YieldHandler  // RUST:清 from_cpu_0(电平源)+ scheduler::schedule

  // ---- 切换与恢复(公共出口) ----
_switch_and_restore:
  // 回任务栈前先切回旧 sp 由调度器换新任务:
  // 调用 switch_context(等价 gd32 的 INT_SFT 主体)
  csrrw sp, mscratch, sp  // 先换回 mscratch 语义:此时 sp=中断栈
  jal ra, switch_context
  // 中断栈换出、新任务 sp 装入
  csrrw sp, mscratch, sp
  lw t0, CURRENT_TASK_PTR
  lw sp, 0x0(t0)
  RESTORE_CONTEXT
  mret

/*
  riscv-rt link.x 在启动时调用:配 mtvec(Direct)与 HPE 关闭。
*/
.section .text
.global _setup_interrupts
_setup_interrupts:
  // ESP 核无 HPE——纯软件保存模型天然成立,无需任何自定义 CSR
  // mtvec = _start_trap,Direct 模式(低位 0)
  la t0, _start_trap
  andi t0, t0, -4       // 4 字节对齐(Direct 模式的最低要求)
  csrw mtvec, t0
  ret
