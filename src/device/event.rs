//! 定长事件通道：零分配环形缓冲 + 单等待者（v3 §5.8）。
//!
//! 与 [`sync::Queue`](crate::sync::Queue) / [`sync::Semaphore`](crate::sync::Semaphore)
//! 的区别：后两者靠 `Arc<RefCell<..>>` 堆分配，本通道用 `[MaybeUninit<T>; N]` +
//! 原子游标，可在无堆目标与 ISR 中使用。
//!
//! 语义：
//! - **单订阅**：同一时刻只有一个等待者；并发 `recv` 的第二个返回 `Busy`；
//! - **满时失败**：`post` 满 → `Err(Full)`，**不覆盖**已入队事件，`overflow_count` 计数；
//! - **关闭**：`close` 后 `recv`/`post` → `Cancelled`；
//! - **上下文**：`post`/`post_isr`/`try_recv` 可在 C1/C0 调用（内部只进短临界区）；
//!   `recv`/`recv_timeout` 仅任务上下文（会阻塞）。

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::AtomicBool;
use core::time::Duration;
// RMW 一律走 atomic-polyfill：riscv32imc（ch572/esp32c3）无 A 扩展，core 的 fetch_add/swap
// 在该目标上不存在；与 src/sync/arc.rs:2、src/sync/semaphore.rs:13 保持同一取法。
use atomic_polyfill::{AtomicUsize, Ordering};

use crate::device::DeviceError;
use crate::port::{Portable, Porting};
use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::task::{yield_now, Task};

/// 定长事件通道。
pub struct EventChannel<T, const N: usize> {
    ring: [UnsafeCell<MaybeUninit<T>>; N],
    /// 生产者游标（只增）
    head: AtomicUsize,
    /// 消费者游标（只增）
    tail: AtomicUsize,
    /// 满时失败计数
    overflow: AtomicUsize,
    closed: AtomicBool,
    /// 单等待者（任务指针地址值，0 = 无）
    waiter: AtomicUsize,
}

// SAFETY: 环形缓冲的读写全部在 `sync::free` 临界区内（单核关中断 / SMP 全局自旋），
// 生产者与消费者不会并发访问同一槽位；`T: Send` 保证跨上下文移动安全。
unsafe impl<T: Send, const N: usize> Send for EventChannel<T, N> {}
unsafe impl<T: Send, const N: usize> Sync for EventChannel<T, N> {}

impl<T, const N: usize> EventChannel<T, N> {
    /// 零分配构造（可 const）。
    pub const fn new() -> Self {
        Self {
            ring: [const { UnsafeCell::new(MaybeUninit::uninit()) }; N],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            overflow: AtomicUsize::new(0),
            closed: AtomicBool::new(false),
            waiter: AtomicUsize::new(0),
        }
    }

    /// 当前排队事件数
    pub fn len(&self) -> usize {
        sync::free(|_| {
            self.head
                .load(Ordering::SeqCst)
                .wrapping_sub(self.tail.load(Ordering::SeqCst))
        })
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 通道是否已关闭
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    /// 累计投递失败（满）次数
    pub fn overflow_count(&self) -> usize {
        self.overflow.load(Ordering::SeqCst)
    }

    /// 关闭通道：唤醒等待者；此后 `recv` → `Cancelled`，`post` → `Cancelled`。
    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        self.wake();
    }

    /// 投递一条事件（C1/C2，厂商回调线程）。满 → `Err(Full)`，不覆盖旧事件。
    pub fn post(&self, v: T) -> Result<(), DeviceError> {
        self.push(v)
    }

    /// 投递一条事件（C0，硬件 ISR）。与 [`post`](Self::post) 同语义；ISR 侧不重试、不记日志。
    pub fn post_isr(&self, v: T) -> Result<(), DeviceError> {
        self.push(v)
    }

