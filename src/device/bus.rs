//! 共享总线仲裁与所有者守卫（v3 §5.15.1，core 层）。
//!
//! - [`BusArbiter`]：**同一物理总线名共享一把零分配任务阻塞锁**。`with_transaction`
//!   在 release 构建也真实串行化（`sync::mutex::Mutex`，任务阻塞、无堆分配），因此同一
//!   总线上挂多颗从机（多个 `BusDevice`/多个 arbiter）时不会互相插入。
//!   嵌套事务与 C2-in-C0/C1 由断言拦截；约束见 [`BusArbiter::with_transaction`]。
//! - [`claim_owner`]/[`OwnerGuard`]：厂商 SDK 设备（总线由厂商库内部持有、不经
//!   `BusDevice::transfer`）的**单份 claim** 执行机制：定长静态 owner 表、零分配，
//!   同一 name 第二次 claim → [`DeviceError::Busy`]，`Drop` 幂等释放。
//!   守卫必须**绑定并持有到任务结束**（`let _guard = claim_owner(..)?;`），
//!   语句结束即释放——`OwnerGuard` 标了 `#[must_use]`。
//! - **诚实边界**：owner 越权断言只在 dev/host 构建生效（`debug_assert`）；
//!   release 的串行化由 `with_transaction` 的锁保证，不由 owner 断言保证。
//! - 与顶层 [`crate::bus`]（PubSub 消息总线）无关，勿混淆。
//!
//! 上下文分级：`with_transaction` 为 C2（入口 `assert_c2`）；
//! `claim_owner`/`owner_of` 为 C1（单段短临界区、不阻塞、ISR 安全）；
//! `OwnerGuard::drop` 使用可重入 `sync::free`，任何上下文安全。

use core::cell::UnsafeCell;
// 原子走 atomic-polyfill（与 device/event.rs:19、sync/arc.rs 同源）：`in_txn` 用的是
// **swap**（CAS 式），riscv32imc（ch572/esp32c3）无 A 扩展时 core 的 `AtomicBool::swap`
// 根本不存在——用 core 会在这两个目标上编译失败（门禁 [2/4] 抓到的就是这个）。
use atomic_polyfill::{AtomicBool, Ordering};

use crate::device::{BusDevice, DeviceError};
use crate::port::{Portable, Porting};
use crate::sync;

/// owner 表容量：定长、零分配（样本 12 个厂商 SDK 句柄 + 余量）
const OWNER_CAP: usize = 16;

#[derive(Clone, Copy)]
struct OwnerEntry {
    /// 登记名（空串 = 空槽）
    name: &'static str,
    /// 任务身份令牌（`xworker.current_ptr()` 的地址值；0 = 未绑定/boot 上下文）
    task: usize,
}

#[cfg(test)]
thread_local! {
    /// 重入标志按线程隔离：cargo test 并行线程互不干扰。
    static IN_TABLE: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}
#[cfg(not(test))]
static mut IN_TABLE: [bool; crate::port::MAX_HARTS] = [false; crate::port::MAX_HARTS];

/// 本上下文是否已在 `OwnerTable::with` 内（重入检测）。
#[inline]
fn table_busy() -> bool {
    #[cfg(test)]
    {
        IN_TABLE.with(|c| c.get())
    }
    #[cfg(not(test))]
    {
        // SAFETY: 每核只读写自己的槽；`with` 的访问都在 sync::free 内或紧邻其前后，
        // 同核不会并发改写本槽。
        unsafe { IN_TABLE[Porting::hart_id() as usize] }
    }
}

#[inline]
fn set_table_busy(v: bool) {
    #[cfg(test)]
    {
        IN_TABLE.with(|c| c.set(v))
    }
    #[cfg(not(test))]
    {
        // SAFETY: 同上。
        unsafe { IN_TABLE[Porting::hart_id() as usize] = v }
    }
}

/// 定长静态 owner 表：name → 任务令牌。
struct OwnerTable {
    slots: UnsafeCell<[OwnerEntry; OWNER_CAP]>,
}

// SAFETY: 唯一访问入口 `with` 把每次访问包在 `sync::free` 内（目标端关本核中断 +
// SMP 全局自旋；host 测试为进程互斥锁），且不向临界区外暴露指向表内部的引用
// （对外只按值拷出 `&'static str` / `usize`）。因此不存在跨上下文的并发访问。
unsafe impl Sync for OwnerTable {}

