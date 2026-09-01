//! GD32VF103移植模块实现
//! 使用了gd32vf103xx_hal硬件抽象层的配置中断模块
//! 一部分由rust实现，一部分由汇编实现，请参考hal库汇编代码

mod port;
pub mod stdout;
#[cfg(feature = "usb")]
pub mod usb;

use super::{CPU_CLOCK_HZ, SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ, TIMER_CTRL_ADDR};
use crate::port::Portable;
use crate::prelude::CriticalSection;
use crate::task::scheduler;
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

/// 每拍 mtime 计数 = SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ。
/// reset_systick 重装、一次性武装(tickless_arm_delta)、tick ISR
/// 实测 el 三处共用(修前各写各的局部 const,名称/类型也不一致)
pub(crate) const TICK_PERIOD: u64 = (SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u64;

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

// ---- tickless 动态节拍(ch29)----
// 与 qemu_riscv 口同构:一次性武装 mtimecmp,tick ISR 实测拍数跳账。

/// 一次性武装时刻(= 0 未武装/恒定节拍模式)。单核独占(tickless
/// 门控在单核语义):volatile 而非原子——与 TICKS 同款处理
static mut TICKLESS_ARMED: vcell::VolatileCell<u64> = vcell::VolatileCell::new(0);

/// gd32芯片移植层实现
pub struct Gd32vf103Porting;

// port.S 蹦床 `_task_entry_trampoline` 依赖的 Task 布局偏移(失配编译期炸)
const _: () = assert!(core::mem::offset_of!(Task, sp) == 0);
const _: () = assert!(core::mem::offset_of!(Task, entry) == 8);

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
        log::info!("Start scheduler");
        //从任务栈恢复CPU状态，汇编实现
        unsafe { asm!(include_str!("restore_ctx.S"), options(noreturn, raw)) };
        //这个函数不会返回，因为在汇编中最后一条指令是mret，而不是ret
        //mret把mepc更新到PC，而ret把ra更新到PC
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

    // ---- tickless 动态节拍(ch29,见 book/src/ch29-tickless.md)----

    #[inline]
    fn tickless_supported() -> bool {
        true
    }
    /// 一次性武装:cmp = 当前 mtime + delta×PERIOD(走 set_mtimecmp 的
    /// lo=0xffffffff 先行防中途匹配);TICKLESS_ARMED 记武装时刻,到点
    /// ISR 实测 el 跳账。整段在临界区内(单核 ISR 不可插入,无竞态);
    /// 停表可能已 mask 掉定时器中断,顺带补开
    #[inline]
    fn tickless_arm_delta(delta_ticks: u64) {
        Self::free(|_| unsafe {
            let now = Gd32vf103Porting::systick();
            TICKLESS_ARMED.set(now);
            set_mtimecmp(now + delta_ticks * TICK_PERIOD);
            ECLIC::unmask(Interrupt::INT_TMR);
        });
    }
    /// 停表:mask 定时器中断 + cmp 推到 64 位上限(双保险,理由同 qemu 口)
    #[inline]
    fn tickless_stop_timer() {
        ECLIC::mask(Interrupt::INT_TMR);
        set_mtimecmp(u64::MAX);
    }
    /// 睡眠等待中断(wfi:任意已使能中断 pending 即返回)
    #[inline]
    fn tickless_wait() {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
    /// 即将离开本核 idle(调度器回调):放弃未到期的一次性武装、按实测
    /// 补账(只补整拍,子拍不记),并把节拍拨回恒定。没有这一步,
    /// "睡眠中被外部中断早醒 → 有任务运行 → idle 重新武装"会把新武装
    /// 锚在冻结的 TICKS 上——墙钟期限被每个清醒片段整体拖后;任务
    /// 运行期也没有逐拍时间片/到期摘取
    #[inline]
    fn tickless_leave_idle() {
        // tick 主核独占(ch25 ⑤):从核从不武装/停表,也不许恢复
        if Self::hart_id() != 0 {
            return;
        }
        Self::free(|_| unsafe {
            let armed = TICKLESS_ARMED.get();
            if armed != 0 {
                TICKLESS_ARMED.set(0);
                let el = Gd32vf103Porting::systick().wrapping_sub(armed) / TICK_PERIOD;
                if el > 0 {
                    scheduler::systick_jump(el);
                }
            }
            // 无论睡眠形态(武装深睡/停表长眠),离开空闲都回恒定节拍。
            // 先装回 cmp 再补开:停在陈旧 cmp(≤mtime)上补开会立触发
            reset_systick();
            ECLIC::unmask(Interrupt::INT_TMR);
        });
    }
    /// 恒定节拍兜底自旋前的"节拍恢复"(自愈幂等):长眠期间应用/ISR
    /// 关掉 tickless 后,自旋等的是被 mask 的节拍中断——不恢复就饿死。
    /// 读 ECLIC 使能位自检:未停表时一行分支即返回
    #[inline]
    fn tickless_resume_periodic() {
        if Self::hart_id() != 0 {
            return;
        }
        if !ECLIC::is_enabled(Interrupt::INT_TMR) {
            reset_systick();
            unsafe { ECLIC::unmask(Interrupt::INT_TMR); }
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
            // mepc = 首调蹦床:经标准 jalr 进入 task.entry(mret 直入会被
            // 编译器 outlined 的入口 stub 坑到野跳,见 port.S 蹦床注)
            unsafe extern "C" {
                fn _task_entry_trampoline();
            }
            sp.offset(-3)
                .write_volatile((_task_entry_trampoline as *const ()).addr()); // mepc
            sp.offset(-4).write_volatile(0x00000C80);
            for i in 0..32usize {
                if i != 1 && i != 10 {
                    sp.offset(i as isize - 36).write_volatile(0);
                }
            }
            sp.offset(-26).write_volatile(task.args.addr());
            sp.offset(-35)
                .write_volatile((port::task_exit as *const ()).addr());
            task.sp = sp.offset(-36).addr();
        }
    }
}

/// 设置 mtimecmp 比较寄存器(lo=0xffffffff 先写防中途匹配,标准 64 位
/// 写序;tickless 一次性武装与周期重装共用)
#[inline]
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

/// 重新设置mtimecmp寄存器
/// mtimecmp=TICKS+mtime的值，当mtimecmp的值大于等于mtime时触发定时器中断
#[inline]
pub(crate) fn reset_systick() {
    let mtime = Gd32vf103Porting::systick();
    set_mtimecmp(TICK_PERIOD + mtime);
}
