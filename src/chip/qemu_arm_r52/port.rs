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

/// 调度器已启动门控(irq_entry 判别用):0=启动阶段(CURRENT 可能已指向
/// 未首跑的 idle,但真实上下文是 main/Task::new 的启动栈——中断不得存帧);
/// 1=首任务已由 scheduler_loop 恢复,current!=NULL ⟺ 线程真实在跑。
/// ThreadX 不变式:current!=NULL 才允许补帧,本标志补齐它的启动半边
#[no_mangle]
static mut SCHED_STARTED: u8 = 0;

/// 中断抢占判定(irq_dispatch 调用):被打断任务(current 槽)是否仍是
/// 最高优先级就绪?r0=1 需要抢占(走补全存帧+调度循环),r0=0 原子弹回
#[no_mangle]
unsafe extern "C" fn irq_preempt_check() -> u32 {
    let cur = crate::task::scheduler::xworker::current_ptr();
    if cur.is_null() {
        return 0;
    }
    // DEBUG: current 的 sp 必须落在它自己的任务栈内(R5 49 字帧的
    // Task.sp = VFP 块底,在栈顶之下 196 字节处),否则帧归属已坏
    let sp = (*cur).sp;
    let s0 = (*cur).stack as usize;
    let s1 = s0 + (*cur).stack_size * core::mem::size_of::<usize>();
    if !(s0..s1).contains(&sp) {
        crate::sprint!("<BADSP cur={:p} sp={:#x}>", cur, sp);
    }
    let tz = crate::task::scheduler::xtask::highest_ready_prio();
    // 与调度器契约同款的 <=:do_systick(xtask.rs)对同优先级也判抢占
    // (时间片轮转),本口 IrqHandler 丢弃 systick 返回的 wake 布尔、
    // irq_to 又是 no-op——IRQ 出口判定是同优先级轮转的唯一触发通道,
    // 写成严格 < 会让"不主动让出"的任务永久霸占,同优先级任务饿死
    if tz < 16 && (tz as u32 + 1) <= (*cur).priority as u32 {
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

/// 任务初始帧构造(49 字 = 现场 16 字 + VFP 33 字,与 port.S 存/恢复
/// 路径互为镜像):
/// 现场低到高 = [0]r0(args) [1..13]r1-r12 [13]spsr [14]lr(蹦床)
/// [15]sp_svc; VFP 块(33 字)压在现场之下 = [16..47]D0-D15 [48]FPSCR,
/// Task.sp = VFP 块底(scheduler_loop 弹帧起点)
///
/// 首调度经调度循环弹帧: VFP 块 -> 现场 -> movs pc 跳蹦床 -> blx entry
/// (r0=args)进入任务。spsr = 0x13(SVC 模式、I/F 开、ARM 态)——任务
/// 首跑即中断开启; FPSCR 存启动值(默认 0 = 最接近舍入), 任务首跑
/// 浮点行为与复位后一致
#[inline]
pub(crate) fn save_context(task: &mut Task) {
    unsafe {
        let top = task.stack.add(task.stack_size - 1) as usize;
        let sp_svc = top & !7; // AAPCS:SP 8 字节对齐(堆顶只保证 4 对齐)
        let base = (sp_svc - 64) as *mut u32; // 现场帧底(r0 槽)
        let vfp = base.sub(33) as *mut u32; // VFP 块底(D0 槽)
        base.write_volatile(task.args.addr() as u32); // [0] r0 = args
        for i in 1..13usize {
            base.add(i).write_volatile(0); // [1..13] r1-r12 清零
        }
        base.add(13).write_volatile(0x13); // [13] spsr:SVC 模式,IRQ/FIQ 开
        base.add(14).write_volatile(_task_entry_trampoline as usize as u32); // [14] lr
        base.add(15).write_volatile(sp_svc as u32); // [15] sp_svc
        // VFP 块(33 字):D0-D15 + FPSCR——先清零占位,_start 已开 FPEXC.EN,
        // 首跑前由首帧构建;真正的寄存器内容在任务让出/抢占时由 port.S 存入
        for i in 0..33usize {
            vfp.add(i).write_volatile(0);
        }
        task.sp = vfp as usize; // 帧底(VFP 块底)= RESTORE 起点
        // DEBUG: 初始帧布局对照(与 switch_ctx 探针的 cur.sp 互查)
        crate::sprintln!(
            "save_ctx: task={:p} stack={:#x} frame={:#x} sp_svc={:#x} size={}",
            task as *mut Task as *mut u8,
            task.stack as usize,
            vfp as usize,
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
