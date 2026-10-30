//! v3 控制面能力（caps）：开关量、厂商原始通道、执行模型分类骨架。
//!
//! 模块纪律（见 `docs/设备驱动抽象层v3.md` §5.4/§5.5/§5.10）：
//! - 能力 trait 一律继承 [`DeviceBase`]（**不要求 `Sync`**），供独占句柄直接实现；
//! - 逃生舱与执行模型骨架**尚未接通**，一律收在 [`reserved`] 子模块里：
//!   `caps::SyncOp` 这类顶层路径取不到它们，用错了编译期就撞墙（见该模块文档）。
//!
//! 判断一个 trait 该放顶层还是 `reserved` 的唯一标准：**有没有实现方**。有实现方
//! （至少一个驱动 + 一条端到端调用）才进顶层；只有设计没有实现的，放 `reserved`——
//! 空 trait 摆在顶层会让「能力齐全」看起来成立，而 `find_capability` 永远返回
//! `None`。
//!
//! 本轮顶层判据的落点（§5.6）：`Switchable` 有实现方（`DrvLed` + driver 示例的
//! 端到端调用）；`Openable` 同样由 `DrvLed` 实现、driver 示例在填槽前 `open`
//! 端到端调用。`Recoverable` 是 §5.14 的规范项，已有真实实现方 `DrvLed`；其
//! owner 队列端到端调用随首个 owner 队列设备落地，本轮用 host 假件
//! （`FakeOwnerQueue`）钉住 `Cancelled` 语义。

use core::cell::Cell;

use crate::device::{DeviceBase, DeviceError};

/// 开关量能力：LED / 继电器 / 背光使能。
pub trait Switchable: DeviceBase {
    /// 打开（C2）。
    fn on(&self) -> Result<(), DeviceError>;
    /// 关闭（C2）。
    fn off(&self) -> Result<(), DeviceError>;
    /// 翻转（C2）。允许读回设备自己的输出寄存器，不允许另存影子副本。
    fn toggle(&self) -> Result<(), DeviceError>;
}

/// 常驻设备的开/闭生命周期（v3 §5.6）。
///
/// 两种形状并存：常驻设备用 `open(&self)` 做状态迁移（本 trait）；独占句柄用
/// `open() -> Result<Self>` 构造 + `Drop` 归还，**不实现**本 trait（§5.6 对齐修订）。
pub trait Openable: DeviceBase {
    /// 使能设备（C2）。重复 open → `Busy`，且不得重跑初始化。
    fn open(&self) -> Result<(), DeviceError>;
    /// 关闭设备（C2）。幂等；返回 () 因为析构路径无法返回错误。
    fn close(&self);
    /// 仅供诊断（C1）。**不得作为同步判据**：open/close 执行期间可能瞬时不准。
    fn is_open(&self) -> bool;
}

/// 复位/故障恢复（v3 §5.6 / §5.14，C2）。
///
/// 语义：reset 结束后设备状态 = `close` 后状态（业务方自行重新 `open`）；幂等。
/// reset 期间到达的命令**丢弃并回 `Cancelled`**，不排队等 reset 完成——这是 owner
/// 队列的职责；本仓当前没有生产 owner 队列，用 `#[cfg(test)]` 假件钉住语义。
pub trait Recoverable: DeviceBase {
    fn reset(&self) -> Result<(), DeviceError>;
}

/// 生命周期状态机（core 侧助手）。
///
/// # 临界区纪律
/// 调用方必须让**同一设备的 state 变更与硬件操作处于同一次 `sync::free` 内**
/// （§4.7 R5：多步不变量不得横跨两次临界区）。因此本类型自己**不取锁**：
/// - 目标端与宿主端的 `sync::free` 都可重入（§4.5；`src/sync/critical.rs` 有
///   线程本地深度计数），不取锁不是为了防嵌套死锁，而是为了不把
///   "检查状态 + 写硬件"拆成两段，并满足 §4.5"可能在 free 内被调用的方法必须
///   lock-free"；
/// - 使用 `Cell` 而不是原子：与 BSP 驱动现有 `RefCell + unsafe impl Sync` 的
///   临界区纪律同构，不引入 CAS，对 riscv32imc（ch572/esp32c3）零影响。
///
/// # 非重入
/// `open_with` 不防重入：驱动的 init 不得回调自身的 `open`。
pub struct OpenState {
    open: Cell<bool>,
}

