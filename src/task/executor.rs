//! 硬件工作线程抽象以及单线程默认实现
//! 这里参考FreeRTOS实现，一个全局指针指向正在执行的任务

#[cfg(feature = "xtask_executor")]
use crate::task::scheduler::xworker::XTaskExecutor;
use crate::task::Task;

/// 如果移植层提供了更好的实现，那么请在这里替换它
/// 否则采用默认实现
#[allow(non_upper_case_globals)]
pub(crate) static xworker: XTaskExecutor = ();

/// 硬件工作线程抽象
pub trait Executor {
    /// 硬件线程数
    fn threads() -> u16;
    fn current(&self) -> &'static mut Task;
    fn execute(&self, task: *mut Task) -> Option<*mut Task>;
    fn halt(&self);
}
