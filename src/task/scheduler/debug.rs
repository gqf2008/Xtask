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
            let ticks_sec = tick_ms() / 1000 / 60;
            sprintln!(
                ":{} systicks({}),ticks({}/{}min),used({}KiB),free({}KiB)",
                xworker.current().name(),
                systick(),
                tick(),
                ticks_sec,
                used_memory() / 1024,
                free_memory() / 1024
            );
            sprintln!("任务列表");
            print_task_list("running", xworker.current());
            unsafe {
                print_ready_task();
                print_delay_task();
                print_blocked_task();
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
    let readys = &[
        &Q1, &Q2, &Q3, &Q4, &Q5, &Q6, &Q7, &Q8, &Q9, &Q10, &Q11, &Q12, &Q13, &Q14, &Q15, &Q16,
    ];

    for q in readys.iter() {
        if let Some(q) = *q {
            q.iter().for_each(|item| {
                print_task_list("ready.task", *item);
            });
        }
    }
}
unsafe fn print_blocked_task() {
    use super::xtask::*;
    if let Some(q) = &BLOCKED {
        q.iter().for_each(|item| {
            print_task_list("block.task", *item);
        });
    }
}
unsafe fn print_delay_task() {
    use super::xtask::*;
    if let Some(q) = &DELAY {
        q.iter().for_each(|item| {
            print_task_list("delay.task", *item);
        });
    }
}

#[track_caller]
fn print_task_list(prefix: &str, task: *mut Task) {
    if let Some(task) = unsafe { task.as_mut() } {
        sprintln!(
            "{} '{}'/{}/{}/{}/{:?}",
            prefix,
            task.name(),
            task.priority,
            task.ticks,
            task.delay_ticks,
            task.state
        );
    }
}