impl OpenState {
    /// 初态：closed。
    pub const fn new() -> Self {
        Self {
            open: Cell::new(false),
        }
    }

    /// 已开 → `Busy`（不跑 init）；init 成功后才置位。
    /// init 失败时状态位未动 = 完全未 open，因此下一次 open 天然可重试。
    /// 半初始化资源的撤销是 init 闭包（驱动侧）的义务。
    pub fn open_with<F>(&self, init: F) -> Result<(), DeviceError>
    where
        F: FnOnce() -> Result<(), DeviceError>,
    {
        if self.open.get() {
            return Err(DeviceError::Busy);
        }
        init()?;
        self.open.set(true);
        Ok(())
    }

    /// 置为 closed（close/reset 共用；幂等）。
    pub fn mark_closed(&self) {
        self.open.set(false);
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    /// 业务方法的状态门：未 open → `InvalidInput`（§5.6）。
    pub fn ensure_open(&self) -> Result<(), DeviceError> {
        if self.open.get() {
            Ok(())
        } else {
            Err(DeviceError::InvalidInput)
        }
    }
}

/// Drop 凭证：析构时 `close`（析构不能返回错误）。
///
/// # 上下文
/// `drop` 调 `close`（C2 + `sync::free`）：**必须在任务上下文析构**。在
/// `sync::free` 内或 ISR 内析构会在 debug 构建触发 `assert_c2`；release 不检查。
///
/// # Drop 顺序
/// 含本类型的用户结构体按**字段声明顺序**析构。要得到"停 owner → close → 释放缓冲"，
/// 字段按此顺序声明：`SomeOwnerGuard` → `Owned` → 缓冲。
///
/// # 准入检查是 best-effort
/// `Owned::new` 的 `is_open` 检查只是早错，不是同步判据（§5.6：is_open 只作诊断）；
/// 正确性依赖单 owner 纪律 + `close` 幂等。`Owned::new` 调 C1 的 `is_open`，
/// 同样不要在 `sync::free`/ISR 内调用（R4）。
#[must_use = "Owned 在 Drop 时 close；立即丢弃等于立刻关闭"]
pub struct Owned<'a, T: Openable + ?Sized> {
    dev: &'a T,
}

impl<'a, T: Openable + ?Sized> Owned<'a, T> {
    pub fn new(dev: &'a T) -> Result<Self, DeviceError> {
        if dev.is_open() {
            Ok(Self { dev })
        } else {
            Err(DeviceError::InvalidInput)
        }
    }

    pub fn get(&self) -> &'a T {
        self.dev
    }
}

impl<'a, T: Openable + ?Sized> Drop for Owned<'a, T> {
    fn drop(&mut self) {
        self.dev.close();
    }
}

/// **预留面：只有设计、零实现**（接通前不要在业务代码里引用）。
///
/// 本模块收纳 v3 §5.3/§5.10 定义但**当前仓内没有任何实现方**的 trait：
///
/// - [`IoCtl`]：厂商原始命令逃生舱（§5.10）。首个消费者出现时移出到顶层，
///   并在 `define_capabilities!` 里加一行（若确实要经注册表下发）。
/// - [`SyncOp`] / [`PollingAsyncOp`] / [`EventSource`] / [`TransactionOp`]：
///   §5.3 的三型执行模型骨架，只有关联类型、没有方法。接通方式：先让**一个**
///   真设备走通端到端（例如 ADC → `SyncOp`），再按实测形状补方法。
///
/// 纪律：放进来的前提是「文档里写了、代码里没有」；移出去的前提是
/// 「至少一个驱动实现 + 至少一条端到端调用 + 一条测试」。**两边都不许含糊**。
pub mod reserved {
    use core::ffi::c_void;

    use crate::device::{DeviceBase, DeviceError};

    /// 厂商私有命令逃生舱（每设备一个）。
    ///
    /// 只有持有具体设备（typed accessor）的调用方才能用到它；**不经 `as_*`**，
    /// 因此不会变成一条"任何 `&dyn Device` 都能发任意 op"的万能控制面。
    pub trait IoCtl: DeviceBase {
        /// # Safety
        ///
        /// `arg` 的类型 / 大小 / 对齐 / 生命周期由 `op` 的厂商契约决定；仅在本次调用
        /// 期间有效，**不得留存**。`op` 取 `u32`（厂商 `ulong cmd`）。错用即厂商驱动侧 UB。
        unsafe fn ioctl_raw(&self, op: u32, arg: *mut c_void) -> Result<isize, DeviceError>;
    }

