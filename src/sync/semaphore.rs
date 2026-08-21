//! 计数信号量，公平调度，多对多通知

use crate::sync::Error;
use crate::task::executor::{xworker, Executor};
use crate::task::Task;
use crate::TaskQueue;
use crate::{sync, yield_now};
// use alloc::sync::Arc;
use super::arc::Arc;
use core::cell::RefCell;
// use core::sync::atomic::AtomicUsize;
// use core::sync::atomic::Ordering;
use atomic_polyfill::{AtomicUsize, Ordering};
// use crossbeam::atomic::AtomicCell;

/// 信号量
/// 设计思想是维护两个任务挂起队列
/// 当信号量为零时挂起当前任务到挂起队列
/// 当信号量大于零时从挂起队列弹出任务交给调度器
#[derive(Clone)]
pub struct Semaphore {
    waiters: Arc<RefCell<TaskQueue>>,
    notifiers: Arc<RefCell<TaskQueue>>,
    signal: Arc<AtomicUsize>, //信号量
    max_value: usize,
}

unsafe impl Send for Semaphore {}

impl Semaphore {
    pub fn new() -> Self {
        Self::with_signal(0)
    }

    pub fn with_signal(signal: usize) -> Self {
        Self::with_signal_max_value(signal, usize::MAX)
    }

    pub fn with_max_value(max_value: usize) -> Self {
        Self::with_signal_max_value(0, max_value)
    }

    pub fn with_signal_max_value(signal: usize, max_value: usize) -> Self {
        Self {
            waiters: Arc::new(RefCell::new(TaskQueue::new())),
            notifiers: Arc::new(RefCell::new(TaskQueue::new())),
            signal: Arc::new(AtomicUsize::new(signal)),
            max_value: max_value,
        }
    }
}

impl Semaphore {
    /// 发送信号
    /// 可以在中断服务中使用
    ///
    pub fn post_isr(&self) -> nb::Result<(), Error> {
        //带边界的原子自增：仅当未达上限时才 +1，避免越界和失败泄漏计数
        match self
            .signal
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |signal| {
                if signal >= self.max_value {
                    None
                } else {
                    Some(signal + 1)
                }
            }) {
            Ok(_) => {
                unsafe {
                    if let Some(waiter) = self.waiters.borrow_mut().pop_front() {
                        if let Some(waiter) = waiter.as_mut() {
                            waiter.wakeup();
                        }
                    }
                };
                Ok(())
            }
            Err(_) => Err(nb::Error::Other(Error::SemaphoreFull)),
        }
    }
    /// 发送信号
    /// 不能在中断服务中使用
    pub fn post(&self) {
        loop {
            //"尝试计数 + 失败则入队挂起"必须在同一个临界区内完成，
            //否则两者之间存在窗口：对方在此时完成 wait 弹出 notifiers 时队列还是空的，
            //本任务随后入队挂起，信号量已有空余容量却无人唤醒（丢失唤醒）。
            //带边界的原子自增：仅当未达上限时才 +1，避免越界和失败泄漏计数
            let posted = sync::free(|_| {
                let posted = self
                    .signal
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |signal| {
                        if signal >= self.max_value {
                            None
                        } else {
                            Some(signal + 1)
                        }
                    })
                    .is_ok();
                if posted {
                    unsafe {
                        if let Some(waiter) = self.waiters.borrow_mut().pop_front() {
                            if let Some(waiter) = waiter.as_mut() {
                                waiter.wakeup();
                            }
                        }
                    }
                } else {
                    let task = xworker.current();
                    self.notifiers.borrow_mut().push_back(task as *mut Task);
                    task.block();
                }
                posted
            });
            if posted {
                break;
            }
            yield_now();
        }
    }

    /// 等待一个信号量
    /// 禁止在中断服务中调用
    /// 注意：不要同时使用post_isr和post，不然可能会错误的唤醒poster
    pub fn wait(&self) {
        loop {
            //与 post 同理："尝试取信号 + 失败则入队挂起"必须在同一个临界区内完成，
            //否则对方在此窗口内 post 会发现 waiters 为空，本任务挂起后无人唤醒（丢失唤醒）。
            let acquired = sync::free(|_| {
                let acquired = self
                    .signal
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |signal| {
                        if signal == 0 {
                            None
                        } else {
                            Some(signal - 1)
                        }
                    })
                    .is_ok();
                if acquired {
                    unsafe {
                        if let Some(poster) = self.notifiers.borrow_mut().pop_front() {
                            if let Some(poster) = poster.as_mut() {
                                poster.wakeup();
                            }
                        }
                    }
                } else {
                    let task = xworker.current();
                    self.waiters.borrow_mut().push_back(task);
                    task.block();
                }
                acquired
            });
            if acquired {
                break;
            }
            yield_now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Semaphore;
    use atomic_polyfill::Ordering;

    /// 回归：post 越界（bug #3）。
    /// 修复前用 `fetch_add(1) <= max`：旧值 == max 时仍 +1 越界，且失败分支不回滚导致计数泄漏。
    /// 修复后用带边界的 fetch_update：达到上限后 post 必须失败且计数保持在上限。
    #[test]
    fn post_isr_never_exceeds_max_and_does_not_leak() {
        let sem = Semaphore::with_signal_max_value(0, 2);
        assert!(sem.post_isr().is_ok()); // 1
        assert!(sem.post_isr().is_ok()); // 2
                                         // 第三次越界：应返回 SemaphoreFull 且计数不增长
        assert!(sem.post_isr().is_err());
        assert_eq!(sem.signal.load(Ordering::SeqCst), 2);
        // 再发仍失败且不泄漏（修复前每次失败都已先 +1，计数会一路涨上去）
        assert!(sem.post_isr().is_err());
        assert_eq!(sem.signal.load(Ordering::SeqCst), 2);
    }

    /// 回归：wait/post 配对后计数守恒。
    /// 走信号充足路径（不触发阻塞，因此不依赖 host 上的任务切换）。
    #[test]
    fn wait_then_post_balances() {
        let sem = Semaphore::with_signal_max_value(2, 3);
        sem.wait(); // 2 -> 1
        sem.wait(); // 1 -> 0
        assert_eq!(sem.signal.load(Ordering::SeqCst), 0);
        assert!(sem.post_isr().is_ok()); // 0 -> 1
        assert_eq!(sem.signal.load(Ordering::SeqCst), 1);
    }

    /// 回归：计数信号量语义——初始为 0 时 post 一次才能 wait 一次。
    #[test]
    fn counting_semantics() {
        let sem = Semaphore::with_signal(0);
        assert!(sem.post_isr().is_ok());
        assert_eq!(sem.signal.load(Ordering::SeqCst), 1);
        sem.wait();
        assert_eq!(sem.signal.load(Ordering::SeqCst), 0);
    }
}
