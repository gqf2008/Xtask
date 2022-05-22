//! 一对一通知

use crate::sync::Error;
use crate::task::executor::{xworker, Executor};
use crate::task::Task;
use crate::{sync, yield_now};
use alloc::sync::Arc;
use crossbeam::atomic::AtomicCell;

#[derive(Clone)]
pub struct Notifier {
    blocker: Arc<usize>,           //当前挂起者任务指针
    signal: Arc<AtomicCell<bool>>, //信号标记，智能指针包下，防止move过程中地址里的值被转移到其他任务栈
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            blocker: Arc::new(0),
            signal: Arc::new(AtomicCell::new(false)),
        }
    }
}

unsafe impl Send for Notifier {}

impl Notifier {
    #[inline]
    unsafe fn block(&self) {
        let task = xworker.current();
        let addr = (task as *mut Task).addr();
        core::ptr::write_volatile(self.blocker.as_ref() as *const _ as *mut usize, addr);
        task.block();
    }

    #[inline]
    unsafe fn wakeup(&self) {
        let blocker = core::ptr::read_volatile(self.blocker.as_ref());
        if blocker != 0 {
            let blocker = &mut *(blocker as *mut Task);
            core::ptr::write_volatile(self.blocker.as_ref() as *const _ as *mut usize, 0);
            blocker.wakeup();
        }
    }

    /// 产生一个信号，如果信号写入
    /// 成功则唤醒挂起的任务否则报错
    pub fn notify_isr(&self) -> nb::Result<(), Error> {
        match self.signal.compare_exchange(false, true) {
            Ok(_) => unsafe {
                self.wakeup();
                Ok(())
            },
            Err(_) => Err(nb::Error::WouldBlock),
        }
    }

    /// 产生一个信号，如果信号写入
    /// 成功则唤醒挂起的任务，如果
    /// 信号写入失败则挂起自己
    pub fn notify(&self) {
        loop {
            match self.signal.compare_exchange(false, true) {
                Ok(_) => {
                    sync::free(|_| unsafe {
                        self.wakeup();
                    });
                    break;
                }
                Err(_) => {
                    sync::free(|_| unsafe {
                        self.block();
                    });
                    yield_now();
                }
            }
        }
    }

    /// 等待一个信号
    /// 如果有信号则唤醒通知者，否则挂起自己
    pub fn wait(&self) {
        loop {
            match self.signal.compare_exchange(true, false) {
                Ok(_) => {
                    sync::free(|_cs| unsafe {
                        self.wakeup();
                    });
                    break;
                }
                Err(_) => {
                    sync::free(|_cs| unsafe {
                        self.block();
                    });
                    yield_now();
                }
            }
        }
    }
}
