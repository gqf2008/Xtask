//! GD32VF103移植模块实现
//! 使用了gd32vf103xx_hal硬件抽象层的配置中断模块
//! 一部分由rust实现，一部分由汇编实现，请参考hal库汇编代码

mod port;

use super::{CPU_CLOCK_HZ, SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ, TIMER_CTRL_ADDR};
use crate::port::Portable;
use crate::prelude::CriticalSection;
use crate::sprintln;
use crate::task::Task;
use core::arch::asm;
use gd32vf103xx_hal::eclic::*;
use gd32vf103xx_hal::pac::{Interrupt, ECLIC};

/// mtime计数器寄存器偏移量
const TIMER_MTIME: usize = 0x0;
/// mtimecmp比较寄存器偏移量
const TIMER_MTIMECMP: usize = 0x8;
/// msip软中断寄存器偏移量
const TIMER_MSIP: usize = 0xFFC;

/// 配置定时器、软中断、使能定时器中断和软中断
#[inline]
pub(crate) fn setup_intrrupt() {
    unsafe {
        //设置定时器中断
        ECLIC::setup(
            //定时器中断号
            Interrupt::INT_TMR,
            //上升沿触发
            TriggerType::RisingEdge,
            //中断等级
            Level::L0,
            //中断优先级
            Priority::P0,
        );
        //设置软中断
        ECLIC::setup(
            //软中断号
            Interrupt::INT_SFT,
            //上升沿触发
            TriggerType::RisingEdge,
            //中断等级
            Level::L0,
            //中断优先级
            Priority::P0,
        );
        //定时器中断使能
        ECLIC::unmask(Interrupt::INT_TMR);
        //软中断使能
        ECLIC::unmask(Interrupt::INT_SFT);
    }
}

/// gd32芯片移植层实现
pub struct Gd32vf103Porting;

