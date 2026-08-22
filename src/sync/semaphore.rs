//! 计数信号量，公平调度，多对多通知

use crate::sync::Error;
use crate::task::executor::{xworker, Executor};
use crate::task::Task;
use crate::TaskQueue;
use crate::{sync, yield_now};
// use alloc::sync::Arc;
use super::arc::Arc;
use core::cell::RefCell;
// use core::sync::atomic::AtomicUsize;
// use core::sync::atomic::Ordering;
use atomic_polyfill::{AtomicUsize, Ordering};
// use crossbeam::atomic::AtomicCell;

/// 信号量
/// 设计思想是维护两个任务挂起队列
/// 当信号量为零时挂起当前任务到挂起队列
/// 当信号量大于零时从挂起队列弹出任务交给调度器
#[derive(Clone)]
pub struct Semaphore {
    waiters: Arc<RefCell<TaskQueue>>,
    notifiers: Arc<RefCell<TaskQueue>>,
    signal: Arc<AtomicUsize>, //信号量
    max_value: usize,
}

unsafe impl Send for Semaphore {}

// SAFETY: 与 Send 同构——waiters/notifiers 两个队列的访问全部发生在 sync::free
// 临界区内（单核关中断，任务侧与 ISR 侧不可能并发借用同一 RefCell），post_isr
// 只碰原子计数与 wakeup（不动借用）；共享引用 &Semaphore 经"临界区串行化"后
// 的可变访问是单核安全模型的既定纪律（与 bus.rs/REGISTRY 同构）。
// 现状必要：Mutex<T> 要能进 static（OnceCell<Mutex<T>> 要求 T: Sync）。
unsafe impl Sync for Semaphore {}

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
            waiters: Arc::new(RefCell::new(TaskQueue::new())),
            notifiers: Arc::new(RefCell::new(TaskQueue::new())),
            signal: Arc::new(AtomicUsize::new(signal)),
            max_value: max_value,
        }
    }
}

