//! 一对多通知
//! 当有信号时会通知所有消费者

use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::TaskQueue;
use alloc::sync::Arc;
use core::cell::RefCell;

#[derive(Clone)]
pub struct Broadcast {
    waiters: Arc<RefCell<TaskQueue>>,
}

impl Broadcast {
    pub fn new() -> Self {
        Self {
            waiters: Arc::new(RefCell::new(TaskQueue::new())),
        }
    }
}
unsafe impl Send for Broadcast {}

impl Broadcast {
    /// 可以在中断服务里调用
    pub fn notify_isr(&self) -> nb::Result<(), nb::Error<()>> {
        unsafe {
            let mut have = false;
            loop {
                if let Some(waiter) = self.waiters.borrow_mut().pop_front() {
                    if let Some(waiter) = waiter.as_mut() {
                        waiter.wakeup();
                    }
                    have = true;
                } else if have {
                    return Ok(());
                } else {
                    return Err(nb::Error::WouldBlock);
                }
            }
        }
    }

    /// 不能在中断服务里调用
    pub fn notify(&self) {
        sync::free(|_| unsafe {
            loop {
                if let Some(waiter) = self.waiters.borrow_mut().pop_front() {
                    if let Some(waiter) = waiter.as_mut() {
                        waiter.wakeup();
                    }
                } else {
                    break;
                }
            }
        });
    }

    pub fn wait(&self) {
        sync::free(|_| {
            let task = xworker.current();
            self.waiters.borrow_mut().push_back(task);
            task.block();
        });
    }
}