    // ---------------------------------------------------------------------------

    /// 同步型：一次调用完成，结果即 `Outcome`。
    pub trait SyncOp {
        type Outcome;
    }

    /// 轮询异步型：`start` 返回 `Op`，`poll` 轮询，`cancel` 取消。
    pub trait PollingAsyncOp {
        type Op: Copy;
        type Outcome;
    }

    /// 事件流型：结果经事件通道持续送达。
    pub trait EventSource {
        type Event: Copy;
    }

    /// 事务型：请求-应答 + 会话句柄 + 显式超时 + 可取消。
    pub trait TransactionOp {
        type Session: Copy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::DeviceKind;
    use core::cell::Cell;

    struct FakeLed {
        state: OpenState,
        fail_left: Cell<u32>,
        dirty: Cell<bool>,
        ops: Cell<u32>,
        close_calls: Cell<u32>,
        open_attempts: Cell<u32>,
        /// Drop 顺序用例的记录槽：[0] owner 守卫、[1] close、[2] 缓冲
        drop_log: Cell<[u8; 3]>,
    }

    impl FakeLed {
        fn new(fail_left: u32) -> Self {
            Self {
                state: OpenState::new(),
                fail_left: Cell::new(fail_left),
                dirty: Cell::new(false),
                ops: Cell::new(0),
                close_calls: Cell::new(0),
                open_attempts: Cell::new(0),
                drop_log: Cell::new([0; 3]),
            }
        }

        fn log_mark(&self, idx: usize, val: u8) {
            let mut log = self.drop_log.get();
            log[idx] = val;
            self.drop_log.set(log);
        }
    }

    impl DeviceBase for FakeLed {
        fn kind(&self) -> DeviceKind {
            DeviceKind::Control
        }
    }

    impl Openable for FakeLed {
        fn open(&self) -> Result<(), DeviceError> {
            self.state.open_with(|| {
                self.open_attempts.set(self.open_attempts.get() + 1);
                self.dirty.set(true); // 取得资源（尚未提交）
                if self.fail_left.get() > 0 {
                    self.fail_left.set(self.fail_left.get() - 1);
                    self.dirty.set(false); // 同一失败分支内撤销（§5.6 驱动侧义务）
                    return Err(DeviceError::Io);
                }
                self.dirty.set(false); // 成功：资源归设备所有
                Ok(())
            })
        }

        fn close(&self) {
            self.close_calls.set(self.close_calls.get() + 1);
            self.log_mark(1, 2);
            self.state.mark_closed();
        }

        fn is_open(&self) -> bool {
            self.state.is_open()
        }
    }

    impl Recoverable for FakeLed {
        fn reset(&self) -> Result<(), DeviceError> {
            self.state.mark_closed();
            Ok(())
        }
    }

    impl Switchable for FakeLed {
        fn on(&self) -> Result<(), DeviceError> {
            self.state.ensure_open()?;
            self.ops.set(self.ops.get() + 1);
            Ok(())
        }
        fn off(&self) -> Result<(), DeviceError> {
            self.state.ensure_open()?;
            Ok(())
        }
        fn toggle(&self) -> Result<(), DeviceError> {
            self.state.ensure_open()?;
            Ok(())
        }
    }

    // 说明：假件在单线程测试内独占，故直接访问 Cell；生产实现（DrvLed）必须把
    // OpenState 的每次调用放进 sync::free。OpenState 的临界区纪律由 DrvLed 与
    // 其文档保证。

    #[test]
    fn open_failure_leaves_fully_closed_and_is_retryable() {
        let f = FakeLed::new(1);
        assert_eq!(f.open(), Err(DeviceError::Io));
        assert!(!f.is_open(), "失败后必须完全未 open");
        assert!(!f.dirty.get(), "半初始化必须在失败分支内撤销");
        assert_eq!(f.open(), Ok(()), "失败后再次 open 必须成功");
        assert!(f.is_open());
        assert_eq!(f.open_attempts.get(), 2, "重试必须真正重跑 init");
    }

