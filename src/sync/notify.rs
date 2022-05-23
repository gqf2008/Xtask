//! 一对一通知

use crate::sync::Error;
use crate::task::executor::{xworker, Executor};
use crate::task::Task;
use crate::{sync, yield_now};
use alloc::rc::Rc;
// use alloc::sync::Arc;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
// use crossbeam::atomic::AtomicCell;

pub struct Notifier {
    blocker: Rc<usize>,     //当前挂起者任务指针
    signal: Rc<AtomicBool>, //信号标记，智能指针包下，防止move过程中地址里的值被转移到其他任务栈
}

impl Clone for Notifier {
    fn clone(&self) -> Self {
        sync::free(|_| Self {
            blocker: self.blocker.clone(),
            signal: self.signal.clone(),
        })
    }
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            blocker: Rc::new(0),
            signal: Rc::new(AtomicBool::new(false)),
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
        #[cfg(not(target_has_atomic = "8"))]
        {
            if !self.signal.load(Ordering::SeqCst) {
                self.signal.store(true, Ordering::SeqCst);
                unsafe { self.wakeup() };
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }
        #[cfg(target_has_atomic = "8")]
        {
            match self
                .signal
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            {
                Ok(_) => unsafe {
                    self.wakeup();
                    Ok(())
                },
                Err(_) => Err(nb::Error::WouldBlock),
            }
        }
    }

    /// 产生一个信号，如果信号写入
    /// 成功则唤醒挂起的任务，如果
    /// 信号写入失败则挂起自己
    pub fn notify(&self) {
        loop {
            #[cfg(not(target_has_atomic = "8"))]
            {
                match sync::free(|_| unsafe {
                    if !self.signal.load(Ordering::SeqCst) {
                        self.signal.store(true, Ordering::SeqCst);
                        self.wakeup();
                        true
                    } else {
                        self.block();
                        false
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
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                {
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
    }

    /// 等待一个信号
    /// 如果有信号则唤醒通知者，否则挂起自己
    pub fn wait(&self) {
        loop {
            #[cfg(not(target_has_atomic = "8"))]
            {
                match sync::free(|_| unsafe {
                    if self.signal.load(Ordering::SeqCst) {
                        self.signal.store(false, Ordering::SeqCst);
                        self.wakeup();
                        true
                    } else {
                        self.block();
                        false
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
                    .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
                {
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
}
