//! 一对一通知

use crate::sync::Error;
use crate::task::executor::{xworker, Executor};
use crate::task::Task;
use crate::{sync, yield_now};
// use alloc::sync::Arc;
// use core::sync::atomic::AtomicBool;
// use core::sync::atomic::Ordering;
use super::arc::Arc;
use atomic_polyfill::{AtomicBool, Ordering};
#[cfg(target_has_atomic = "ptr")]
use crossbeam::atomic::AtomicCell;
#[cfg(not(target_has_atomic = "ptr"))]
use crate::sync::atomic_cell::AtomicCell;

#[derive(Clone)]
pub struct Notifier {
    blocker: Arc<AtomicCell<usize>>, //当前挂起者任务指针
    signal: Arc<AtomicBool>, //信号标记，智能指针包下，防止move过程中地址里的值被转移到其他任务栈
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            blocker: Arc::new(AtomicCell::new(0)),
            signal: Arc::new(AtomicBool::new(false)),
        }
    }
}

unsafe impl Send for Notifier {}

impl Notifier {
    #[inline]
    unsafe fn block(&self) {
        let task = xworker.current();
        let addr = (task as *mut Task).addr();
        self.blocker.store(addr);
        task.block();
    }

    #[inline]
    unsafe fn wakeup(&self) {
        if let Ok(ptr) = self.blocker.fetch_update(|_ptr| Some(0)) {
            if ptr > 0 {
                let blocker = &mut *(ptr as *mut Task);

                blocker.wakeup();
            }
        }
    }

    /// 产生一个信号，如果信号写入
    /// 成功则唤醒挂起的任务否则报错
    pub fn notify_isr(&self) -> nb::Result<(), Error> {
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

    /// 产生一个信号，如果信号写入
    /// 成功则唤醒挂起的任务，如果
    /// 信号写入失败则挂起自己
    pub fn notify(&self) {
        loop {
            // "尝试置位信号 + 失败则登记挂起"必须在同一临界区（与信号量同款纪律）：
            // 修前 CAS 在临界区外，ISR 恰在 CAS 成功之后、本任务登记 blocker 之前
            // 调用 notify_isr，会因 blocker 仍为 0 而无人可唤——信号被吞、读者永眠
            //（遗留问题 #6，2026-08-22 修复）。临界区内 ISR 无法插进窗口。
            let delivered = sync::free(|_| unsafe {
                if self
                    .signal
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    self.wakeup();
                    true
                } else {
                    self.block();
                    false
                }
            });
            if delivered {
                break;
            }
            yield_now();
        }
    }

    /// 等待一个信号
    /// 如果有信号则唤醒通知者，否则挂起自己
    pub fn wait(&self) {
        loop {
            // 与 notify 同款："尝试取走信号 + 失败则登记挂起"在同一临界区，
            // ISR 的 notify_isr 不可能插在两者之间——不会出现"信号发给了
            // 还没登记的人"（比"信号已置位却无人消费"更隐蔽的另一半窗口）。
            let got = sync::free(|_cs| unsafe {
                if self
                    .signal
                    .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    self.wakeup();
                    true
                } else {
                    self.block();
                    false
                }
            });
            if got {
                break;
            }
            yield_now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 回归：信号先到（notify_isr）、wait 后到——wait 走"取走信号"路径不挂起，
    /// 并保证信号被消费（二次 notify_isr 回到可置位状态）。
    /// 修复(#6)前后该路径行为一致，它锁死的是 CAS 取值/消费语义不被改坏；
    /// 丢失唤醒窗口的消除是构造性的（检查+登记+挂起同一临界区，ISR 无法插入），
    /// 单线程 host 复现不了 ISR 时序——真机压测项见第 20 章踩坑记录 2。
    #[test]
    fn wait_consumes_preposted_signal() {
        let n = Notifier::new();
        assert!(n.notify_isr().is_ok(), "预置信号应成功");
        n.wait(); // 不挂起、直接取走
        assert!(n.notify_isr().is_ok(), "消费后信号归零,可再次置位(修前若 wait 没消费则第二次返回 WouldBlock)");
    }
}
