/*
  QEMU xlnx-zcu102 Cortex-R5F 移植——上下文切换与中断分发。
  镜像布局:向量表(32B)在 OCM @0xFFFF0000(复位向量,SCTLR.V=1),
  其余全在 DDR @0x00000000(-device loader 按 ELF 段加载)。

  ★ 帧布局(16 字 = 64B,与 mod.rs save_context 互为镜像,低→高):
    [0..12] r0-r12 现场
    [13]    spsr(被打断任务的 CPSR)
    [14]    lr(被打断任务的返回地址)
    [15]    sp_svc(被打断任务的栈指针)
  帧恒建在被打断/让出任务自己的栈上:IRQ 入口切 SVC 后压任务栈
  (IRQ 栈仅作 16B 寄存器草稿区),SVC 入口直接压任务栈——切换只认
  帧指针(CURRENT_TASK_PTR->sp),帧永远跟任务走、互不覆盖。

  ★ 嵌套策略:入口即全程关中断(IRQ 硬件自动关 I;SVC 入口显式
  cpsid i),handler/调度器在 I=0 下运行——零重入、单一 ISR 层。
  被延迟的中断在 GIC 侧保持 pending(电平源),恢复任务现场时
  (movs pc 载入帧内 spsr)才重新开 I,延迟响应但不丢。
*/
.syntax unified

// 保存现场宏:压到当前 SP 所在栈。
// 帧布局(16 字 = 64B,低→高):
//   [0..12]  r0-r12 现场(13 字)
//   [13]     spsr
//   [14]     lr(返回地址)
//   [15]     sp_svc(被打断任务的栈指针)
// 【ARM 陷阱】LDM/STM 列表含 sp 时禁止 writeback,sp_svc 槽单独压入。
// 参数:1=IRQ 入口(任务 SP 在 banked SP_svc);0=SVC 入口(入口 SP 即任务 SP)
.macro SAVE_CTX from_irq
  stmdb sp!, {r0-r12}        // [0..12] 现场 13 字:r0 最低 ... r12 最高
  mrs r12, spsr              // 草稿 r12(现场已入帧,可覆盖)
  stmdb sp!, {r12}           // [13] spsr
  stmdb sp!, {lr}            // [14] lr
  .if \from_irq
    mrs r12, sp_svc          // ★ IRQ 入口:被打断任务的 SP 在 banked SP_svc
  .else
    add r12, sp, #60         // SVC 入口:已压 15 字,入口 SP = sp+60 = 任务 SP
  .endif
  stmdb sp!, {r12}           // [15] sp_svc(任务栈指针)
.endm

// 恢复现场宏:从当前 SP(帧底 = r0 槽)弹回。
// 【修前致命 bug】先弹控制槽到草稿 r0/r1/r2 再 `ldmia {r0-r12}` 弹现场——
// 现场把三个草稿全部覆盖,`movs pc, r1` 跳到垃圾(实测 PC=0,NOP 刷 1M
// 回 text——任务永不执行、调度空转)。新序:现场先弹,控制槽后弹;
// lr 兼作唯一草稿(现场不含 lr),sp_svc 槽用 `ldr sp,[sp]` 直弹
.macro RESTORE_CTX
  ldmia sp!, {r0-r12}        // [0..12] → r0-r12 现场
  ldmia sp!, {lr}            // [13] → lr = spsr
  msr spsr_cxsf, lr          // SPSR 写入(lr 释放)
  ldmia sp!, {lr}            // [14] → lr = 返回地址(最后加载,不再被覆盖)
  ldr sp, [sp]               // [15] → SP = 任务栈指针
  movs pc, lr                // 跳回(CPSR ← SPSR)
.endm

// ============ 向量表 @0xFFFF0000(SCTLR.V=1 → 高向量,OCM) ============
// OCM↔DDR 距离超 b 的 ±32M 域:所有向量一律绝对跳(ldr pc, =)
.section .vectors, "ax"
.arm
.balign 32
  ldr pc, =_start            // 0x00 Reset
  b .                        // 0x04 Undefined
  ldr pc, =svc_entry         // 0x08 SVC(yield)
  b .                        // 0x0C Prefetch Abort
  b .                        // 0x10 Data Abort
  b .                        // 0x14 Reserved
  ldr pc, =irq_entry         // 0x18 IRQ
  b .                        // 0x1C FIQ

// ============ 启动(Rust main 由例程提供) ============
.section .text._start, "ax"
.global _start
_start:
  ldr sp, =__stack_top       // SP_svc = DDR 顶(8 对齐,link.x)
  mrs r0, cpsr
  bic r0, r0, #0x1F
  orr r0, r0, #0x12          // IRQ 模式
  msr cpsr_c, r0
  ldr sp, =__stack_irq       // SP_irq = 专用中断栈(IRQ 入口压帧 + handler 栈)
  mrs r0, cpsr
  bic r0, r0, #0x1F
  orr r0, r0, #0x13          // 回 SVC 模式
  msr cpsr_c, r0
  bl main                    // 例程的 #[no_mangle] extern "C" fn main
