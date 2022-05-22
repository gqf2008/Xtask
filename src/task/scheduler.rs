//! 调度器抽象
#[cfg(feature = "debug_task")]
mod debug;
mod idle;
pub(crate) mod xtask;
pub(crate) mod xworker;
use crate::task::executor::{xworker, Executor};
use crate::task::Task;
use crate::time;

#[cfg(feature = "xtask_scheduler")]
use xtask::XTaskScheduler;

/// 调度器默认实现，如果有更好的实现，那么请在这里替换它
#[allow(non_upper_case_globals)]
pub(crate) static schedulee: XTaskScheduler = ();

pub fn start() -> ! {
    #[cfg(feature = "timer")]
    crate::timer::start_timer_task();
    #[cfg(feature = "debug_task")]
    debug::start_debug_task();
    schedulee.start()
}
pub(crate) unsafe fn exit_current_task() {
    xworker.current().exit();
}

#[inline]
pub(crate) unsafe fn systick() -> bool {
    time::increase_tick();
    schedulee.do_systick()
}

pub(crate) unsafe fn schedule() {
    schedulee.do_schedule()
}

/// 调度器抽象
pub trait Scheduler {
    /// 调度器名称
    fn name(&self) -> &'static str {
        "Schedulee"
    }

    /// 启动调度器
    /// 用户程序里调用
    fn start(&self) -> !;

    /// 提交一个任务，不触发任务切换
    /// 用户程序里调用
    fn submit(&self, task: *mut Task);

    /// 心跳，推动任务状态机运行
    /// 分时任务调度精度就看这个函数的调用频率
    /// 只能在中断服务里调用
    fn do_systick(&self) -> bool;

    /// 把当前任务切出去
    /// 只能在中断服务里调用
    fn do_schedule(&self);
}
