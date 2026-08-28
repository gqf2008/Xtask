// 调度器启动:进调度循环恢复第一个任务(start_scheduler 调用,不返回)。
// start_idle_task 已把 CURRENT[0] 设为 idle(初始帧由 save_context 构造)。
// 跳 port.S 的 scheduler_loop —— 恢复逻辑与切换/抢占路径同源,帧布局
// 单一实现。关中断后进入;首任务 I 位随帧内 spsr 复位(中断开启)。
// 注意:本文件经 asm!(include_str!, options(raw)) 内联——不得定义符号,
// 裸指令序列接在调用点尾部执行

  cpsid i                        // 调度启动路径全程关中断
  b scheduler_loop               // 跳调度循环(port.S 全局符号,链接期解析)