1: b 1b                      // main 若返回则挂起(正常路径不返回)

// ============ IRQ 入口(IRQ 模式进入,SP_irq) ============
.section .text.irq_entry, "ax"
.global irq_entry
irq_entry:
  // 中断帧压【被打断任务自己的栈】(切 SVC 后 SP 即任务栈)——与 SVC
  // 入口同策略,帧随任务走。【修前坑】帧压专用 IRQ 栈且入口复位 SP_irq
  // 到栈顶:任务 A 被打断(A.sp=栈顶帧)→切 B→B 又被打断(同位置覆盖
  // A 的现场)→A 恢复时弹到 B 的寄存器——调度错乱。IRQ 栈仅作 16B
  // 草稿区暂存 r0-r12(草稿寄存器不够存 SPSR_irq/LR_irq + 现场)
  ldr sp, =__stack_irq       // 复位 IRQ 栈(草稿区;暂存随后全部取走,复位安全)
  stmdb sp!, {r0-r12}        // 13 字现场暂存(含草稿要覆盖的 r0/r1/r2/r12)
  mov r12, sp                // r12 = 暂存区指针
  mrs r0, spsr               // SPSR_irq → r0(必须在 IRQ 模式读)
  mov r1, lr                 // LR_irq(返回地址)→ r1
  mrs r2, cpsr
  bic r2, r2, #0x1F
  orr r2, r2, #0x13          // 切 SVC 模式(硬件已关 I,切模式保持)
  msr cpsr_c, r2
  mov r2, sp                 // ★ SVC 模式下 SP 即被打断任务的栈指针
  stmdb sp!, {r2}            // [15] sp_svc = 任务栈入口 SP
  stmdb sp!, {r1}            // [14] lr(返回地址)
  stmdb sp!, {r0}            // [13] spsr
  ldmia r12, {r0-r3}         // 取回现场 r0-r3(暂存+0)
  add r12, r12, #16
  ldmia r12, {r4-r12}        // 取回现场 r4-r12(暂存+16;r12 复位为现场值)
  stmdb sp!, {r4-r12}        // [4..12]
  stmdb sp!, {r0-r3}         // [0..3]
  ldr r0, =CURRENT_TASK_PTR  // 存帧指针到任务块(帧在任务栈上)
  ldr r0, [r0]
  str sp, [r0]
  ldr sp, =__stack_irq       // handler/调度器跑 IRQ 栈(帧已安全在任务栈)
  b switch_and_restore

// ============ SVC 入口(yield:任务执行 svc #0x5) ============
// 进入时已在 SVC 模式,SP = 任务栈——帧直接压任务栈,与 IRQ 帧同布局
.section .text.svc_entry, "ax"
.global svc_entry
svc_entry:
  cpsid i                    // 关 IRQ(调度路径全程关中断,嵌套=0)
  SAVE_CTX 0                 // 帧压任务栈(入口 SP 即任务 SP)
  ldr r0, =CURRENT_TASK_PTR  // 存帧指针到任务块(帧在任务栈上)
  ldr r0, [r0]
  str sp, [r0]
  ldr sp, =__stack_irq       // 切中断栈(handler/调度器跑这,不撑任务栈)
  b switch_and_restore

// ============ 公共切换路径(两个入口合流) ============
.section .text.switch_and_restore, "ax"
switch_and_restore:
  bl IrqHandler              // Rust:读 IAR/清 TTC/EOI/systick(yield 时 IAR=1023 直接返回)
  bl switch_context          // Rust:scheduler::schedule()(换 CURRENT 槽)
  ldr r0, =CURRENT_TASK_PTR  // 装载新任务帧底(RESTORE 起点)
  ldr r0, [r0]
  ldr sp, [r0]
  RESTORE_CTX

// ============ 任务首调蹦床(save_context 造的帧 lr 槽指向这里) ============
// 帧已把 r0 = args 弹好;标准 blx 调用入口,返回后接统一退出路径。
// 不用 mret/直跳:入口函数是编译器(LTO)可能 outlined 的闭包序言,
// 只在标准调用约定下成立——蹦床扮演"调用方"角色(与 RISC-V 口同因)
.section .text.trampoline, "ax"
.global _task_entry_trampoline
_task_entry_trampoline:
  // DEBUG 探针:写 UART 一个 'T',验证 RESTORE 后 PC 真的到了蹦床
  ldr r12, =0xFF000030       // UART0 TXFIFO
  mov r11, #84               // 'T'(首跑帧 r11=0,破坏无妨)
  str r11, [r12]
  ldr r12, =CURRENT_TASK_PTR
  ldr r12, [r12]
  ldr r12, [r12, #8]          // Task.entry(offset 8)→ r12(r0 的 args 保留!)
  blx r12                     // 标准调用(ARM 态,r0 = args);返回 → 下一行
  b task_exit                // 入口跑完 → 退出路径(Rust,不返回)
