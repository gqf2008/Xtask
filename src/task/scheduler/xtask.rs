use crate::port::{Portable, Porting};
use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::task::State;
use crate::task::{scheduler::Scheduler, Task, TaskQueue};
use alloc::boxed::Box;
use alloc::vec::Vec;
use bit_field::BitField;

use super::idle::start_idle_task;

pub(super) type XTaskScheduler = ();

impl Scheduler for XTaskScheduler {
    fn name(&self) -> &'static str {
        "XTaskScheduler"
    }

    /// 启动调度器
    fn start(&self) -> ! {
        unsafe {
            if !INITED {
                init_queue();
            }
        }
        start_idle_task();
        Porting::start_scheduler()
    }
    /// 提交一个任务进队列，待调度
    fn submit(&self, task: *mut Task) {
        sync::free(|_| unsafe { submit_task(task) });
    }

    fn do_systick(&self) -> bool {
        unsafe {
            let mut ready = false;
            //更新延时任务
            if let Some(delay) = &mut DELAY {
                let readys: Vec<usize> = delay
                    .iter()
                    .enumerate()
                    .filter_map(|(i, task)| {
                        if let Some(task) = (*task).as_mut() {
                            if task.tick() {
                                Some(i)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                readys.iter().for_each(|i| {
                    if let Some(task) = delay.remove(*i) {
                        submit_task(task);
                    }
                });
                ready = !readys.is_empty();
            }

            //如果延时队列没有就绪任务，那么再检查就绪队列
            if !ready {
                ready = READU_BITS.trailing_zeros() < 16;
            }
            //有就绪任务
            ready
        }
    }
    // 找到一个就绪任务把当前任务切出去
    fn do_schedule(&self) {
        unsafe {
            //弹出一个就绪任务
            let new = pop_ready();
            if new != xworker.current() {
                if let Some(new) = new.as_mut() {
                    if let Some(old) = xworker.execute(new).and_then(|item| item.as_mut()) {
                        //检查是否栈溢出
                        old.stack_overflow();
                        submit_task(old);
                    }
                }
            }
        }
    }
}

/// 任务入队列
#[track_caller]
#[inline(always)]
pub(crate) unsafe fn submit_task(task: *mut Task) {
    if !INITED {
        init_queue();
    }
    if let Some(task) = task.as_mut() {
        match task.state {
            State::Ready => {
                push_ready(task);
            }
            State::Blocked => {
                push_delay(task);
            }
            State::Suspended => {
                // task.bind_none();
                //push_blocked(task);
            }
            State::Terminated => {
                //就地删除
                let _ = Box::from_raw(task as *mut Task);
            }
            State::Running => {
                task.ready();
                push_ready(task);
            }
        }
    }
}

/// 查找并弹出就绪任务
/// 如果任务队列里没有就绪任务，则返回IDLE任务
#[inline(always)]
unsafe fn pop_ready() -> *mut Task {
    let trailing_zeros = READU_BITS.trailing_zeros();
    match match trailing_zeros {
        0 => &mut Q1,
        1 => &mut Q2,
        2 => &mut Q3,
        3 => &mut Q4,
        4 => &mut Q5,
        5 => &mut Q6,
        6 => &mut Q7,
        7 => &mut Q8,
        8 => &mut Q9,
        9 => &mut Q10,
        10 => &mut Q11,
        11 => &mut Q12,
        12 => &mut Q13,
        13 => &mut Q14,
        14 => &mut Q15,
        15 => &mut Q16,
        _ => return IDLE_TASK,
    } {
        Some(q) => {
            if let Some(task) = q.pop_front() {
                if q.is_empty() {
                    READU_BITS.set_bit(trailing_zeros as usize, false);
                }
                task
            } else {
                IDLE_TASK
            }
        }
        None => IDLE_TASK,
    }
}

/// 推入就绪队列
#[track_caller]
unsafe fn push_ready(task: *mut Task) {
    if let Some(task) = task.as_mut() {
        match task.priority {
            1 => {
                if let Some(q) = &mut Q1 {
                    task.bind(q);
                }
            }
            2 => {
                if let Some(q) = &mut Q2 {
                    task.bind(q);
                }
            }
            3 => {
                if let Some(q) = &mut Q3 {
                    task.bind(q);
                }
            }
            4 => {
                if let Some(q) = &mut Q4 {
                    task.bind(q);
                }
            }
            5 => {
                if let Some(q) = &mut Q5 {
                    task.bind(q);
                }
            }
            6 => {
                if let Some(q) = &mut Q6 {
                    task.bind(q);
                }
            }
            7 => {
                if let Some(q) = &mut Q7 {
                    task.bind(q);
                }
            }
            8 => {
                if let Some(q) = &mut Q8 {
                    task.bind(q);
                }
            }
            9 => {
                if let Some(q) = &mut Q9 {
                    task.bind(q);
                }
            }
            10 => {
                if let Some(q) = &mut Q10 {
                    task.bind(q);
                }
            }
            11 => {
                if let Some(q) = &mut Q11 {
                    task.bind(q);
                }
            }
            12 => {
                if let Some(q) = &mut Q12 {
                    task.bind(q);
                }
            }
            13 => {
                if let Some(q) = &mut Q13 {
                    task.bind(q);
                }
            }
            14 => {
                if let Some(q) = &mut Q14 {
                    task.bind(q);
                }
            }
            15 => {
                if let Some(q) = &mut Q15 {
                    task.bind(q);
                }
            }
            16 => {
                if let Some(q) = &mut Q16 {
                    task.bind(q);
                }
            }
            _ => {}
        }
        READU_BITS.set_bit((task.priority - 1) as usize, true);
    } else {
        panic!("put_task, illegal task {:p}", task);
    }
}

/// 推入延时队列
#[inline(always)]
unsafe fn push_delay(task: *mut Task) {
    if let Some(task) = task.as_mut() {
        if let Some(q) = &mut DELAY {
            task.bind(q);
        }
    }
}
// /// 推入阻塞队列
// #[inline(always)]
// unsafe fn push_blocked(task: *mut Task) {
//     if let Some(task) = task.as_mut() {
//         if let Some(q) = &mut BLOCKED {
//             task.bind(q);
//         }
//     }
// }
static mut INITED: bool = false;

unsafe fn init_queue() {
    DELAY.replace(TaskQueue::new());
    BLOCKED.replace(TaskQueue::new());
    Q1.replace(TaskQueue::new());
    Q2.replace(TaskQueue::new());
    Q3.replace(TaskQueue::new());
    Q4.replace(TaskQueue::new());
    Q5.replace(TaskQueue::new());
    Q6.replace(TaskQueue::new());
    Q7.replace(TaskQueue::new());
    Q8.replace(TaskQueue::new());
    Q9.replace(TaskQueue::new());
    Q10.replace(TaskQueue::new());
    Q11.replace(TaskQueue::new());
    Q12.replace(TaskQueue::new());
    Q13.replace(TaskQueue::new());
    Q14.replace(TaskQueue::new());
    Q15.replace(TaskQueue::new());
    Q16.replace(TaskQueue::new());
    INITED = true;
}

static mut READU_BITS: u16 = 0;

/// 空闲任务，没有就绪任务时就切到这个任务
pub(crate) static mut IDLE_TASK: *mut Task = core::ptr::null_mut();

/// 延时队列
pub(crate) static mut DELAY: Option<TaskQueue> = None;

/// 阻塞队列
pub(crate) static mut BLOCKED: Option<TaskQueue> = None;

/// 1-16优先级任务就绪队列，数字越小优先级越高
pub(crate) static mut Q1: Option<TaskQueue> = None;
pub(crate) static mut Q2: Option<TaskQueue> = None;
pub(crate) static mut Q3: Option<TaskQueue> = None;
pub(crate) static mut Q4: Option<TaskQueue> = None;
pub(crate) static mut Q5: Option<TaskQueue> = None;
pub(crate) static mut Q6: Option<TaskQueue> = None;
pub(crate) static mut Q7: Option<TaskQueue> = None;
pub(crate) static mut Q8: Option<TaskQueue> = None;
pub(crate) static mut Q9: Option<TaskQueue> = None;
pub(crate) static mut Q10: Option<TaskQueue> = None;
pub(crate) static mut Q11: Option<TaskQueue> = None;
pub(crate) static mut Q12: Option<TaskQueue> = None;
pub(crate) static mut Q13: Option<TaskQueue> = None;
pub(crate) static mut Q14: Option<TaskQueue> = None;
pub(crate) static mut Q15: Option<TaskQueue> = None;
pub(crate) static mut Q16: Option<TaskQueue> = None;
