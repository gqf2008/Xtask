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
use core::time::Duration;
// 原子一律走 atomic-polyfill：riscv32imc（ch572/esp32c3）无 A 扩展，core 的 fetch_add/swap
// 在该目标上不存在；`AtomicBool` 现在只做 load/store 没事，但一旦改成 CAS 式（close 的
// 实现演进方向）就必须走 polyfill——统一取法，与 src/sync/arc.rs:2、src/sync/notify.rs:11 一致。
use atomic_polyfill::{AtomicBool, AtomicUsize, Ordering};

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
        // N = 0 是退化通道：push 的 `head - tail >= N` 先命中，永远 `Err(Full)`。
        // 拦在编译期，别留成运行期静默失效。
        const { assert!(N > 0, "EventChannel<N>: N 必须 >= 1") };
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
        // `closed.store` 与 `waiter.swap` 必须在**同一临界区**：否则 SMP 下 `recv` 的
        // 「复查 closed → 判 waiter 空 → 登记 → 挂起」可插进这两步之间——close 先走完
        // （swap 拿到 0、没人可唤醒），recv 随后登记并挂起，此后无人再清槽、也无事件可等
        // → 任务永久 Suspended，且整条通道此后 recv 一律 Busy、无 API 可复位。
        let w = sync::free(|_| {
            self.closed.store(true, Ordering::SeqCst);
            self.waiter.swap(0, Ordering::SeqCst)
        });
        self.wake_waiting(w);
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
        // closed 的复查必须在临界区内（单核亦然：close 可能来自 ISR）——否则
        // 「读到 false」与「写槽」之间被 close 插进，会往已关闭通道里塞一条永远无人
        // 再取的事件（recv 已按 Cancelled 退出，且它不会再被唤醒）。
        // 同时把「唤醒谁」也在同区内取走：与 recv 的登记同区，才不会出现
        // 「recv 登记完发现没数据、而 post 的唤醒已经放完空枪」的丢唤醒。
        let outcome = sync::free(|_| {
            if self.closed.load(Ordering::SeqCst) {
                return Err(DeviceError::Cancelled);
            }
            let head = self.head.load(Ordering::SeqCst);
            let tail = self.tail.load(Ordering::SeqCst);
            if head.wrapping_sub(tail) >= N {
                return Err(DeviceError::Full);
            }
            // SAFETY: 槽位空闲（head - tail < N），且全程在临界区内。
            unsafe { (*self.ring[head % N].get()).write(v) };
            self.head.store(head.wrapping_add(1), Ordering::SeqCst);
            Ok(self.waiter.swap(0, Ordering::SeqCst))
        });
        match outcome {
            Ok(w) => {
                self.wake_waiting(w);
                Ok(())
            }
            Err(DeviceError::Full) => {
                self.overflow.fetch_add(1, Ordering::SeqCst);
                Err(DeviceError::Full)
            }
            Err(e) => Err(e),
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
    /// 入口断言调用方不在 `sync::free` 内（R8）。
    pub fn recv(&self) -> Result<T, DeviceError> {
        debug_assert!(!sync::in_critical_section(), "C2 in sync::free");
        // 本任务在登记表里的值（只由自己/`wake_waiting` 清除）
        let me = (xworker.current() as *mut Task).addr();
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
                self.waiter.store(me, Ordering::SeqCst);
                t.block();
                Ok(false)
            })?;
            if ready {
                continue;
            }
            yield_now();
            // 自愈：无论谁把我弄醒，回到任务上下文后幂等地清掉「我」这份登记。
            // 否则任何非 `wake_waiting` 的退出（将来加取消/任务注销、或唤醒被谁抢走）
            // 都会让 waiter 永久非 0 → 此后整条通道 recv 一律 Busy 且无 API 可复位。
            let _ = self
                .waiter
                .compare_exchange(me, 0, Ordering::SeqCst, Ordering::SeqCst);
        }
    }

    /// 带超时取一条（C2）。超时 → `Timeout`；关闭 → `Cancelled`。
    /// 入口断言调用方不在 `sync::free` 内（R8）。
    ///
    /// ⚠ 实现是**分片轮询**（片长 ≤ 1 tick），不是真阻塞：`Task::wakeup` 只对
    /// `State::Suspended` 生效，而 `sleep_ms` 走的是 `State::Blocked` + 延时队列
    /// （task.rs 两个不同的挂起通道），所以内核目前**没有**「延时 + IPC 唤醒」双登记
    /// 的 timed-block 原语。可见差异：事件到达的唤醒延迟 ≤ 1 tick（非立即）、
    /// 超时精度 = tick。期间仍占着单订阅登记位（`WaiterGuard`），所以并发
    /// `recv`/`recv_timeout` 照常 `Busy`。待内核补 timed-block 后改真阻塞（v3 §10.6 phase2）。
    pub fn recv_timeout(&self, timeout: Duration) -> Result<T, DeviceError> {
        debug_assert!(!sync::in_critical_section(), "C2 in sync::free");
        // 亚毫秒超时按 1 tick 计：tick 是本通道的时间精度上限；截断为 0 会
        // 静默退化成「不等待」（`elapsed >= 0` 恒真 → 一次 try_recv 失败就 Timeout）。
        let ms = (timeout.as_millis() as u64).max(1);
        let _guard = WaiterGuard::acquire(self)?;
        let start = crate::time::tick_ms();
        loop {
            // 事件优先：超时到点与事件到达同时发生时，不能报 Timeout 丢掉已到的事件
            if let Some(v) = self.try_recv() {
                return Ok(v);
            }
            if self.closed.load(Ordering::SeqCst) {
                return Err(DeviceError::Cancelled);
            }
            let elapsed = crate::time::tick_ms().wrapping_sub(start);
            if elapsed >= ms {
                return Err(DeviceError::Timeout);
            }
            // 分片睡觉：片长 ≤ 1 tick（多睡一片会成倍拉长唤醒延迟）
            crate::task::sleep_ms((ms - elapsed).min(1) as usize);
        }
    }

    /// 唤醒在登记表里等的那一个任务（`w = 0` 表示无等待者）。
    ///
    /// 必须在**取走登记之后、临界区之外**调用：`Task::wakeup` 内部自带临界区
    /// （task.rs 的 F1 修复），且此刻任务必为 `Suspended` 不会自醒，
    /// 所以「取登记」与「改状态」分两步不会丢唤醒。
    fn wake_waiting(&self, w: usize) {
        if w == 0 {
            return;
        }
        // SAFETY: waiter 存的是 `xworker.current()` 的有效任务指针；任务注销前不会被释放。
        unsafe { &mut *(w as *mut Task) }.wakeup(); // 状态改回就绪
        // 只在真有任务被唤醒时才踢调度器：无等待者时每次 post 都拉一次软中断是纯开销
        Porting::irq();
    }
}

