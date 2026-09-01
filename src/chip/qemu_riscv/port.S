/*
  QEMU RISC-V virt 机上下文切换与中断分发——**标准 CLINT,零厂商层**:
  mtvec=Direct 单入口,mcause 7=机器定时中断(MTIMECMP)、3=机器软件
  中断(MSIP,yield)、11=机器外部中断(PLIC:UART RX,ch28 练习 1)。
  36 字帧与 gd32/ch32/esp 各口同一约定。

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
// ★ CSR 装载必须放在通用寄存器之前:CSR 段用 t0 作草稿,若先恢复
//   x5(t0)再装 CSR,最后一次 `lw t0, mcause槽` 会把恢复好的 t0 覆盖成
//   0x80000007——任务若是在编译器 outlined 序言(`jr t0` 结尾)中被
//   抢占的,恢复后 jr t0 → 0x80000006(_start+6)→ 整机"重启"
//   (qemu_kernel_tests 偶发复位 3/10 的根因)
.macro RESTORE_CONTEXT
  lw t0, 32 * 4(sp)
  csrw mstatus, t0
  lw t0, 33 * 4(sp)
  csrw mepc, t0
  // 34 槽跳过(保留)
  lw t0, 35 * 4(sp)
  csrw mcause, t0
  // 通用寄存器最后装载——t0(x5)在所有草稿使用之后恢复
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
  addi sp, sp, 4 * 36
.endm

// 当前任务全局指针(RUST 代码维护)
.extern CURRENT_TASK_PTR
// RUST 侧 tick 处理(mcause=7)
.extern TickHandler
// RUST 侧 yield 处理(mcause=3:清 MSIP 后调度)
.extern YieldHandler
// RUST 侧机器外部中断处理(mcause=11:PLIC——现接 UART RX,ch28)
.extern ExternalIrqHandler
// 未处理异常/中断的默认处理(riscv-rt 提供,弱符号——应用可强定义覆盖)
.extern DefaultHandler

/*
  统一 trap 入口(mtvec = Direct 模式指向这里):
  中断(mcause bit31=1)号 7 → TickHandler(RUST);号 3 → YieldHandler(RUST);
  其余中断/全部异常 → DefaultHandler。
  返回后统一走切换/恢复路径。
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
  // 任务块首字段存 sp——本核槽位:CURRENT_TASK_PTR 是每核指针表基址,
  // 按 mhartid*4 偏移寻址(单核时 mhartid=0,与旧写死符号等价)
  csrr t0, mhartid
  slli t0, t0, 2
  la t1, CURRENT_TASK_PTR
  add t0, t0, t1
  lw t0, 0(t0)
  sw sp, 0x0(t0)
  // 切到中断栈:mscratch 恒持中断栈地址(restore_ctx.S 启动时写一次,
  // 之后永不改)——必须 csrr 只读,csrrw 交换会把任务栈地址写进 mscratch,
  // 下一次中断就跑在陈旧/已释放的任务栈上(任务退出+内存复用后现场错乱)
  csrr sp, mscratch

  // 分发:中断? (mcause < 0 即 bit31=1)
  csrr t0, mcause
  bltz t0, _int_dispatch

  // ---- 异常路径 ----
_exc_dispatch:
  // 所有异常进 DefaultHandler(构建级:无 FPU/无虚存,异常即致命)
  jal ra, DefaultHandler
  j _switch_and_restore

  // ---- 中断路径 ----
_int_dispatch:
  li t1, 0x7fffffff
  and t0, t0, t1        // 取中断号(去 bit31)
  li t1, 7              // 机器定时中断(CLint MTIMECMP)
  beq t0, t1, _isr_tick
  li t1, 3              // 机器软件中断(CLint MSIP)
  beq t0, t1, _isr_yield
  li t1, 11             // 机器外部中断(PLIC:UART RX 等,ch28 练习 1)
  beq t0, t1, _isr_external
  jal ra, DefaultHandler
  j _switch_and_restore

_isr_tick:
  jal ra, TickHandler   // RUST:重装 MTIMECMP + 时间账(scheduler::systick)
  j _switch_and_restore

_isr_external:
  jal ra, ExternalIrqHandler  // RUST:claim/分派/完成(PLIC 握手)
  j _switch_and_restore

_isr_yield:
  jal ra, YieldHandler  // RUST:清 MSIP(电平源)
  // 顺流落入 _switch_and_restore(yield 的公共切换出口)

  // ---- 切换与恢复(公共出口) ----
_switch_and_restore:
  // 调度器在中断栈上运行(mscratch 恒持中断栈——绝不在任务栈上跑:
  // ①小栈任务会被调度器自身栈帧撑爆;②csrrw 交换会污染 mscratch,
  //   修前 bug:任务退出后 mscratch 残留已释放的任务栈地址,下一次
  //   中断在已释放内存上运行 → 内存复用后现场错乱 → guest 复位)
  jal ra, switch_context
  // 装载本核新任务 sp(调度器已更新 CURRENT[本核];mhartid 索引同上)
  csrr t0, mhartid
  slli t0, t0, 2
  la t1, CURRENT_TASK_PTR
  add t0, t0, t1
  lw t0, 0(t0)
  lw sp, 0x0(t0)
  RESTORE_CONTEXT
  mret

/*
  任务首调蹦床:save_context 造的初始帧 mepc 指向这里(而非 task.entry)。
  mret 直入入口函数不可靠:编译器(LTO)会把闭包入口的序言/尾声 outlined
  成共享 stub(如 `sw..; jr t0` / `lw ra,0xc(sp); ret`),这些 stub 只在
  调用方先设置好寄存器(jalr 调用模式)时成立,mret 路径跳过了调用方 →
  stub 读到未初始化的帧槽(陈旧堆数据)→ 野跳(实测跳进 _start → 整机
  "重启")。task.entry 是地址已取的 fn 指针,经标准 jalr 调用必然合法
  ——蹦床恢复"调用方"角色,入口返回后接统一的退出路径。
*/
.section .text
.global _task_entry_trampoline
_task_entry_trampoline:
  csrr t0, mhartid        // CURRENT[本核] == 本任务(restore 前由 execute 设置)
  slli t0, t0, 2
  la t1, CURRENT_TASK_PTR
  add t0, t0, t1
  lw t0, 0(t0)
  lw t1, 8(t0)              // Task.entry(offset 8,见 mod.rs 布局断言)
  jalr t1                   // 标准调用:a0=args(帧设置),ra=返回蹦床
  j task_exit               // 入口跑完 → 退出路径(不返回)

/*
  riscv-rt link.x 在启动时调用:配 mtvec(Direct)与 HPE 关闭。
*/
.section .text
.global _setup_interrupts
_setup_interrupts:
  // virt 机是标准核——无任何厂商 CSR,纯软件保存模型天然成立
  // mtvec = _start_trap,Direct 模式(低位 0)
  la t0, _start_trap
  andi t0, t0, -4       // 4 字节对齐(Direct 模式的最低要求)
  csrw mtvec, t0
  ret
