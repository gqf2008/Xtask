//! QEMU xlnx-zcu102 Cortex-R5F 中断胶水:IRQ 入口(port.S)分派与
//! 初始任务帧构造。

use core::arch::global_asm;

use super::{GICC, TTC, TTC_SPI_INTID};
use crate::port::Portable;
use crate::task::scheduler;
use crate::task::Task;

global_asm!(include_str!("port.S"), options(raw));

/// 中断嵌套计数(port.S irq_entry 三路判别用;ThreadX
/// _tx_thread_system_state 同款)——非零表示嵌套中断,绝不触碰任务帧
#[no_mangle]
static mut IRQ_NESTING: u32 = 0;

/// 中断抢占判定(irq_dispatch 调用):被打断任务(current 槽)是否仍是
/// 最高优先级就绪?r0=1 需要抢占(走补全存帧+调度循环),r0=0 原子弹回
#[no_mangle]
unsafe extern "C" fn irq_preempt_check() -> u32 {
    let cur = crate::task::scheduler::xworker::current_ptr();
    if cur.is_null() {
        return 0;
    }
    // DEBUG: current 的 sp 落在堆区之外 = 帧归属已坏,打印现场
    let sp = (*cur).sp;
    if !(0x0010_7000usize..0x0010_a000).contains(&sp) {
        crate::sprint!("<BADSP cur={:p} sp={:#x}>", cur, sp);
    }
    let tz = crate::task::scheduler::xtask::highest_ready_prio();
    if tz < 16 && (tz as u32 + 1) < (*cur).priority as u32 {
        1
    } else {
        0
    }
}

/// IRQ 入口统一 handler(port.S switch_and_restore 调用):
/// - 真中断(IRQ 入口):IAR 取号 → TTC 清中断 + 记拍 → EOI;
/// - yield 误入(SVC 入口也会调):IAR = 1023(spurious)直接返回。
/// 电平源 TTC 在区间模式下自动重装,无需手动重写间隔
#[no_mangle]
unsafe extern "C" fn IrqHandler() {
    let gicc = GICC as *mut u32;
    let iar = gicc.add(0x0C / 4).read_volatile(); // GICC_IAR
    let intid = iar & 0x3FF;
    if intid == TTC_SPI_INTID {
        // 读 INT_REG(0x54)清中断——电平型,不清则 GIC 永远 pending
        (TTC as *mut u32).add(0x54 / 4).read_volatile();
        scheduler::systick();
    }
    if intid != 0x3FF {
        gicc.add(0x10 / 4).write_volatile(intid); // GICC_EOI
    }
}

/// 软中断等价入口:port.S 公共出口调用,执行调度与切换
#[no_mangle]
unsafe extern "C" fn switch_context() {
    scheduler::schedule();
}

/// 所有任务的退出函数(port.S 首调蹦床以 `b task_exit` 引用——保持
/// 符号不改名)
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn task_exit() {
    scheduler::exit_current_task();
}

/// 任务初始帧构造(16 字,与 port.S SAVE_CTX/RESTORE_CTX 互为镜像):
/// 帧低→高 = [0]r0(args) [1..13]r1-r12 [13]spsr [14]lr(蹦床) [15]sp_svc
///
/// 首调度经 RESTORE_CTX 弹帧:r0 = args,lr = 蹦床地址,ldr sp,[15]
/// 弹回任务栈顶,movs pc 跳蹦床 → blx entry(r0=args)进入任务。
/// spsr = 0x13(SVC 模式、I/F 开、ARM 态)——任务首跑即中断开启
#[inline]
pub(crate) fn save_context(task: &mut Task) {
    unsafe {
        let top = task.stack.add(task.stack_size - 1) as usize;
        let sp_svc = top & !7; // AAPCS:SP 8 字节对齐(堆顶只保证 4 对齐)
        let base = (sp_svc - 64) as *mut u32; // 帧底(r0 槽)
        base.write_volatile(task.args.addr() as u32); // [0] r0 = args
        for i in 1..13usize {
            base.add(i).write_volatile(0); // [1..13] r1-r12 清零
        }
        base.add(13).write_volatile(0x13); // [13] spsr:SVC 模式,IRQ/FIQ 开
        base.add(14).write_volatile(_task_entry_trampoline as usize as u32); // [14] lr
        base.add(15).write_volatile(sp_svc as u32); // [15] sp_svc
        task.sp = base as usize; // 帧底 = RESTORE 起点
        // DEBUG: 初始帧布局对照(与 switch_ctx 探针的 cur.sp 互查)
        crate::sprintln!(
            "save_ctx: task={:p} stack={:#x} frame={:#x} sp_svc={:#x} size={}",
            task as *mut Task as *mut u8,
            task.stack as usize,
            base as usize,
            sp_svc,
            task.stack_size
        );
    }
}

/// Task 布局偏移(port.S 蹦床 `ldr r0, [r12, #8]` 取 entry)——失配在
/// 编译期炸,而非静默错跳
const _: () = assert!(core::mem::offset_of!(Task, sp) == 0);
const _: () = assert!(core::mem::offset_of!(Task, entry) == 8);

// Rust 侧引用蹦床地址(save_context 写帧 lr 槽)
extern "C" {
    fn _task_entry_trampoline();
}