impl OwnerTable {
    const fn new() -> Self {
        Self {
            slots: UnsafeCell::new([OwnerEntry { name: "", task: 0 }; OWNER_CAP]),
        }
    }

    /// 唯一访问入口：整段「查找 + 写入」在同一段 `sync::free` 内（R5）。
    fn with<R>(&self, f: impl FnOnce(&mut [OwnerEntry; OWNER_CAP]) -> R) -> R {
        struct BusyGuard;
        impl Drop for BusyGuard {
            fn drop(&mut self) {
                set_table_busy(false);
            }
        }
        debug_assert!(
            !table_busy(),
            "OwnerTable::with 重入：闭包内不得再次调用 claim/owner_of/release"
        );
        set_table_busy(true);
        let _g = BusyGuard;
        sync::free(|_| {
            // SAFETY: 已持本核临界区，且这是表的唯一访问路径；`f` 无法把表内
            // 引用带出闭包（R 与闭包入参的生命周期无关）。重入由 busy 守卫拦截。
            f(unsafe { &mut *self.slots.get() })
        })
    }

    /// 查找登记名本身（`owner_of` 诊断用）。
    fn name_of(&self, name: &str) -> Option<&'static str> {
        self.with(|s| {
            s.iter()
                .find(|e| !e.name.is_empty() && e.name == name)
                .map(|e| e.name)
        })
    }

    /// 登记：查重 + 找空槽 + 写入在同一段临界区（R5 单段不变量）。
    fn claim(&self, name: &'static str, task: usize) -> Result<(), DeviceError> {
        self.with(|s| {
            if s.iter().any(|e| !e.name.is_empty() && e.name == name) {
                return Err(DeviceError::Busy);
            }
            match s.iter_mut().find(|e| e.name.is_empty()) {
                Some(slot) => {
                    *slot = OwnerEntry { name, task };
                    Ok(())
                }
                None => Err(DeviceError::Full),
            }
        })
    }

    /// 幂等释放：只释放 `(name, task)` 完全匹配的登记——旧守卫不会误放后来被
    /// 其他任务重新 claim 的同名登记；重复释放是空操作。
    fn release(&self, name: &str, task: usize) {
        self.with(|s| {
            if let Some(slot) = s
                .iter_mut()
                .find(|e| !e.name.is_empty() && e.name == name && e.task == task)
            {
                *slot = OwnerEntry { name: "", task: 0 };
            }
        });
    }

    /// 越权断言：在同一段临界区里读 (claimed, current)，避免跨区不一致（R5）。
    fn owner_matches_current(&self, name: &str) -> bool {
        self.with(|s| {
            let claimed = s
                .iter()
                .find(|e| !e.name.is_empty() && e.name == name)
                .map(|e| e.task);
            // SAFETY: 正在本表 `with` 的 sync::free 内。
            let cur = unsafe { current_task_token_locked() };
            match (claimed, cur) {
                // 未 claim / task=0（init 占位）/ 调度器未启动 → 无法判定，跳过
                (Some(0), _) => true,
                (Some(claimed), Some(cur)) => claimed == cur,
                _ => true,
            }
        })
    }
}

static OWNERS: OwnerTable = OwnerTable::new();

#[cfg(test)]
thread_local! {
    /// host 测试无调度器：用线程本地令牌注入模拟“不同任务”。按线程隔离，
    /// cargo test 并行线程互不干扰。
    static TEST_TOKEN: core::cell::Cell<Option<usize>> = core::cell::Cell::new(None);
}

/// 当前任务身份令牌（`xworker.current_ptr()` 的地址值）；调度器未启动 → None。
/// **调用方必须已持 `sync::free`**：本核 CURRENT 槽由本核调度器写，关中断即互斥。
#[cfg(all(not(test), feature = "xtask_executor"))]
unsafe fn current_task_token_locked() -> Option<usize> {
    // SAFETY: 只读本核 CURRENT 槽且不 deref；调用方已持临界区（关本核中断），
    // 本核不会在其间切换任务；该槽是机器字，读不会撕裂。
    let p = unsafe { crate::task::scheduler::xworker::current_ptr() };
    if p.is_null() {
        None
    } else {
        Some(p as usize)
    }
}

#[cfg(test)]
unsafe fn current_task_token_locked() -> Option<usize> {
    // host 测试从不写 CURRENT 槽，令牌由 `set_test_token` 注入。
    TEST_TOKEN.with(|c| c.get())
}

#[cfg(all(not(test), not(feature = "xtask_executor")))]
unsafe fn current_task_token_locked() -> Option<usize> {
    None
}

