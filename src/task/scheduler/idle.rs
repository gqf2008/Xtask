use core::ffi::c_void;

use crate::port::{Portable, Porting, MAX_HARTS};
use crate::task::executor::{xworker, Executor};
use crate::task::scheduler::xtask::IDLE_TASKS;
use crate::{Task, IDLE_TASK_NAME};

/// 启动每核 idle 任务:参与调度的每核各一个(同一任务块不能在两核并发)。
/// 本核先以 idle 为当前任务——`start_scheduler` 恢复的第一帧就是它
pub(crate) fn start_idle_task() {
    fn idle_task(_args: *mut c_void) {
        loop {}
    }

    let n = Porting::core_count().min(MAX_HARTS as u16);
    for h in 0..n {
        let task = Task::new(IDLE_TASK_NAME, 128, 16, idle_task, core::ptr::null_mut());
        unsafe {
            IDLE_TASKS[h as usize] = task;
        }
    }
    unsafe {
        // 只 execute 本核的 idle(置 CURRENT[本核]);别核的 CURRENT
        // 由其进调度后的首个 do_schedule 自行装载
        let _ = xworker.execute(IDLE_TASKS[(Porting::hart_id() as usize).min(MAX_HARTS - 1)]);
    }
}
