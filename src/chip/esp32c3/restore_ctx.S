// 调度器启动:恢复第一个任务(CH32/ESP 同款无 msubm 变体)

// 关全局中断
csrc mstatus, 0x00000008
// 栈顶下留 512 字节作中断栈(启动函数可能已占一部分;硬件启动栈 2k)
// 存入 mscratch,trap 入口经 csrr 只读换栈(mscratch 恒持中断栈,永不改写)
la t0, _stack_start
addi t0, t0, -512
csrw mscratch, t0
// 装载第一个任务
lw t0, CURRENT_TASK_PTR
lw sp, 0x0(t0)
// 恢复通用寄存器
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
// CSR 出栈(34 槽保留,跳过)
lw t0, 32 * 4(sp)
csrw mstatus, t0
lw t0, 33 * 4(sp)
csrw mepc, t0
lw t0, 35 * 4(sp)
csrw mcause, t0
// 释放帧,跳进任务(mret:PC=mepc,mie=mpie)
addi sp, sp, 4 * 36
mret
