//! 一对多通知
//! 当有信号时会通知所有消费者

use crate::task::executor::{xworker, Executor};
use crate::TaskQueue;
use crate::{sync, yield_now};
use alloc::rc::Rc;
use core::cell::RefCell;

pub struct Broadcast {
    waiters: Rc<RefCell<TaskQueue>>,
}

impl Clone for Broadcast {
    fn clone(&self) -> Self {
        sync::free(|_| self.clone())
    }
}

unsafe impl Send for Broadcast {}

impl Broadcast {
    pub fn new() -> Self {
        Self {
            waiters: Rc::new(RefCell::new(TaskQueue::new())),
        }
    }
}

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

        yield_now();
    }
}
