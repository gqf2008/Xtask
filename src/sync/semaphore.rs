//! 计数信号量，公平调度，多对多通知

use crate::sync::Error;
use crate::task::executor::{xworker, Executor};
use crate::task::Task;
use crate::TaskQueue;
use crate::{sync, yield_now};
use alloc::sync::Arc;
use core::cell::RefCell;
use crossbeam::atomic::AtomicCell;

/// 信号量
/// 设计思想是维护两个任务挂起队列
/// 当信号量为零时挂起当前任务到挂起队列
/// 当信号量大于零时从挂起队列弹出任务交给调度器
#[derive(Clone)]
pub struct Semaphore {
    waiters: Arc<RefCell<TaskQueue>>,
    notifiers: Arc<RefCell<TaskQueue>>,
    signal: Arc<AtomicCell<u64>>, //信号量
    max_value: u64,
}

unsafe impl Send for Semaphore {}
impl Semaphore {
    pub fn new() -> Self {
        Self::with_signal(0)
    }

    pub fn with_signal(signal: u64) -> Self {
        Self::with_signal_max_value(signal, u64::MAX)
    }

    pub fn with_max_value(max_value: u64) -> Self {
        Self::with_signal_max_value(0, max_value)
    }

    pub fn with_signal_max_value(signal: u64, max_value: u64) -> Self {
        Self {
            waiters: Arc::new(RefCell::new(TaskQueue::new())),
            notifiers: Arc::new(RefCell::new(TaskQueue::new())),
            signal: Arc::new(AtomicCell::new(signal)),
            max_value: max_value,
        }
    }
}

impl Semaphore {
    /// 发送信号
    /// 可以在中断服务中使用
    ///
    pub fn post_isr(&self) -> nb::Result<(), Error> {
        loop {
            if self.signal.fetch_add(1) <= self.max_value {
                unsafe {
                    if let Some(waiter) = self.waiters.borrow_mut().pop_front() {
                        if let Some(waiter) = waiter.as_mut() {
                            waiter.wakeup();
                        }
                    }
                };
            } else {
                return Err(nb::Error::Other(Error::SemaphoreFull));
            }
        }
    }
    /// 发送信号
    /// 不能在中断服务中使用
    pub fn post(&self) {
        loop {
            if self.signal.fetch_add(1) <= self.max_value {
                sync::free(|_| unsafe {
                    if let Some(waiter) = self.waiters.borrow_mut().pop_front() {
                        if let Some(waiter) = waiter.as_mut() {
                            waiter.wakeup();
                        }
                    }
                });
                break;
            } else {
                sync::free(|_| {
                    let task = xworker.current();
                    self.notifiers.borrow_mut().push_back(task as *mut Task);
                    task.block();
                });
                yield_now();
            }
        }
    }

    /// 等待一个信号量
    /// 禁止在中断服务中调用
    /// 注意：不要同时使用post_isr和post，不然可能会错误的唤醒poster
    pub fn wait(&self) {
        loop {
            match self.signal.fetch_update(
                |signal| {
                    if signal == 0 {
                        None
                    } else {
                        Some(signal - 1)
                    }
                },
            ) {
                Ok(_) => {
                    sync::free(|_| unsafe {
                        if let Some(poster) = self.notifiers.borrow_mut().pop_front() {
                            if let Some(poster) = poster.as_mut() {
                                poster.wakeup();
                            }
                        }
                    });
                    break;
                }
                Err(_) => {
                    sync::free(|_| {
                        let task = xworker.current();
                        self.waiters.borrow_mut().push_back(task);

                        task.block();
                    });
                    yield_now();
                }
            }
        }
    }
}
