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
            //更新延时任务，把到期（tick 递减到 0）的任务重新提交调度
            if let Some(delay) = &mut DELAY {
                for task in take_expired(delay) {
                    submit_task(task);
                }
            }

            // 检查尾零数，是否有比当前任务相等或更高优先级的任务
            // 如果想等优先级则时间片调度，否则就一直抢占着，直到任务主动挂起
            // TODO 需改进 ARM CLZ指令计算前导零
            let trailing_zero = READY_BITS.trailing_zeros();
            trailing_zero < 16 && (trailing_zero + 1) <= self.current().priority as u32
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

/// 从延时队列取出所有到期任务（tick 递减到 0、状态回到 Ready 的任务）。
/// 抽成纯函数以便 host 单测直接驱动。
/// 关键不变式：必须**降序**删除——`VecDeque::remove(i)` 会使其后元素前移，
/// 升序删除多个任务时下标错位，会删错任务或漏唤醒。
#[inline(always)]
pub(crate) fn take_expired(delay: &mut TaskQueue) -> Vec<*mut Task> {
    let mut readys: Vec<usize> = delay
        .iter()
        .enumerate()
        .filter_map(|(i, &task)| {
            // SAFETY: 延时队列中的任务指针入队时均为有效 Task；此函数只在临界区
            // （ISR 关中断）或 host 单线程测试下被调用，无二度可变别名。
            unsafe { task.as_mut() }.and_then(|t| if t.tick() { Some(i) } else { None })
        })
        .collect();
    readys.sort_unstable_by(|a, b| b.cmp(a));
    readys.iter().filter_map(|i| delay.remove(*i)).collect()
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
    let trailing_zeros = READY_BITS.trailing_zeros();
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
                    READY_BITS.set_bit(trailing_zeros as usize, false);
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
        READY_BITS.set_bit((task.priority - 1) as usize, true);
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

static mut READY_BITS: u16 = 0;

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

#[cfg(test)]
mod tests {
    use super::take_expired;
    use crate::task::{State, Task, TaskQueue};
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use core::ffi::c_void;

    fn dummy_entry(_args: *mut c_void) {}

    /// 构造一个阻塞态、delay_ticks 到期的任务（模拟 sleep 到点）。
    fn blocked_task(ticks: usize) -> *mut Task {
        let t = Task::new("t", 128, 8, dummy_entry, core::ptr::null_mut());
        // SAFETY: 刚创建的独占任务，直接设置字段模拟"延时中"状态
        unsafe {
            (*t).delay_ticks = ticks;
            (*t).state = State::Blocked;
        }
        t
    }

    /// 回收任务，避免测试泄漏（Task 的 Drop 会一并释放任务栈）
    unsafe fn reclaim(ptrs: &[*mut Task]) {
        for &p in ptrs {
            drop(Box::from_raw(p));
        }
    }

    /// 回归：多个任务同一 tick 到期时必须全部取出（bug #1）。
    /// 修复前升序 `remove(i)` 会因元素前移导致下标错位：3 个到期任务只删掉前 2 个、漏 1 个。
    #[test]
    fn take_expired_removes_all_without_index_slippage() {
        let mut q = TaskQueue::new();
        let ptrs: Vec<*mut Task> = (0..3).map(|_| blocked_task(1)).collect();
        for &p in &ptrs {
            q.push_back(p);
        }

        let expired = take_expired(&mut q);

        assert_eq!(expired.len(), 3, "3 个到期任务都应被取出");
        assert!(q.is_empty(), "延时队列应被清空");
        unsafe { reclaim(&ptrs) };
    }

    /// 回归：未到期的任务必须留在队列里。
    #[test]
    fn take_expired_keeps_unexpired() {
        let mut q = TaskQueue::new();
        let expired_one = blocked_task(1); // 本次 tick 到期
        let pending = blocked_task(5); // 还剩 4 tick，未到期
        q.push_back(expired_one);
        q.push_back(pending);

        let expired = take_expired(&mut q);

        assert_eq!(expired.len(), 1);
        assert_eq!(q.len(), 1, "未到期任务必须保留");
        assert!(core::ptr::eq(q[0], pending));
        unsafe { reclaim(&[expired_one, pending]) };
    }

    /// 回归：空队列安全。
    #[test]
    fn take_expired_empty_queue() {
        let mut q = TaskQueue::new();
        assert!(take_expired(&mut q).is_empty());
    }
}
