//! 一对多通知
//! 当有信号时会通知所有消费者

use crate::task::executor::{xworker, Executor};
use crate::TaskQueue;
use crate::{sync, yield_now};
// use alloc::sync::Arc;
use super::arc::Arc;
use core::cell::RefCell;

#[derive(Clone)]
pub struct Broadcast {
    waiters: Arc<RefCell<TaskQueue>>,
}

unsafe impl Send for Broadcast {}

impl Broadcast {
    pub fn new() -> Self {
        Self {
            waiters: Arc::new(RefCell::new(TaskQueue::new())),
        }
    }
}

impl Broadcast {
    /// 可以在中断服务里调用
    /// SMP(ch25 ⑥):ISR 侧借用同样进全局锁——wait/notify 的 borrow_mut
    /// 全在 sync::free 内,ISR 裸借用会与别核并发(RefCell 借位标志非原子,
    /// 并发借用即 panic/UB)。trap 上下文持锁安全:同核持区者中断已关,
    /// 别核持区者短临界区有界自旋即得
    pub fn notify_isr(&self) -> nb::Result<(), nb::Error<()>> {
        sync::free(|_| unsafe {
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
        })
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
