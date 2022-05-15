//! 杂项

use crate::chip::{DEBUG_TASK_ENABLE, TIMER_TASK_ENABLE};
use crate::scheduler::Scheduler;
use crate::task::executor::{xworker, Executor};
use crate::*;

use crate::task::{sleep_ms, DEBUG_TIMER_NAME};
use crate::task::{Task, TIMER_TASK_NAME};
use core::ffi::c_void;

/// 启动几个系统任务
pub(crate) fn start_sys_task() {
    if TIMER_TASK_ENABLE {
        start_timer_task();
    }
    if DEBUG_TASK_ENABLE {
        start_debug_task();
    }
}

fn start_timer_task() {
    fn timer_task(_args: *mut c_void) {
        loop {
            sprintln!(":{} {}", xworker.current().name(), systick());
            sleep_ms(1000);
        }
    }

    let task = Task::new(TIMER_TASK_NAME, 1024, 1, timer_task, core::ptr::null_mut());
    crate::task::schedulee.submit(task);
}

fn start_debug_task() {
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
    crate::task::schedulee.submit(task);
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