/// 当前任务身份令牌，供 examples / 厂商 SDK 驱动取得 `claim_owner` 的 `task`。
/// 调度器未启动（init）→ None，此时调用方传 0（未绑定占位）。
///
/// 用法：`let _guard = claim_owner(OWNER, current_task_token().unwrap_or(0))?;`
/// ——守卫必须绑定持有，否则语句结束即释放。
pub fn current_task_token() -> Option<usize> {
    sync::free(|_| {
        // SAFETY: 刚进本核临界区。
        unsafe { current_task_token_locked() }
    })
}

#[cfg(test)]
fn set_test_token(v: Option<usize>) {
    TEST_TOKEN.with(|c| c.set(v));
}

/// owner 守卫：持有期间该 name 被本任务独占；`Drop` 释放。
///
/// **必须绑定并持有到任务/actor 生命周期结束**：`claim_owner(..)?;` 这种不绑定
/// 写法会在语句结束即释放。它应作为结构体**首个字段**声明（先于缓冲/设备状态
/// Drop，对应 §5.6「停 owner → close → 释放」）。`Drop` 从任意任务执行都视作
/// 撤销所有权。
#[must_use = "claim_owner 守卫必须持有；Drop 即释放"]
pub struct OwnerGuard {
    name: &'static str,
    task: usize,
}

impl OwnerGuard {
    /// 登记名（诊断/测试用）
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// 任务身份令牌（诊断/测试用）
    pub fn task(&self) -> usize {
        self.task
    }
}

impl Drop for OwnerGuard {
    fn drop(&mut self) {
        OWNERS.release(self.name, self.task);
    }
}

/// claim 设备/总线的 owner。
///
/// - `name`：全局唯一登记名（通常取设备名或 `const OWNER`），不得为空；
/// - `task`：任务身份令牌（目标端取 [`current_task_token`]；0 表示 boot/init
///   上下文占位、不参与任务断言；host 测试可传任意显式非 0 令牌）；
/// - 同一 `name` 第二次 claim（含同一任务重复 claim）→ [`DeviceError::Busy`]；
/// - 表满 → [`DeviceError::Full`]；空名 → [`DeviceError::InvalidInput`]。
pub fn claim_owner(name: &'static str, task: usize) -> Result<OwnerGuard, DeviceError> {
    if name.is_empty() {
        return Err(DeviceError::InvalidInput);
    }
    OWNERS.claim(name, task)?;
    Ok(OwnerGuard { name, task })
}

/// 厂商 SDK 设备的 C2 入口越权断言（dev/host 生效）。
///
/// 用法：在驱动的 C2 方法首行 `crate::device::bus::assert_owner(OWNER);`。
/// 该 name 未 claim、task=0（init 占位）或调度器未启动时判为通过（无法判定）。
pub fn assert_owner(name: &str) {
    debug_assert!(
        OWNERS.owner_matches_current(name),
        "C2 越权：{} 的 owner 不是当前任务",
        name
    );
}

/// 共享总线仲裁器（core 层具体类型，不是 trait：`with_transaction` 带泛型，
/// 带泛型方法的 trait 不 object-safe；core 只有一个实现）。
pub struct BusArbiter<'a> {
    bus: &'a dyn BusDevice,
    /// 总线名（同时是 owner 登记名）：同名 arbiter 共享同一把锁
    owner: &'static str,
    /// 该**物理总线**的共享锁（release 生效的真实串行化）。
    /// 同一总线的所有从机必须传入同一把 `&'static Mutex<()>`。
    lock: &'static sync::mutex::Mutex<()>,
    /// 嵌套事务检测（RAII 复位，panic 安全）
    in_txn: AtomicBool,
}

impl<'a> BusArbiter<'a> {
    /// 构造仲裁器。
    ///
    /// - `bus`：本 arbiter 代理的从机（一个 `BusDevice` 对应一个 `(总线, 地址)`）；
    /// - `owner`：总线/设备名（诊断与 owner 断言用）；
    /// - `lock`：**该物理总线唯一的一把锁**。同总线上挂多颗从机时，所有
    ///   `BusArbiter` 必须传入同一个 `&'static Mutex<()>`（零分配）：
    ///   ```ignore
    ///   static I2C0_LOCK: xtask::sync::mutex::Mutex<()> = xtask::sync::mutex::Mutex::new(());
    ///   let a = BusArbiter::new(&sensor, "i2c0.sensor", &I2C0_LOCK);
    ///   ```
    pub fn new(
        bus: &'a dyn BusDevice,
        owner: &'static str,
        lock: &'static sync::mutex::Mutex<()>,
    ) -> Self {
        assert!(!owner.is_empty(), "BusArbiter owner 名不得为空");
        Self {
            bus,
            owner,
            lock,
            in_txn: AtomicBool::new(false),
        }
    }