    #[test]
    fn double_open_is_busy_and_does_not_rerun_init() {
        let f = FakeLed::new(0);
        f.open().unwrap();
        assert_eq!(f.open(), Err(DeviceError::Busy));
        assert_eq!(f.open_attempts.get(), 1, "Busy 时不得重跑 init");
        assert!(f.is_open());
    }

    #[test]
    fn switchable_rejected_before_open_and_after_close() {
        let f = FakeLed::new(0);
        assert_eq!(f.on(), Err(DeviceError::InvalidInput));
        assert_eq!(f.off(), Err(DeviceError::InvalidInput));
        assert_eq!(f.toggle(), Err(DeviceError::InvalidInput));
        assert_eq!(f.ops.get(), 0, "未 open 不得触达业务实现");

        f.open().unwrap();
        f.on().unwrap();
        assert_eq!(f.ops.get(), 1);

        f.close();
        f.close();
        assert_eq!(f.close_calls.get(), 2, "close 幂等：重复调用仍关闭");
        assert_eq!(f.on(), Err(DeviceError::InvalidInput));
    }

    #[test]
    fn reset_closes_and_is_idempotent() {
        let f = FakeLed::new(0);
        f.open().unwrap();
        f.reset().unwrap();
        assert!(!f.is_open());
        assert_eq!(
            f.on(),
            Err(DeviceError::InvalidInput),
            "reset 后 = close 后状态"
        );
        f.reset().unwrap(); // 已关闭再 reset 仍 Ok（幂等）
        assert!(!f.is_open());
    }

    #[test]
    fn owned_closes_on_drop_and_requires_open() {
        let f = FakeLed::new(0);
        assert_eq!(Owned::new(&f).err(), Some(DeviceError::InvalidInput));
        f.open().unwrap();
        {
            let _o = Owned::new(&f).unwrap();
            assert!(f.is_open());
        }
        assert!(!f.is_open(), "Drop 必须 close");
        assert_eq!(f.close_calls.get(), 1);
    }

    /// §5.6 Drop 顺序：停 owner → close → 释放缓冲。
    /// 局部变量按声明逆序析构，故声明顺序为 缓冲 → Owned → owner 守卫。
    #[test]
    fn drop_order_stops_owner_then_closes_then_frees_buffer() {
        struct OwnerGuard<'a>(&'a FakeLed);
        impl Drop for OwnerGuard<'_> {
            fn drop(&mut self) {
                self.0.log_mark(0, 1);
            }
        }
        struct BufferGuard<'a>(&'a FakeLed);
        impl Drop for BufferGuard<'_> {
            fn drop(&mut self) {
                self.0.log_mark(2, 3);
            }
        }

        let f = FakeLed::new(0);
        f.open().unwrap();
        {
            let _buffer = BufferGuard(&f);
            let _owned = Owned::new(&f).unwrap();
            let _owner = OwnerGuard(&f);
        }
        assert_eq!(
            f.drop_log.get(),
            [1, 2, 3],
            "析构顺序必须是 owner → close → 缓冲"
        );
        assert!(!f.is_open());
    }

    struct FakeOwnerQueue {
        in_flight: Cell<bool>,
        result: Cell<Option<Result<(), DeviceError>>>,
    }

    impl FakeOwnerQueue {
        fn new() -> Self {
            Self {
                in_flight: Cell::new(false),
                result: Cell::new(None),
            }
        }

        fn submit(&self) {
            self.in_flight.set(true);
        }

        /// owner 在 reset 开始时的动作：在途命令丢弃，结果槽回 Cancelled。
        fn discard_on_reset(&self) {
            if self.in_flight.replace(false) {
                self.result.set(Some(Err(DeviceError::Cancelled)));
            }
        }

        fn result(&self) -> Option<Result<(), DeviceError>> {
            self.result.get()
        }
    }

    #[test]
    fn reset_discards_inflight_as_cancelled() {
        let q = FakeOwnerQueue::new();
        q.submit();
        q.discard_on_reset();
        assert_eq!(
            q.result(),
            Some(Err(DeviceError::Cancelled)),
            "在途命令必须丢弃并回 Cancelled，不得排队等 reset"
        );
        assert!(!q.in_flight.get());
    }
}
