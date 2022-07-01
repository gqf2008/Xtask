// 调度器启动时恢复第一个任务到CPU寄存器

//关全局中断
csrc mstatus, 0x00000008 
// 获取link.x链接文件中_stack_start栈顶地址，因为这个函数不会返回，所以
// 当函数结束时还会留下一部分栈空间，这部分空间作为中断服务函数栈使用，以提
// 高内存利用率，同时，留512字节空间给启动函数，因为启动函数可能已经占用了
// 一部分堆栈空间，硬件启动栈空间是2k，剩下的1.5k作为中断栈使用
// 把栈顶地址保存到mscratch备用寄存器，当进入中断服务函数时可以拿出来给中断函数用
la t0, _stack_start
addi t0, t0, -512
csrw mscratch, t0
// 加载当前任务块地址
lw t0, CURRENT_TASK_PTR 
// SP指向任务栈栈顶，任务块第一个变量就是指向任务栈顶
lw sp, 0x0(t0) 
// 从栈顶出栈恢复CPU状态
// 通用寄存器出栈操作
lw x1, 1 * 4(sp) // 任务返回地址
lw x5, 5 * 4(sp)
lw x6, 6 * 4(sp)
lw x7, 7 * 4(sp)
lw x8, 8 * 4(sp)
lw x9, 9 * 4(sp)
lw x10, 10 * 4(sp) // a0，任务参数
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

// CSR寄存器出栈
lw t0, 32 * 4(sp)
csrw mstatus, t0 // 恢复mstatus，当mret时mie=mpie，即打开全局中断
lw t0, 33 * 4(sp)
csrw mepc, t0 // 任务入口函数在这里，也是PC地址
lw t0, 34 * 4(sp)
csrw 0x7c4, t0 // 自定义寄存器
lw t0, 35 * 4(sp)
csrw mcause, t0 // 保存异常代码
// 释放栈空间，栈指针上移，
// 任务块栈顶指针的值还是保持在原来的地方，这里更
// 不更新无所谓，因为任务已经在运行，当被切换掉时会被更新掉
addi sp, sp, 4 * 36 
// 这条伪指令CPU就开始执行任务函数了，使PC=mepc，mie=mpie
mret