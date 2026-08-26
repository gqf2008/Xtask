// 调度器启动:恢复本核第一个任务(gd32 restore_ctx.S 的无 msubm 变体)
// 主核由 start_scheduler 调用;从核由 secondary_main 在放行后调用——
// 同一份代码,mhartid 寻址天然每核正确(ch25 改造路线②)

// 关全局中断
csrc mstatus, 0x00000008
// 每核中断栈:mscratch = _stack_start - MAX_HARTS×启动栈 - 4K×(hartid+1)。
// 栈顶下每核各留 4K(trap 后 TickHandler→调度器→take_expired 的 Vec/迭代器
// 全在此栈上跑,512B 会溢出——gd32 的 512 依据是 2K 启动栈只剩 1.5K,不适用)。
// 【布局坑,>2 核才显形】中断栈区必须先整体跳过**全部**启动栈
// (MAX_HARTS×_hart_stack_size):hart h 的启动栈在 [-2K×(h+1), -2K×h],
// 若中断栈直接从 -4K 起排,hart≥2 的启动栈恰好落在 hart0 中断栈的
// 向下生长区——主核 trap 帧踩烂从核 secondary_main 的栈帧。
// (2 核时 hart1 启动栈 [-4K,-2K] 恰好错开,潜伏未发。)
// 存入 mscratch,trap 入口经 csrr 只读换栈(mscratch 恒持本核中断栈,永不改写)
la t0, _stack_start
la t1, _hart_stack_size
slli t1, t1, 4          // ×16 = MAX_HARTS 个启动栈(整个启动栈区)
sub t0, t0, t1
csrr t1, mhartid
addi t1, t1, 1
slli t1, t1, 12         // ×4K 本核中断栈
sub t0, t0, t1
csrw mscratch, t0
// 装载本核当前任务:CURRENT_TASK_PTR 是每核指针表的基址,按 mhartid 索引
csrr t1, mhartid
slli t1, t1, 2
la t0, CURRENT_TASK_PTR
add t0, t0, t1
lw t0, 0(t0)
lw sp, 0x0(t0)
// CSR 出栈先于通用寄存器(34 槽保留,跳过)——CSR 段用 t0 作草稿,
// 若先恢复 x5 再装 CSR,t0 会被 mcause(0x80000007)覆盖;任务入口在
// 编译器 outlined 序言(`jr t0` 结尾)中时会野跳(port.S RESTORE_CONTEXT 同注)
lw t0, 32 * 4(sp)
csrw mstatus, t0
lw t0, 33 * 4(sp)
csrw mepc, t0
lw t0, 35 * 4(sp)
csrw mcause, t0
// 恢复通用寄存器(t0 最后,在所有草稿使用之后)
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
// 释放帧,跳进任务(mret:PC=mepc,mie=mpie)
addi sp, sp, 4 * 36
mret
