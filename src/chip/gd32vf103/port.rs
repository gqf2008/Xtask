//! 标准寄存器说明
//! https://suda-morris.github.io/blog/cs/risc-v.html#%E9%80%9A%E7%94%A8%E5%AF%84%E5%AD%98%E5%99%A8%E7%BB%84
//! 下面这个链接很好的解释了PC寄存器和x0/zero寄存器的用处
//! https://balancetwk.github.io/2020/12/01/hexo-blog/RISC_V_Note/RISC-V%20%E7%9A%84%20PC%20%E5%92%8C%20X0/
//! 芯莱科技riscv ip自定义寄存器msubm=0x7c4，自定义寄存器用于保存Core当前的Trap类型，以及进入Trap前的Trap类型。
//! 详情：https://www.riscv-mcu.com/quickstart-doc-u-nuclei_n_isa.html
//! 从零开始写RISC-V处理器
//! https://liangkangnan.gitee.io/2020/04/29/%E4%BB%8E%E9%9B%B6%E5%BC%80%E5%A7%8B%E5%86%99RISC-V%E5%A4%84%E7%90%86%E5%99%A8/
//! 基本指令
//! https://blog.csdn.net/qq_39507748/article/details/120150936
//!
//! CSR寄存器操作指令说明
//! csrr，读取一个 CSR 的值到通用寄存器。如：csrr t0, mstatus，读取 mstatus 的值到 t0 中。
//! csrw，把一个通用寄存器中的值写入 CSR 中。如：csrw mstatus, t0，将 t0 的值写入 mstatus。
//! csrs，把 CSR 中指定的 bit 置 1。如：csrsi mstatus, (1 << 2)，将 mstatus 的右起第 3 位置 1。
//! csrc，把 CSR 中指定的 bit 置 0。如：csrci mstatus, (1 << 2)，将 mstatus 的右起第 3 位置 0。
//! csrrw，读取一个 CSR 的值到通用寄存器，然后把另一个值写入该 CSR。如：csrrw t0, mstatus, t0，将 mstatus 的值与 t0 的值交换。
//! csrrs，读取一个 CSR 的值到通用寄存器，然后把该 CSR 中指定的 bit 置 1。
//! csrrc，读取一个 CSR 的值到通用寄存器，然后把该 CSR 中指定的 bit 置 0。

use super::*;
use crate::task::scheduler;
use core::arch::global_asm;

// 导入汇编代码
global_asm!(include_str!("port.S"));

/**导出中断服务函数，导出名称必须与port.S汇编代码中定义的一致**/

/// riscv规定，进入中断函数前，全局中断被硬件自动关闭，mpie=mie，mie=0
/// 从中断函数退出后，mie被mpie恢复，恢复到中断前的中断状态
/// 注意，退出中断服务不是指退出当前这个函数，而是在汇编代码实现的_irq_handler函数

/// 定时中断服务函数，驱动任务调度，当有任务需求切换时触发软中断即可，
/// 任务切换由软中断服务函数实现，gd32里使用自定义寄存器（0x7ED）巧
/// 妙的实现了中断嵌套，工作职责清晰。
/// 当进入中断函数时SP已经在port.S汇编代码中切换到了中断栈，中断栈只
/// 有1.5k，所以函数不要嵌套太深，特别要防止递归调用
#[export_name = "INT_TMR"]
unsafe extern "C" fn mtimer_irq_isr() {
    //isr_sprintln!("mtimer_irq_isr");
    //设置下一次中断时间
    super::Gd32vf103Porting::reset_systick();
    scheduler::systick();
}

/// 软中断服务函数，这里只要实现任务切换即可，上下文保存
/// 在port.S汇编代码里实现，这个函数工作在中断栈，同样
/// 要注意函数嵌套和递归调用
///
/// 任务切换原理
/// 1.保存当然cpu状态到当前任务栈（port.S里实现）
/// 2.保存任务当前栈顶地址到任务块第一个变量里，将来任务被切回来时要用到
/// 3.关软中断，防止被再一次触发；根据调度算法选择合适的任务，把全局任务指针指向新的任务
/// 4.从新任务恢复cpu状态在port.S里实现）
#[export_name = "INT_SFT"]
unsafe extern "C" fn soft_irq_isr() {
    // isr_sprintln!("soft_irq_isr");
    //关闭软中断
    super::Gd32vf103Porting::disable_irq();
    scheduler::schedule();
}

/// 所有任务的退出函数，调用exit函数即可
pub(crate) unsafe extern "C" fn task_exit() {
    scheduler::exit_current_task();
}