impl Portable for Gd32vf103Porting {
    /// 完全内存屏障
    /// 保证在屏障之前的任何存储操作先于屏障之后的代码执行。
    #[inline]
    fn barrier() {
        unsafe {
            riscv::asm::sfence_vma_all();
        }
    }
    /// 临界区保护
    #[inline]
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        riscv::interrupt::free(f)
    }

    /// 开全局中断
    #[inline]
    fn enable_interrupt() {
        unsafe {
            riscv::interrupt::enable();
        }
    }
    /// 关全局中断
    #[inline]
    fn disable_interrupt() {
        unsafe {
            riscv::interrupt::disable();
        }
    }

    /// 启动调度器
    /// 1. 配置定时器中断、软中断触发类型和优先级
    /// 2、把第一个任务恢复到CPU中，内联汇编实现
    fn start_scheduler() -> ! {
        reset_systick();
        //配置中断，这个函数就是定时中断和软中断使能
        setup_intrrupt();
        //从任务栈恢复CPU状态，汇编实现
        unsafe { asm!(include_str!("restore_ctx.S")) };
        //这个函数不会返回，因为在汇编中最后一条指令是mret，而不是ret
        //mret把mepc更新到PC，而ret把ra更新到PC

        panic!("~!@#$%^&*()_");
    }

    /// 软中断
    /// 当软中断被打开时触发软中断，直到软中断或者全局中断关闭为止
    #[inline]
    fn irq() {
        let ptr = (TIMER_CTRL_ADDR + TIMER_MSIP) as *mut u8;
        unsafe {
            ptr.write_volatile(*ptr | 0x01);
        }
    }
    /// 关闭软中断
    #[inline]
    fn disable_irq() {
        let ptr = (TIMER_CTRL_ADDR + TIMER_MSIP) as *mut u8;
        unsafe {
            ptr.write_volatile(*ptr & !0x01);
        }
    }

    /// 读取计数器寄存器的值，保存了从CPU工作开始到现在的rtc tick数
    /// mtime是个可读写且单调递增寄存器，通常不要去设置它，让它一直保存单调递增即可
    /// 有两个32位寄存器组成，共64位，所以在已知的生命周期内不用考虑这个值的溢出
    #[inline]
    fn systick() -> u64 {
        loop {
            let hi = unsafe { *((TIMER_CTRL_ADDR + TIMER_MTIME + 4) as *mut u32) };
            let lo = unsafe { *((TIMER_CTRL_ADDR + TIMER_MTIME) as *mut u32) };
            if hi == unsafe { *((TIMER_CTRL_ADDR + TIMER_MTIME + 4) as *mut u32) } {
                return (hi as u64) << 32 | (lo as u64);
            }
        }
    }

    /// 重新设置mtimecmp寄存器
    /// mtimecmp=TICKS+mtime的值，当mtimecmp的值大于等于mtime时触发定时器中断

    /// 硬件延时，单位us
    #[inline]
    fn delay_us(us: u64) {
        let t0 = riscv::register::mcycle::read64();
        let clock = (us * (CPU_CLOCK_HZ as u64)) / 1_000_000;
        while riscv::register::mcycle::read64().wrapping_sub(t0) <= clock {}
    }

    /// 任务创建时为CPU准备好任务的现场，一共32个参数，占用36*4个字节
    /// 包括入口函数、任务参加、返回地址、任务栈顶指针
    /// 这段程序与汇编中实现的上文保存是同一个功能，唯一不同的是
    /// 任务初始化时栈顶就是数组的尾地址，任务运行过程中栈顶的位置是不确定的
    ///
    /// 0. 任务栈SP保存在任务sp字段，也是任务的第一个参数
    /// 1. 任务入口函数就是当任务被第一次运行时的PC地址，保存在mepc寄存器中，
    /// 2. 任务函数参数保存在a0寄存器中，寄存器编号为x10
    /// 3. 任务返回地址保存在ra寄存器中，寄存器编号为x1
    /// 4. 其他通用寄存器按约定的顺序保存在任务堆栈中即可
    #[inline]
    fn save_context(task: &mut Task) {
        unsafe {
            //任务栈指针移到栈顶，也就是数组的最后一个元素起始位置
            let sp = task.stack.add(task.stack_size - 1);
            // 需要8字节对齐，参考FreeRTOS，说是为了双精度浮点运算，还没搞明白，暂且注释掉
            //sp = ((sp as usize) & !(0x0007)) as *mut usize;
            /*
             *  从任务栈顶开始压栈，压栈顺序如下，一共32个值，占用36*4字节任务栈空间
             *  mcause=0xb8000000，30:31为中断号（7），0:29为异常原因
             *  msubm(0x7c4)=0x40，自定义寄存器用于保存Core当前的Trap类型，以及进入Trap前的Trap类型。详见《Bumblebee内核指令架构手册》
             *  mepc=task.entry，出栈后mret指令会用mepc的值赋值给PC，这样就可以进入用户任务函数了
             *  mstatus=0x000000C80，mpp[11:12]和mpie[7]置位，mpp在机器模式==11，mpie=1当任务恢复后打开全局中断
             *  x31-x11 默认0
             *  x10(a0)=task.args，任务函数参数
             *  x9-x2 默认0
             *  x1(ra)=task_exit_error，任务返回地址
             *  x0 保留，任务栈sp指向这里
             */
            sp.offset(-1).write_volatile(0xb8000000);
            sp.offset(-2).write_volatile(0x40);
            sp.offset(-3)
                .write_volatile((task.entry as *const ()).addr());
            sp.offset(-4).write_volatile(0x00000C80);
            sp.offset(-26).write_volatile(task.args.addr());
            sp.offset(-35)
                .write_volatile((port::task_exit as *const ()).addr());
            task.sp = sp.offset(-36).addr();
        }
    }
}

#[inline]
pub(crate) fn reset_systick() {
    /// TICKS=RTC_CLOCK_HZ（RTC时钟频率）/ TICK_CLOCK_HZ（TICK频率）
    /// RTC_CLOCK_HZ、TICK_CLOCK_HZ在env.rs里配置
    const TICKS: usize = SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ;
    /// 设置mtimecmp比较寄存器
    fn set_mtimecmp(v: u64) {
        let hi = ((v >> 32) as u32) & 0xffffffff;
        let lo = (v as u32) & 0xffffffff;
        let mtimecmp_lo = (TIMER_CTRL_ADDR + TIMER_MTIMECMP) as *mut u32;
        let mtimecmp_hi = (TIMER_CTRL_ADDR + TIMER_MTIMECMP + 4) as *mut u32;
        unsafe {
            mtimecmp_lo.write_volatile(0xffffffff);
            mtimecmp_hi.write_volatile(hi);
            mtimecmp_lo.write_volatile(lo);
        }
    }
    let mtime = Gd32vf103Porting::systick();
    let mtimecmp = TICKS as u64 + mtime;
    set_mtimecmp(mtimecmp);
}
