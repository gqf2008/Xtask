//! 实现了spmc单生产者多消费者模型
//! 当有信号时会通知所有消费者

use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::TaskQueue;
use alloc::sync::Arc;
use bare_metal::Mutex;
use core::cell::RefCell;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Timeout,
}

#[derive(Clone)]
pub struct Broadcast {
    waiters: Arc<Mutex<RefCell<TaskQueue>>>,
}

impl Broadcast {
    pub fn new() -> Self {
        Self {
            waiters: Arc::new(Mutex::new(RefCell::new(TaskQueue::new()))),
        }
    }
}
unsafe impl Send for Broadcast {}

impl Broadcast {
    pub fn notify(&self) {
        sync::free(|cs| unsafe {
            loop {
                if let Some(waiter) = self.waiters.borrow(*cs).borrow_mut().pop_front() {
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
        sync::free(|cs| {
            let task = xworker.current();
            self.waiters.borrow(*cs).borrow_mut().push_back(task);
            task.block();
        });
    }
}