    /// 在一个事务里执行 `f`（C2）。
    ///
    /// - **release 真实串行化**：持「同一总线名」共享的零分配任务阻塞锁；
    ///   同名 arbiter（同总线的多颗从机）互斥；
    /// - **不可嵌套**：同一 arbiter 的闭包内再调 `with_transaction` → panic（避免
    ///   自锁）；跨 arbiter 但同总线名的嵌套会阻塞（同总线重入本就是设计错误）；
    /// - **C2 约束**：不得在 `sync::free` / ISR 内调用（`assert_c2`）。
    pub fn with_transaction<R>(&self, f: impl FnOnce(&dyn BusDevice) -> R) -> R {
        sync::assert_c2("BusArbiter::with_transaction");
        struct Txn<'a>(&'a AtomicBool);
        impl Drop for Txn<'_> {
            fn drop(&mut self) {
                self.0.store(false, Ordering::SeqCst);
            }
        }
        assert!(
            !self.in_txn.swap(true, Ordering::SeqCst),
            "BusArbiter::with_transaction 不可嵌套"
        );
        let _txn = Txn(&self.in_txn);
        // 越权断言（dev/host）：owner 已 claim 且绑定任务时，调用者必须是该任务。
        debug_assert!(
            OWNERS.owner_matches_current(self.owner),
            "总线事务越权：{} 的 owner 不是当前任务",
            self.owner
        );
        let _g = self.lock.lock();
        f(self.bus)
    }

    /// 诊断（C1）：`bus` 当前是否被 claim、登记名是什么；未登记 → None。
    /// 固定 API 只有名字可用，故返回登记名本身（name 即 key）。
    pub fn owner_of(&self, bus: &str) -> Option<&'static str> {
        OWNERS.name_of(bus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{Device, DeviceBase, DeviceKind};
    use core::sync::atomic::{AtomicUsize, Ordering};

    /// 每个用例独立一把锁（并行测试互不阻塞；生产用 static）。
    fn test_lock() -> &'static crate::sync::mutex::Mutex<()> {
        Box::leak(Box::new(crate::sync::mutex::Mutex::new(())))
    }

    struct MockBus {
        xfers: AtomicUsize,
    }

    impl DeviceBase for MockBus {
        fn kind(&self) -> DeviceKind {
            DeviceKind::Bus
        }
    }
    impl Device for MockBus {
        fn as_bus(&self) -> Option<&dyn BusDevice> {
            Some(self)
        }
    }
    impl BusDevice for MockBus {
        fn transfer(&self, _tx: &[u8], rx: &mut [u8]) -> Result<(), DeviceError> {
            self.xfers.fetch_add(1, Ordering::SeqCst);
            rx.fill(0xA5);
            Ok(())
        }
    }

    #[test]
    fn claim_twice_returns_busy() {
        const N: &str = "bus.test.twice";
        let g = claim_owner(N, 1).unwrap();
        assert!(matches!(claim_owner(N, 2), Err(DeviceError::Busy)));
        assert!(matches!(claim_owner(N, 1), Err(DeviceError::Busy)));
        drop(g);
        drop(claim_owner(N, 3).unwrap());
    }

    #[test]
    fn drop_releases_and_allows_reclaim() {
        const N: &str = "bus.test.reclaim";
        let bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let arb = BusArbiter::new(&bus, N, test_lock());
        let g = claim_owner(N, 1).unwrap();
        assert_eq!(arb.owner_of(N), Some(N));
        drop(g);
        assert_eq!(arb.owner_of(N), None);
        drop(claim_owner(N, 2).unwrap());
        assert_eq!(arb.owner_of(N), None);
    }

    #[test]
    fn release_is_idempotent() {
        const A: &str = "bus.test.idem.a";
        const B: &str = "bus.test.idem.b";
        let ga = claim_owner(A, 7).unwrap();
        let gb = claim_owner(B, 8).unwrap();
        let name = ga.name();
        drop(ga);
        OWNERS.release(name, 7); // 二次释放：空操作，且经 with 自取临界区
        assert_eq!(OWNERS.name_of(name), None);
        assert_eq!(OWNERS.name_of(B), Some(B), "其它登记不得受影响");
        drop(gb);
    }

    #[test]
    fn distinct_names_independent() {
        const A: &str = "bus.test.a";
        const B: &str = "bus.test.b";
        let ga = claim_owner(A, 1).unwrap();
        let gb = claim_owner(B, 2).unwrap();
        drop(ga);
        assert!(matches!(claim_owner(A, 3), Ok(_)));
        assert!(matches!(claim_owner(B, 3), Err(DeviceError::Busy)));
        drop(gb);
    }

    #[test]
    fn with_transaction_transfers_once() {
        let bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let arb = BusArbiter::new(&bus, "bus.test.arbiter", test_lock());
        let mut rx = [0u8; 2];
        let n = arb.with_transaction(|b| {
            b.transfer(&[0x10, 0x00], &mut rx).unwrap();
            7
        });
        assert_eq!(n, 7);
        assert_eq!(rx, [0xA5, 0xA5]);
        assert_eq!(bus.xfers.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn owner_of_reports_transitions() {
        const N: &str = "bus.test.owner_of";
        let bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let arb = BusArbiter::new(&bus, "bus.test.host", test_lock());
        assert_eq!(arb.owner_of(N), None);
        let g = claim_owner(N, 11).unwrap();
        assert_eq!(arb.owner_of(N), Some(N));
        drop(g);
        assert_eq!(arb.owner_of(N), None);
        assert_eq!(arb.owner_of("bus.test.unknown"), None);
    }

    #[test]
    fn claim_empty_name_invalid() {
        assert!(matches!(claim_owner("", 1), Err(DeviceError::InvalidInput)));
        drop(claim_owner("bus.test.after_empty", 1).unwrap());
    }

    #[test]
    fn full_local_table_returns_full() {
        // 用局部 OwnerTable，避免填满进程级 static 表干扰并行测试。
        const NAMES: [&str; 17] = [
            "f00", "f01", "f02", "f03", "f04", "f05", "f06", "f07", "f08", "f09", "f10", "f11",
            "f12", "f13", "f14", "f15", "f16",
        ];
        let t = OwnerTable::new();
        for &n in &NAMES[..OWNER_CAP] {
            t.claim(n, 1).unwrap();
        }
        assert!(matches!(
            t.claim(NAMES[OWNER_CAP], 1),
            Err(DeviceError::Full)
        ));
    }

    /// §5.15.1 验收②：非 owner 任务调用 C2 → dev 断言触发。
    #[test]
    fn non_owner_transaction_asserts() {
        const N: &str = "bus.test.assert";
        let bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let arb = BusArbiter::new(&bus, N, test_lock());
        let g = claim_owner(N, 11).unwrap();
        set_test_token(Some(12));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            arb.with_transaction(|_b| ());
        }));
        set_test_token(Some(11));
        assert!(r.is_err(), "非 owner 任务调用必须触发 dev 断言");
        arb.with_transaction(|_b| ()); // owner 本任务：不 panic
        set_test_token(None);
        drop(g);
    }

    /// R8：C2 在 sync::free 内调用必须 panic（debug 构建）。
    #[test]
    fn c2_in_free_asserts() {
        let bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let arb = BusArbiter::new(&bus, "bus.test.free", test_lock());
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            sync::free(|_| arb.with_transaction(|_b| ()))
        }));
        assert!(r.is_err(), "with_transaction 在 sync::free 内必须触发 R8 断言");
    }

    #[test]
    fn nested_transaction_panics() {
        const N: &str = "bus.test.nested";
        let bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let arb = BusArbiter::new(&bus, N, test_lock());
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            arb.with_transaction(|_b| {
                let _ = arb.with_transaction(|_| ());
            })
        }));
        assert!(r.is_err(), "同一 arbiter 嵌套事务必须 panic");
        // RAII 复位后仍可用
        arb.with_transaction(|_b| ());
    }

    #[test]
    fn assert_owner_rejects_non_owner() {
        const N: &str = "bus.test.assert-owner";
        let g = claim_owner(N, 11).unwrap();
        set_test_token(Some(11));
        assert_owner(N); // 同任务：不 panic
        set_test_token(Some(22));
        let r = std::panic::catch_unwind(|| assert_owner(N));
        assert!(r.is_err(), "非 owner 必须触发越权断言");
        set_test_token(None);
        drop(g);
    }
}