impl Semaphore {
    /// 发送信号
    /// 可以在中断服务中使用
    ///
    pub fn post_isr(&self) -> nb::Result<(), Error> {
        //带边界的原子自增：仅当未达上限时才 +1，避免越界和失败泄漏计数
        match self
            .signal
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |signal| {
                if signal >= self.max_value {
                    None
                } else {
                    Some(signal + 1)
                }
            }) {
            Ok(_) => {
                unsafe {
                    if let Some(waiter) = self.waiters.borrow_mut().pop_front() {
                        if let Some(waiter) = waiter.as_mut() {
                            waiter.wakeup();
                        }
                    }
                };
                Ok(())
            }
            Err(_) => Err(nb::Error::Other(Error::SemaphoreFull)),
        }
    }
    /// 发送信号
    /// 不能在中断服务中使用
    pub fn post(&self) {
        loop {
            //"尝试计数 + 失败则入队挂起"必须在同一个临界区内完成，
            //否则两者之间存在窗口：对方在此时完成 wait 弹出 notifiers 时队列还是空的，
            //本任务随后入队挂起，信号量已有空余容量却无人唤醒（丢失唤醒）。
            //带边界的原子自增：仅当未达上限时才 +1，避免越界和失败泄漏计数
            let posted = sync::free(|_| {
                let posted = self
                    .signal
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |signal| {
                        if signal >= self.max_value {
                            None
                        } else {
                            Some(signal + 1)
                        }
                    })
                    .is_ok();
                if posted {
                    unsafe {
                        if let Some(waiter) = self.waiters.borrow_mut().pop_front() {
                            if let Some(waiter) = waiter.as_mut() {
                                waiter.wakeup();
                            }
                        }
                    }
                } else {
                    let task = xworker.current();
                    self.notifiers.borrow_mut().push_back(task as *mut Task);
                    task.block();
                }
                posted
            });
            if posted {
                break;
            }
            yield_now();
        }
    }

    /// 将信号量计数重置为指定值（仅在 sync::free 临界区内调用）。
    /// 供"清空/截断"类语义使用：Queue 维护"sem 计数 == 元素数"不变量，
    /// 清空后计数必须同步归零，否则残留计数会在空列表上放行 wait()，
    /// 消费方拿到幽灵出队且计数从此漂移（修前遗留问题 #7）。
    #[inline]
    pub(crate) fn reset_signal(&self, count: usize) {
        let count = count.min(self.max_value);
        self.signal.store(count, Ordering::SeqCst);
    }

    /// 测试专用：读当前计数（host 回归需要观察 clear/trancate 后的不变量）
    #[cfg(test)]
    pub(crate) fn signal_count(&self) -> usize {
        self.signal.load(Ordering::SeqCst)
    }

    /// 非阻塞取一个信号：拿到返回 true（计数 N→N-1），并照 wait 的规则
    /// 唤醒一个因 post 满而排队的 poster；拿不到返回 false，**不挂起**。
    /// 供"尝试型"原语（如 Mutex::try_lock）与宿主测试使用——host 无调度器，
    /// wait 的空信号路径会挂死，只有这条路径可被测。
    pub fn try_wait(&self) -> bool {
        sync::free(|_| {
            let acquired = self
                .signal
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |signal| {
                    if signal == 0 {
                        None
                    } else {
                        Some(signal - 1)
                    }
                })
                .is_ok();
            if acquired {
                unsafe {
                    if let Some(poster) = self.notifiers.borrow_mut().pop_front() {
                        if let Some(poster) = poster.as_mut() {
                            poster.wakeup();
                        }
                    }
                }
            }
            acquired
        })
    }

    /// 等待一个信号量
    /// 禁止在中断服务中调用
    /// 注意：不要同时使用post_isr和post，不然可能会错误的唤醒poster
    pub fn wait(&self) {
        loop {
            //与 post 同理："尝试取信号 + 失败则入队挂起"必须在同一个临界区内完成，
            //否则对方在此窗口内 post 会发现 waiters 为空，本任务挂起后无人唤醒（丢失唤醒）。
            let acquired = sync::free(|_| {
                let acquired = self
                    .signal
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |signal| {
                        if signal == 0 {
                            None
                        } else {
                            Some(signal - 1)
                        }
                    })
                    .is_ok();
                if acquired {
                    unsafe {
                        if let Some(poster) = self.notifiers.borrow_mut().pop_front() {
                            if let Some(poster) = poster.as_mut() {
                                poster.wakeup();
                            }
                        }
                    }
                } else {
                    let task = xworker.current();
                    self.waiters.borrow_mut().push_back(task);
                    task.block();
                }
                acquired
            });
            if acquired {
                break;
            }
            yield_now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Semaphore;
    use atomic_polyfill::Ordering;

    /// 回归：post 越界（bug #3）。
    /// 修复前用 `fetch_add(1) <= max`：旧值 == max 时仍 +1 越界，且失败分支不回滚导致计数泄漏。
    /// 修复后用带边界的 fetch_update：达到上限后 post 必须失败且计数保持在上限。
    #[test]
    fn post_isr_never_exceeds_max_and_does_not_leak() {
        let sem = Semaphore::with_signal_max_value(0, 2);
        assert!(sem.post_isr().is_ok()); // 1
        assert!(sem.post_isr().is_ok()); // 2
                                         // 第三次越界：应返回 SemaphoreFull 且计数不增长
        assert!(sem.post_isr().is_err());
        assert_eq!(sem.signal.load(Ordering::SeqCst), 2);
        // 再发仍失败且不泄漏（修复前每次失败都已先 +1，计数会一路涨上去）
        assert!(sem.post_isr().is_err());
        assert_eq!(sem.signal.load(Ordering::SeqCst), 2);
    }

    /// 回归：wait/post 配对后计数守恒。
    /// 走信号充足路径（不触发阻塞，因此不依赖 host 上的任务切换）。
    #[test]
    fn wait_then_post_balances() {
        let sem = Semaphore::with_signal_max_value(2, 3);
        sem.wait(); // 2 -> 1
        sem.wait(); // 1 -> 0
        assert_eq!(sem.signal.load(Ordering::SeqCst), 0);
        assert!(sem.post_isr().is_ok()); // 0 -> 1
        assert_eq!(sem.signal.load(Ordering::SeqCst), 1);
    }

    /// 回归：计数信号量语义——初始为 0 时 post 一次才能 wait 一次。
    #[test]
    fn counting_semantics() {
        let sem = Semaphore::with_signal(0);
        assert!(sem.post_isr().is_ok());
        assert_eq!(sem.signal.load(Ordering::SeqCst), 1);
        sem.wait();
        assert_eq!(sem.signal.load(Ordering::SeqCst), 0);
    }

    /// 回归：try_wait 非阻塞路径——信号足则取走计数、不足则返回 false 且计数不变。
    /// 全部可宿主测（不触发阻塞，因此不依赖 host 上的任务切换）。
    #[test]
    fn try_wait_balances() {
        let sem = Semaphore::with_signal(1);
        assert!(sem.try_wait(), "有 1 个信号应取到");
        assert_eq!(sem.signal.load(Ordering::SeqCst), 0);
        assert!(!sem.try_wait(), "信号为 0 应取不到");
        assert_eq!(sem.signal.load(Ordering::SeqCst), 0, "失败路径不得改计数");
        assert!(sem.post_isr().is_ok());
        assert!(sem.try_wait(), "post 后应能再取");
    }
}
