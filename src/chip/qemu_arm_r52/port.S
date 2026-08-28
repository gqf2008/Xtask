/*
  QEMU xlnx-zcu102 Cortex-R5F 移植——上下文切换与中断分发。
  镜像布局:向量表(32B)在 OCM @0xFFFF0000(SCTLR.V=1),其余在 DDR。
  【架构:参考 Eclipse ThreadX Cortex-R5 port 的「中断借道 + 调度循环」模型】

  ★ 三条上下文路径,职责分离(修前死穴:IRQ 尾部直接恢复新任务,一旦
    中断打断切换路径本身,帧里 lr=调度器代码地址,存进任务 sp 后任务
    恢复即从 execute/do_schedule 中间"继续"——实测无限空转/UNDEF):
    1. 任务让出(yield/sleep):svc_entry 同步存帧 → do_schedule 选
       下一个任务 → 进调度循环。SVC 是同步异常,I 位硬件置 1,零风险。
    2. 中断抢占:irq_entry 三路判别(ThreadX _tx_thread_system_state
       计数器 + current_ptr 判空):
       a. 嵌套中断:绝不触碰任务帧,清完中断原样返回;
       b. 内核/调度循环被打断(current==NULL):不存任何帧——调度循环
          现场天然可弃;
       c. 任务被打断:仅压 SPSR/中断点 4 字到 IRQ 栈,r0-r12 留在 CPU
          给 handler 用;handler 之后判定抢占——不抢占原子弹回,
          抢占才补全现场存线程栈,再进调度循环。
    3. 调度循环(scheduler_loop):恢复 CURRENT 指向的帧。「恢复谁」
       永远只发生在这里,中断绝不替调度器恢复任务。

  ★ 帧布局(线程栈,16 字 = 64B,低→高):
    [0..12] r0-r12 现场 / [13] spsr / [14] lr(返回地址) / [15] sp_svc
  与 mod.rs save_context 互为镜像。SP_irq 只做 ≤20B 草稿暂存。
*/
.syntax unified

// ============ 向量表 @0xFFFF0000(SCTLR.V=1 → 高向量,OCM) ============
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
  ldr sp, =__stack_irq       // SP_irq = 专用中断栈(草稿区)
  mrs r0, cpsr
  bic r0, r0, #0x1F
  orr r0, r0, #0x13          // 回 SVC 模式
  msr cpsr_c, r0
  bl main                    // 例程的 #[no_mangle] extern "C" fn main
1: b 1b                      // main 若返回则挂起(正常路径不返回)

// ============ SVC 入口(任务让出:yield/sleep) ============
// 进入时 SVC 模式、SP=任务栈,I 位硬件已置 1——同步存帧,零被打断风险
.section .text.svc_entry, "ax"
.global svc_entry
svc_entry:
  stmdb sp!, {r0-r12}        // [0..12] 现场
  mrs r0, spsr
  stmdb sp!, {r0}            // [13] spsr
  stmdb sp!, {lr}            // [14] lr(svc 返回地址)
  add r0, sp, #60            // 入口 SP(已压 15 字)
  stmdb sp!, {r0}            // [15] sp_svc = 任务让出时的栈指针
  ldr r0, =CURRENT_TASK_PTR
  ldr r0, [r0]
  str sp, [r0]               // Task.sp = 帧底
  ldr sp, =__stack_irq       // 调度路径跑 IRQ 栈
  b switch_and_schedule

// ============ IRQ 入口(ThreadX 式三路判别) ============
.section .text.irq_entry, "ax"
.global irq_entry
irq_entry:
  stmdb sp!, {r0-r3, r12}    // scratch 暂存 IRQ 栈(5 字)
  ldr r2, =IRQ_NESTING
  ldr r3, [r2]
  cmp r3, #0
  bne irq_nested             // 嵌套:绝不触碰任务帧
  mov r3, #1
  str r3, [r2]
  ldr r1, =SCHED_STARTED     // ★ 调度器门控:启动阶段(main/start_scheduler
  ldrb r1, [r1]              //   的 Rust 代码)即使 CURRENT 已指向未首跑的
  cmp r1, #0                 //   idle,真实上下文也是启动栈——存帧即污染
  beq irq_in_kernel
  ldr r1, =CURRENT_TASK_PTR
  ldr r1, [r1]
  cmp r1, #0
  beq irq_in_kernel          // 调度循环/内核被打断:不存帧!
  mrs r0, spsr               // 任务被打断:仅存 SPSR/中断点(2 字)
  sub lr, lr, #4
  stmdb sp!, {r0, lr}
  b irq_dispatch