impl<T, const N: usize> Default for EventChannel<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for EventChannel<T, N> {
    fn drop(&mut self) {
        // 排空：未被取走的事件由这里析构。`T` 未约束 `Copy`（必须，否则无法装非 POD
        // 事件），因此缺了这个 Drop，`EventChannel<Vec<u8>, 8>` 这类实例就是纯泄漏。
        // `close()` 之后仍允许 `try_recv` 排空，所以也走同一条路径。
        while self.try_recv().is_some() {}
        // 残留登记作废：实例已在注销路上，留着只会让别人对着旧地址 `wakeup`
        self.waiter.store(0, Ordering::SeqCst);
    }
}

/// `recv_timeout` 期间占住单订阅登记位（RAII 释放）。
///
/// 它**不负责唤醒**：`recv_timeout` 走 `sleep_ms`，任务处于 `State::Blocked`，
/// `Task::wakeup` 不会命中（见 `recv_timeout` 的 ⚠ 说明）；它的唯一作用是让并发的
/// `recv` / `recv_timeout` 拿到 `Busy`——否则两个消费者会静默瓜分事件，与 v3 §5.8
/// 的「单订阅」契约不符。
struct WaiterGuard<'a, T, const N: usize> {
    ch: &'a EventChannel<T, N>,
    me: usize,
}

