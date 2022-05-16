//! 计数信号量，公平调度，实现了mpmc多生产者多消费者模型

use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::task::Task;
use crate::TaskQueue;
use alloc::sync::Arc;
use bare_metal::Mutex;
use core::cell::RefCell;
use crossbeam::atomic::AtomicCell;

/// 信号量
/// 设计思想是维护两个任务挂起队列
/// 当信号量为零时挂起当前任务到挂起队列
/// 当信号量大于零时从挂起队列弹出任务交给调度器
#[derive(Clone)]
pub struct Semaphore {
    waiters: Arc<Mutex<RefCell<TaskQueue>>>,
    notifiers: Arc<Mutex<RefCell<TaskQueue>>>,
    signal: Arc<AtomicCell<u64>>, //信号量
}

unsafe impl Send for Semaphore {}
impl Semaphore {
    pub fn new() -> Self {
        Self::with_value(0)
    }

    pub fn with_value(value: u64) -> Self {
        Self {
            waiters: Arc::new(Mutex::new(RefCell::new(TaskQueue::new()))),
            notifiers: Arc::new(Mutex::new(RefCell::new(TaskQueue::new()))),
            signal: Arc::new(AtomicCell::new(value)),
        }
    }
}

impl Semaphore {
    pub fn post(&self) {
        loop {
            if self.signal.fetch_add(1) == u64::MAX {
                sync::free(|cs| {
                    let task = xworker.current();
                    self.notifiers
                        .borrow(*cs)
                        .borrow_mut()
                        .push_back(task as *mut Task);
                    task.block();
                });
            } else {
                sync::free(|cs| unsafe {
                    if let Some(waiter) = self.waiters.borrow(*cs).borrow_mut().pop_front() {
                        if let Some(waiter) = waiter.as_mut() {
                            waiter.wakeup();
                        }
                    }
                });
                break;
            }
        }
    }

    /// 等待一个信号量
    /// 禁止在中断服务中调用
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
                    sync::free(|cs| unsafe {
                        if let Some(poster) = self.notifiers.borrow(*cs).borrow_mut().pop_front() {
                            if let Some(poster) = poster.as_mut() {
                                poster.wakeup();
                            }
                        }
                    });
                    break;
                }
                Err(_) => {
                    sync::free(|cs| {
                        let task = xworker.current();
                        self.waiters.borrow(*cs).borrow_mut().push_back(task);

                        task.block();
                    });
                }
            }
        }
    }
}