irq_in_kernel:
  bl IrqHandler              // 清中断 + tick 记账,不做任何调度
  ldmia sp!, {r0-r3, r12}
  subs pc, lr, #4            // 原样返回被打断的内核上下文
irq_nested:
  mrs r0, spsr
  sub lr, lr, #4
  stmdb sp!, {r0, lr}
  bl IrqHandler
  ldmia sp!, {r0, lr}
  msr spsr_cxsf, r0
  ldmia sp!, {r0-r3, r12}
  subs pc, lr, #4
irq_dispatch:
  bl IrqHandler              // Rust:读 IAR/清 TTC/EOI/systick(到期任务入队)
  bl irq_preempt_check       // Rust:被打断任务仍最高优先级?r0=1 需抢占
  cmp r0, #0
  beq irq_no_preempt
  // ---- 抢占:补全现场存线程栈(ThreadX 同款,此刻才动任务栈) ----
  ldmia sp!, {r0, r1}        // r0=SPSR_irq(任务的 CPSR),r1=中断点
  mrs r2, cpsr
  bic r2, r2, #0x1F
  orr r2, r2, #0x13
  msr cpsr_c, r2             // 切 SVC → SP = 被打断任务自己的栈
  mov r3, sp                 // r3 = 中断点 SP(任务 sp_svc;r3 现场稍后从 scratch 取回)
  stmdb sp!, {r3}            // [15] sp_svc = 任务栈指针
  stmdb sp!, {r1}            // [14] lr = 中断点
  stmdb sp!, {r0}            // [13] spsr
  mrs r0, cpsr
  bic r0, r0, #0x1F
  orr r0, r0, #0x12
  msr cpsr_c, r0             // 切回 IRQ 取 scratch(SP_irq 仍指暂存区)
  ldmia sp!, {r0-r3, r12}    // 任务现场 r0-r3/r12
  mrs r1, cpsr
  bic r1, r1, #0x1F
  orr r1, r1, #0x13
  msr cpsr_c, r1             // 切回 SVC
  stmdb sp!, {r4-r12}        // [4..12]
  stmdb sp!, {r0-r3}         // [0..3] —— 帧完成,sp=帧底
  ldr r0, =CURRENT_TASK_PTR
  ldr r0, [r0]
  str sp, [r0]               // Task.sp = 帧底
  ldr r0, =0xFF000030
  mov r1, #63
  str r1, [r0]
  b switch_and_schedule
irq_no_preempt:
  ldmia sp!, {r0, lr}
  msr spsr_cxsf, r0
  ldmia sp!, {r0-r3, r12}
  ldr r2, =IRQ_NESTING
  mov r3, #0
  str r3, [r2]               // 计数清零(单层设计,返回即出中断)
  subs pc, lr, #4

// ============ 调度切换 + 调度循环(唯一恢复现场的地方) ============
.section .text.switch_and_schedule, "ax"
.global switch_and_schedule
switch_and_schedule:
  // 中断层到此结束:调度循环恢复的任务在 I=0 下运行,嵌套计数必须归零
  // (irq 抢占路径经此进入;不清则后续中断全走嵌套路径,调度死锁)
  ldr r0, =IRQ_NESTING
  mov r1, #0
  str r1, [r0]
  bl switch_context          // Rust: do_schedule —— execute(new) 换 CURRENT 槽
.global scheduler_loop       // restore_ctx.S(启动首调度)跳入此恢复点
scheduler_loop:
  ldr r0, =CURRENT_TASK_PTR
  ldr r0, [r0]
  ldr sp, [r0]               // 帧底(Task.sp)
  ldmia sp!, {r0-r12}        // [0..12] 现场
  ldmia sp!, {r1}            // [13] spsr
  msr spsr_cxsf, r1
  ldmia sp!, {r1}            // [14] lr(返回地址)
  ldr sp, [sp]               // [15] sp_svc = 任务栈指针
  movs pc, r1                // 进任务(CPSR ← SPSR,I 随帧恢复)

// ============ 任务首调蹦床(save_context 帧的 lr 槽指向这里) ============
.section .text.trampoline, "ax"
.global _task_entry_trampoline
_task_entry_trampoline:
  ldr r12, =CURRENT_TASK_PTR
  ldr r12, [r12]
  ldr r12, [r12, #8]         // Task.entry(offset 8)→ r12(r0 的 args 保留)
  blx r12                    // 标准调用(r0 = args);返回 → task_exit
  b task_exit