    fn push(&self, v: T) -> Result<(), DeviceError> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(DeviceError::Cancelled);
        }
        let ok = sync::free(|_| {
            let head = self.head.load(Ordering::SeqCst);
            let tail = self.tail.load(Ordering::SeqCst);
            if head.wrapping_sub(tail) >= N {
                return false;
            }
            // SAFETY: 槽位空闲（head - tail < N），且全程在临界区内。
            unsafe { (*self.ring[head % N].get()).write(v) };
            self.head.store(head.wrapping_add(1), Ordering::SeqCst);
            true
        });
        if ok {
            self.wake();
            Ok(())
        } else {
            self.overflow.fetch_add(1, Ordering::SeqCst);
            Err(DeviceError::Full)
        }
    }

    /// 非阻塞取一条（C1）。空 → `None`。
    pub fn try_recv(&self) -> Option<T> {
        sync::free(|_| {
            let head = self.head.load(Ordering::SeqCst);
            let tail = self.tail.load(Ordering::SeqCst);
            if head == tail {
                return None;
            }
            // SAFETY: 槽位已初始化（tail < head），且全程在临界区内。
            let v = unsafe { (*self.ring[tail % N].get()).assume_init_read() };
            self.tail.store(tail.wrapping_add(1), Ordering::SeqCst);
            Some(v)
        })
    }

    /// 阻塞取一条（C2，仅任务上下文）。关闭 → `Cancelled`；已有等待者 → `Busy`。
    pub fn recv(&self) -> Result<T, DeviceError> {
        loop {
            if let Some(v) = self.try_recv() {
                return Ok(v);
            }
            // 复查 + 登记 + 挂起在同一临界区（丢失唤醒免疫，同 read_blocking）
            let ready = sync::free(|_| -> Result<bool, DeviceError> {
                if self.closed.load(Ordering::SeqCst) {
                    return Err(DeviceError::Cancelled);
                }
                let head = self.head.load(Ordering::SeqCst);
                let tail = self.tail.load(Ordering::SeqCst);
                if head != tail {
                    return Ok(true);
                }
                // 单等待者：已有等待者 → Busy
                if self.waiter.load(Ordering::SeqCst) != 0 {
                    return Err(DeviceError::Busy);
                }
                let t = xworker.current();
                let addr = (t as *mut Task).addr();
                self.waiter.store(addr, Ordering::SeqCst);
                t.block();
                Ok(false)
            })?;
            if ready {
                continue;
            }
            yield_now();
        }
    }

    /// 带超时取一条（C2）。超时 → `Timeout`；关闭 → `Cancelled`。
    pub fn recv_timeout(&self, timeout: Duration) -> Result<T, DeviceError> {
        let start = crate::time::tick_ms();
        let ms = timeout.as_millis() as u64;
        loop {
            if let Some(v) = self.try_recv() {
                return Ok(v);
            }
            if self.closed.load(Ordering::SeqCst) {
                return Err(DeviceError::Cancelled);
            }
            if crate::time::tick_ms().wrapping_sub(start) >= ms {
                return Err(DeviceError::Timeout);
            }
            crate::task::sleep_ms(1);
        }
    }

    /// 唤醒等待者（单等待者；无等待者时为空操作）。
    fn wake(&self) {
        let w = self.waiter.swap(0, Ordering::SeqCst);
        if w != 0 {
            // SAFETY: waiter 存的是 xworker.current() 的有效任务指针；任务注销前不会被释放。
            unsafe { &mut *(w as *mut Task) }.wakeup(); // 状态改回就绪
        }
        // 软中断唤醒内核调度器（同 EventDevice::wake 的纪律）
        Porting::irq();
    }
}

impl<T, const N: usize> Default for EventChannel<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_try_recv_fifo() {
        let ch: EventChannel<u8, 4> = EventChannel::new();
        assert!(ch.is_empty());
        ch.post(1).unwrap();
        ch.post(2).unwrap();
        assert_eq!(ch.len(), 2);
        assert_eq!(ch.try_recv(), Some(1));
        assert_eq!(ch.try_recv(), Some(2));
        assert_eq!(ch.try_recv(), None);
    }

    #[test]
    fn full_fails_without_overwrite() {
        let ch: EventChannel<u8, 2> = EventChannel::new();
        ch.post(1).unwrap();
        ch.post(2).unwrap();
        assert_eq!(ch.post(3), Err(DeviceError::Full));
        assert_eq!(ch.overflow_count(), 1);
        // 旧事件不被覆盖
        assert_eq!(ch.try_recv(), Some(1));
        assert_eq!(ch.try_recv(), Some(2));
    }

    #[test]
    fn close_is_terminal() {
        let ch: EventChannel<u8, 2> = EventChannel::new();
        ch.post(1).unwrap();
        ch.close();
        assert!(ch.is_closed());
        assert_eq!(ch.try_recv(), Some(1));
        assert_eq!(ch.post(2), Err(DeviceError::Cancelled));
    }

    #[test]
    fn const_construct() {
        static CH: EventChannel<u32, 8> = EventChannel::new();
        assert!(CH.is_empty());
    }
}