impl<'a, T, const N: usize> WaiterGuard<'a, T, N> {
    fn acquire(ch: &'a EventChannel<T, N>) -> Result<Self, DeviceError> {
        let me = (xworker.current() as *mut Task).addr();
        sync::free(|_| {
            if ch.closed.load(Ordering::SeqCst) {
                return Err(DeviceError::Cancelled);
            }
            if ch.waiter.load(Ordering::SeqCst) != 0 {
                return Err(DeviceError::Busy);
            }
            ch.waiter.store(me, Ordering::SeqCst);
            Ok(())
        })?;
        Ok(Self { ch, me })
    }
}

impl<T, const N: usize> Drop for WaiterGuard<'_, T, N> {
    fn drop(&mut self) {
        // 只清「我」这份：期间可能已被 `wake_waiting` 取走（swap→0），
        // 也可能被别的任务重新占住——无条件 store(0) 会把别人的登记抹掉。
        let _ = self
            .ch
            .waiter
            .compare_exchange(self.me, 0, Ordering::SeqCst, Ordering::SeqCst);
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

    /// R8：C2 方法在 sync::free 内调用必须 panic（debug 构建）。
    ///
    /// 两个通道都**先预置一条事件**：断言若回归，recv/recv_timeout 会立刻返回
    /// Ok(v)，本测试明确失败；不会走到 xworker.current()（host 无调度器时那里
    /// unwrap 空指针会 panic，使测试"因别的 panic 而假通过"），也不会进入
    /// 轮询/挂起导致挂死。
    #[test]
    fn c2_in_sync_free_asserts() {
        let a: EventChannel<u8, 2> = EventChannel::new();
        a.post(1).unwrap();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::sync::free(|_| a.recv())
        }));
        assert!(r.is_err(), "recv 在 sync::free 内调用必须触发 R8 断言");

        let b: EventChannel<u8, 2> = EventChannel::new();
        b.post(2).unwrap();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::sync::free(|_| b.recv_timeout(core::time::Duration::from_millis(10)))
        }));
        assert!(r.is_err(), "recv_timeout 在 sync::free 内调用必须触发 R8 断言");
    }

    /// Drop 必须排空环里未消费的事件并析构它们（否则 `T` 非 `Copy` 时纯泄漏）。
    #[test]
    fn drop_drains_unconsumed() {
        use alloc::sync::Arc;
        use core::sync::atomic::AtomicUsize as StdAtomicUsize;

        struct Tracked(Arc<StdAtomicUsize>);
        impl Drop for Tracked {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        let drops = Arc::new(StdAtomicUsize::new(0));
        {
            let ch: EventChannel<Tracked, 4> = EventChannel::new();
            ch.post(Tracked(drops.clone())).unwrap();
            ch.post(Tracked(drops.clone())).unwrap();
            drop(ch.try_recv()); // 只取走一条，另一条留给 Drop 排空
            assert_eq!(drops.load(Ordering::SeqCst), 1);
        }
        assert_eq!(
            drops.load(Ordering::SeqCst),
            2,
            "环里未消费的事件必须在 Drop 时析构"
        );
    }

    /// close 之后再 post 必须被拒（close 的 closed 写与 waiter 取在同一临界区）。
    #[test]
    fn post_after_close_rejected() {
        let ch: EventChannel<u8, 2> = EventChannel::new();
        ch.close();
        assert_eq!(ch.post(1), Err(DeviceError::Cancelled));
        assert_eq!(ch.post_isr(2), Err(DeviceError::Cancelled));
        assert!(ch.is_empty(), "关闭后不得入队任何事件");
        assert_eq!(ch.overflow_count(), 0, "关闭不是「满」，不计入 overflow");
    }
}
