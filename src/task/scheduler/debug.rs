//! 打印出任务列表

use crate::task::executor::{xworker, Executor};
use crate::task::sleep_ms;
use crate::task::Task;
use crate::*;
use core::ffi::c_void;

use crate::task::DEBUG_TIMER_NAME;

pub(crate) fn start_debug_task() {
    fn debug_task(_args: *mut c_void) {
        loop {
            print_task_list(xworker.current());
            unsafe {
                print_ready_task();
                print_delay_task();
            }
            sleep_ms(5000);
        }
    }

    let task = Task::new(DEBUG_TIMER_NAME, 256, 16, debug_task, core::ptr::null_mut());
    unsafe {
        crate::task::scheduler::xtask::submit_task(task);
    }
}
unsafe fn print_ready_task() {
    use super::xtask::*;
    for hart in READYQ.iter() {
        for q in hart.iter() {
            q.iter().for_each(|item| {
                print_task_list(*item);
            });
        }
    }
}
unsafe fn print_delay_task() {
    use super::xtask::*;
    DELAY.iter().for_each(|item| {
        print_task_list(*item);
    });
}

#[track_caller]
fn print_task_list(task: *mut Task) {
    if let Some(task) = unsafe { task.as_mut() } {
        log::debug!(
            "'{}'/{}/{}/{}/{:?}",
            task.name(),
            task.priority,
            task.ticks,
            task.wake_tick,
            task.state
        );
    }
}
