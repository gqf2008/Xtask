use core::ffi::c_void;

use crate::task::executor::{xworker, Executor};
use crate::task::scheduler::xtask::IDLE_TASK;
use crate::{Task, IDLE_TASK_NAME};

pub(crate) fn start_idle_task() {
    fn idle_task(_args: *mut c_void) {
        loop {}
    }

    let task = Task::new(IDLE_TASK_NAME, 128, 16, idle_task, core::ptr::null_mut());
    unsafe {
        let _ = xworker.execute(task);
        let _ = core::mem::replace(&mut IDLE_TASK, task);
    }
}
