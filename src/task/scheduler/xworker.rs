use crate::task::executor::Executor;
use crate::task::Task;

/// 全局任务指针，指向当前正在运行的任务
#[cfg(feature = "xtask_executor")]
#[export_name = "CURRENT_TASK_PTR"]
static mut CURRENT_TASK: *mut Task = core::ptr::null_mut();

/// 单硬件线程默认实现
#[cfg(feature = "xtask_executor")]
pub(crate) type XTaskExecutor = ();

impl Executor for XTaskExecutor {
    fn current(&self) -> &'static mut Task {
        unsafe { CURRENT_TASK.as_mut().unwrap() }
    }
    fn execute(&self, task: *mut Task) -> Option<*mut Task> {
        unsafe {
            if let Some(task) = task.as_mut() {
                task.run();
                Some(core::mem::replace(&mut CURRENT_TASK, task))
            } else {
                None
            }
        }
    }
    fn halt(&self) {
        panic!("halt")
    }
}
