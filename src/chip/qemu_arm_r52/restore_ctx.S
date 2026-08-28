// 调度器启动:恢复第一个任务(start_scheduler 调用,不返回)。
// start_idle_task 已把 CURRENT[0] 设为 idle 任务(初始帧由 save_context
// 构造)——恢复路径与 port.S 的 RESTORE_CTX 完全一致,帧布局同源。
// 关中断后直接从 CURRENT_TASK_PTR 取帧,弹回即进入 idle 任务(ARM 态,
// I 位随帧内 spsr 复位——首任务运行在中断开启状态)。
// 注意:本文件经 asm!(include_str!, options(raw)) 内联——不得定义符号
// (LLVM inline asm 禁命名 label),裸指令序列接在调用点尾部执行

  cpsid i                        // 调度启动路径全程关中断
  ldr r0, =CURRENT_TASK_PTR      // 本核当前任务(单核:槽 0)
  ldr r0, [r0]
  ldr sp, [r0]                   // 帧底(Task.sp,= r0 槽)
  ldmia sp!, {r0-r12}            // [0..12] 现场 r0-r12
  ldmia sp!, {r1}                // [13] spsr → 草稿 r1
  msr spsr_cxsf, r1              // SPSR 写入(r1 释放)
  ldmia sp!, {r1}                // [14] lr → 蹦床地址(最后加载)
  ldr sp, [sp]                   // [15] → SP = 任务栈指针
  movs pc, r1                    // 进任务(I 位随 spsr 复位)
