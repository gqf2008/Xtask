//! 计数信号量，公平调度，多对多通知

use crate::sync::Error;
use crate::task::executor::{xworker, Executor};
use crate::task::Task;
use crate::{sprintln, TaskQueue};
use crate::{sync, yield_now};
use alloc::rc::Rc;
use core::cell::RefCell;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
// use crossbeam::atomic::AtomicCell;
/// 信号量
/// 设计思想是维护两个任务挂起队列
/// 当信号量为零时挂起当前任务到挂起队列
/// 当信号量大于零时从挂起队列弹出任务交给调度器
pub struct Semaphore {
    waiters: Rc<RefCell<TaskQueue>>,
    notifiers: Rc<RefCell<TaskQueue>>,
    signal: Rc<AtomicUsize>, //信号量
    max_value: usize,
}

impl Clone for Semaphore {
    fn clone(&self) -> Self {
        sync::free(|_| Self {
            waiters: self.waiters.clone(),
            notifiers: self.notifiers.clone(),
            signal: self.signal.clone(),
            max_value: self.max_value,
        })
    }
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
            waiters: Rc::new(RefCell::new(TaskQueue::new())),
            notifiers: Rc::new(RefCell::new(TaskQueue::new())),
            signal: Rc::new(AtomicUsize::new(signal)),
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
            #[cfg(not(target_has_atomic = "8"))]
            {
                let val = self.signal.load(Ordering::SeqCst);
                if val <= self.max_value {
                    self.signal.store(val + 1, Ordering::SeqCst);
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
            #[cfg(target_has_atomic = "8")]
            {
                if self.signal.fetch_add(1, Ordering::SeqCst) <= self.max_value {
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
    }
    /// 发送信号
    /// 不能在中断服务中使用
    pub fn post(&self) {
        loop {
            #[cfg(not(target_has_atomic = "8"))]
            {
                match sync::free(|_| unsafe {
                    let val = self.signal.load(Ordering::SeqCst);
                    if val <= self.max_value {
                        self.signal.store(val + 1, Ordering::SeqCst);
                        if let Some(waiter) = self.waiters.borrow_mut().pop_front() {
                            if let Some(waiter) = waiter.as_mut() {
                                waiter.wakeup();
                            }
                        }
                        true
                    } else {
                        let task = xworker.current();
                        self.notifiers.borrow_mut().push_back(task as *mut Task);
                        task.block();
                        false
                    }
                }) {
                    true => break,
                    false => yield_now(),
                }
            }
            #[cfg(target_has_atomic = "8")]
            {
                if self.signal.fetch_add(1, Ordering::SeqCst) <= self.max_value {
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
    }

    /// 等待一个信号量
    /// 禁止在中断服务中调用
    /// 注意：不要同时使用post_isr和post，不然可能会错误的唤醒poster
    pub fn wait(&self) {
        loop {
            #[cfg(not(target_has_atomic = "8"))]
            {
                match sync::free(|_| unsafe {
                    let val = self.signal.load(Ordering::SeqCst);
                    if val == 0 {
                        let task = xworker.current();
                        self.waiters.borrow_mut().push_back(task);

                        task.block();
                        false
                    } else {
                        self.signal.store(val + 1, Ordering::SeqCst);
                        if let Some(poster) = self.notifiers.borrow_mut().pop_front() {
                            if let Some(poster) = poster.as_mut() {
                                poster.wakeup();
                            }
                        }
                        true
                    }
                }) {
                    true => break,
                    false => yield_now(),
                }
            }

            #[cfg(target_has_atomic = "8")]
            {
                match self
                    .signal
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |signal| {
                        if signal == 0 {
                            None
                        } else {
                            Some(signal - 1)
                        }
                    }) {
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
}
