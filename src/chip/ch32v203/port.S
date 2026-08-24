/*
  CH32V307(QingKe V4F)上下文切换与中断分发——gd32vf103 port.S 的
  "纯软件全保存"变体:HPE 显式关闭(0x804=0),mtvec=Direct 单入口分发。

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
  // ★ CSR 装载必须先于通用寄存器:CSR 段用 t0 作草稿,若先恢复 x5(t0)
  //   再装 CSR,最后一次 lw t0(=mcause 槽)会把恢复好的 t0 覆盖——任务在
  //   编译器 outlined 序言(`jr t0` 结尾)中被抢占时,恢复后 jr t0 野跳
  //   (qemu_riscv 口实测:跳 _start+6 → 整机重启,详见该口 port.S 注)
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
// RUST 侧 SysTick 处理(真 tick 与 SWIE 软中断二合一)
.extern SysTick
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
  slli t0, t0, 0        // 保位
  li t1, 0x7fffffff
  and t0, t0, t1        // 取中断号(去 bit31)
  li t1, 12             // SysTick
  beq t0, t1, _isr_systick
  // 未接入的外设中断/其它核心中断 → DefaultHandler(BSP 扩展点)
  jal ra, DefaultHandler
  j _switch_and_restore

_isr_systick:
  jal ra, SysTick       // RUST:清 CNTIF/时间账/必要时置 SWIE 请求切换

  // ---- 切换与恢复(公共出口) ----
_switch_and_restore:
  // 调度器在中断栈上运行(mscratch 恒持中断栈——绝不在任务栈上跑:
  // ①小栈任务会被调度器自身栈帧撑爆;②csrrw 交换会污染 mscratch,
  //   修前 bug:任务退出后 mscratch 残留已释放的任务栈地址,下一次
  //   中断在已释放内存上运行 → 内存复用后现场错乱 → guest 复位)
  jal ra, switch_context
  // 装载新任务 sp(调度器已更新 CURRENT_TASK)
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
  // 显式关闭 HPE(QingKe 自定义 CSR 0x804:intsyscr)——
  // 硬件序言是 WCH 私有工具链特性;写 0 = 纯软件保存模型
  csrw 0x804, zero
  // mtvec = _start_trap,Direct 模式(低位 0)
  la t0, _start_trap
  andi t0, t0, -4       // 4 字节对齐(Direct 模式的最低要求)
  csrw mtvec, t0
  ret

/*
  任务首调蹦床:save_context 造的初始帧 mepc 指向这里(而非 task.entry)。
  mret 直入入口函数不可靠:编译器(LTO)会把闭包入口的序言/尾声 outlined
  成共享 stub(如 `sw..; jr t0` / `lw ra,0xc(sp); ret`),这些 stub 只在
  调用方先设置好寄存器(jalr 调用模式)时成立,mret 路径跳过了调用方 →
  stub 读到未初始化的帧槽(陈旧堆数据)→ 野跳(qemu_riscv 口实测曾跳进
  _start → 整机"重启")。task.entry 是地址已取的 fn 指针,经标准 jalr
  调用必然合法——蹦床恢复"调用方"角色,入口返回后接统一的退出路径。
*/
.section .text
.global _task_entry_trampoline
_task_entry_trampoline:
  lw t0, CURRENT_TASK_PTR   // CURRENT == 本任务(restore 前由 execute 设置)
  lw t1, 8(t0)              // Task.entry(offset 8,见 mod.rs 布局断言)
  jalr t1                   // 标准调用:a0=args(帧设置),ra=返回蹦床
  j task_exit               // 入口跑完 → 退出路径(不返回)
