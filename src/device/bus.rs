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
//! - **owner 绑定与身份代次**：init 阶段 `claim_owner(OWNER, 0)` 建立的是**未绑定**
//!   占位（此时 `assert_owner` 对任何任务放行）；owner 任务入口调用
//!   [`OwnerGuard::bind_current`]（或 `bind(task)`）后断言才真正生效。绑定同时记录
//!   **身份指纹**（启发式代次），用于识别「同一 Task 地址上的另一代任务」。
//!   [`OwnerGuard::rebind`] 用于 owner 重启/所有权移交。
//! - **诚实边界**：owner 越权断言只在 dev/host 构建生效（`debug_assert`）；
//!   release 的串行化由 `with_transaction` 的锁保证，不由 owner 断言保证。
//!   `bind` 只在建立绑定/换绑时算一次指纹，**不能**据此宣称 release 有运行时校验。
//! - **指纹是启发式代次，不是硬代次**：名称字节 + 栈地址 + 栈尺寸 + entry/args + id。
//!   分配器同时复用同一个 Task 地址**与同一栈地址**、且新任务同名同栈尺寸时仍会
//!   碰撞——而这恰是最可能的复用形态。硬代次需要给 `Task` 加 generation 字段
//!   （改布局）或 spawn/drop 钩子（跨文件），本轮不做，风险如实保留。
//! - **信任边界**：未绑定时**首个**拿到守卫、在任务上下文里调 `bind_current` 的任务
//!   即成为 owner；`bind(task)` 的参数不是「替别人绑定」，而是「要求 task 必须等于
//!   当前任务」。`rebind` 还要求新任务能访问到守卫。这些 API 不能用来约束不可信代码。
//! - 与顶层 [`crate::bus`]（PubSub 消息总线）无关，勿混淆。
//!
//! 上下文分级：`with_transaction` 为 C2（入口 `assert_c2`）；
//! `claim_owner`/`owner_of`/`assert_owner` 为 C1（单段短临界区、不阻塞、ISR 安全）；
//! `bind`/`bind_current`/`rebind` 需要**任务上下文**（读当前身份），只能在调度器
//! 启动后由 owner 任务调用；`OwnerGuard::drop` 使用可重入 `sync::free`，任何上下文安全。
//!
//! 跨 arbiter 共享同一把总线锁时，同一任务在事务内再进另一个 arbiter 会**自锁**
//! （普通 Mutex 不可重入）；`with_transaction` 在取锁前用只读 owner 快照识别并
//! 立即 panic，而不是永久阻塞。

use core::cell::UnsafeCell;
// 原子一律走 atomic-polyfill（与 device/event.rs、sync/arc.rs 同源）：无 A 扩展的
// riscv32imc（ch572/esp32c3）上 core 没有 CAS 式 RMW；`in_txn` 的 compare_exchange
// 必须经 polyfill 的 critical-section 兜底。
use atomic_polyfill::{AtomicUsize, Ordering};

use crate::device::{BusDevice, DeviceError};
use crate::port::{Portable, Porting};
use crate::sync;

/// owner 表容量：定长、零分配（样本 12 个厂商 SDK 句柄 + 余量）
const OWNER_CAP: usize = 16;

#[derive(Clone, Copy)]
struct OwnerEntry {
    /// 登记名（空串 = 空槽）
    name: &'static str,
    /// 任务身份令牌（`current_ptr` 地址值；0 = 未绑定 / init 占位）
    task: usize,
    /// 绑定时刻的任务身份指纹；0 = 未知（host 注入、task==0、无执行器）
    fingerprint: usize,
}

#[cfg(test)]
thread_local! {
    /// 重入标志按线程隔离：cargo test 并行线程互不干扰。
    static IN_TABLE: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}
#[cfg(not(test))]
static mut IN_TABLE: [bool; crate::port::MAX_HARTS] = [false; crate::port::MAX_HARTS];

/// 本核在 `IN_TABLE` 里的槽下标（与 critical.rs / xworker.rs 同款闸：端口报出
/// 超界 hart id 时开发期断言可见，release 下夹到末槽，绝不越界读写 `static mut`）。
#[cfg(not(test))]
#[inline]
fn table_slot() -> usize {
    let h = Porting::hart_id() as usize;
    debug_assert!(h < crate::port::MAX_HARTS);
    h.min(crate::port::MAX_HARTS - 1)
}

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
        unsafe { IN_TABLE[table_slot()] }
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
        unsafe { IN_TABLE[table_slot()] = v }
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
            slots: UnsafeCell::new([OwnerEntry {
                name: "",
                task: 0,
                fingerprint: 0,
            }; OWNER_CAP]),
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
        sync::free(|_| {
            // 置位/清除必须与表访问同在本核临界区内：放在 free 之外时，
            // 「置 true → 关中断」窗口可被 systick 抢占，另一个任务随后调
            // claim/release 会误命中 debug_assert（把正常并发报成重入）；
            // 「重开中断 → 清标志」窗口里的 ISR 调用（C1 声明为 ISR 安全）同理。
            debug_assert!(
                !table_busy(),
                "OwnerTable::with 重入：闭包内不得再次调用 claim/owner_of/release"
            );
            set_table_busy(true);
            let _g = BusyGuard;
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
    /// `fingerprint` 只有「入参 task 就是当前任务」时才非 0，否则为未知（0）。
    fn claim(
        &self,
        name: &'static str,
        task: usize,
        fingerprint: usize,
    ) -> Result<(), DeviceError> {
        self.with(|s| {
            if s.iter().any(|e| !e.name.is_empty() && e.name == name) {
                return Err(DeviceError::Busy);
            }
            match s.iter_mut().find(|e| e.name.is_empty()) {
                Some(slot) => {
                    *slot = OwnerEntry {
                        name,
                        task,
                        fingerprint,
                    };
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
                *slot = OwnerEntry {
                    name: "",
                    task: 0,
                    fingerprint: 0,
                };
            }
        });
    }

    /// 绑定/换绑：把 `name` 登记绑定到 `task` 并记录其身份指纹。
    ///
    /// 查 + 校验 + 写在同一段 `sync::free`（R5）。三道身份检查：
    /// 1. 登记不存在（已释放）→ [`DeviceError::Busy`]；
    /// 2. 登记不属于本守卫记录的旧令牌（已被替换）→ `Busy`；
    /// 3. 登记已绑定到**同一令牌**但两侧指纹都已知且不同（地址复用）→ `Busy`
    ///    ——否则复用同一 Task 地址的新任务只要拿到守卫调一次 bind 就能洗白指纹。
    ///
    /// 残余风险：登记被 `release` 后又被同名 `task=0` 重新 claim 时，仍存活的
    /// 旧守卫会把新登记绑到自己身上（`slot.task == 0 == expect_task`）。现实中
    /// release 只能经 Drop 消耗守卫，属于误用已释放守卫的未定义用法；要真正
    /// 闭合需要 claim serial，本轮不做。
    fn bind(
        &self,
        name: &str,
        expect_task: usize,
        expect_fp: usize,
        task: usize,
        fp: usize,
        allow_rebind: bool,
    ) -> Result<(), DeviceError> {
        self.with(|s| {
            // 查 + 校验 + 写 在同一段 sync::free（R5）
            let Some(slot) = s.iter_mut().find(|e| !e.name.is_empty() && e.name == name) else {
                return Err(DeviceError::Busy); // 登记已释放/不存在
            };
            if slot.task != expect_task {
                return Err(DeviceError::Busy); // 不是本守卫的登记（或已被替换）
            }
            if expect_fp != 0 && slot.fingerprint != 0 && slot.fingerprint != expect_fp {
                return Err(DeviceError::Busy); // 登记被同令牌的另一代次替换
            }
            // 地址复用的核心防线：登记已绑定到同一令牌，但当前身份指纹不同 → 拒绝
            if slot.task == task && slot.fingerprint != 0 && fp != 0 && slot.fingerprint != fp {
                return Err(DeviceError::Busy);
            }
            // bind 只做 0→真实任务或同任务幂等；换绑必须显式 rebind
            if slot.task != 0 && slot.task != task && !allow_rebind {
                return Err(DeviceError::Busy);
            }
            slot.task = task;
            slot.fingerprint = fp;
            Ok(())
        })
    }

    /// 越权断言：在同一段临界区里读 (claimed, current)，避免跨区不一致（R5）。
    fn owner_matches_current(&self, name: &str) -> bool {
        self.with(|s| {
            let Some(e) = s.iter().find(|e| !e.name.is_empty() && e.name == name) else {
                return true; // 未 claim / 已释放 → 无法判定，放行
            };
            if e.task == 0 {
                return true; // init 占位（未绑定）
            }
            // SAFETY: 正在本表 `with` 的 sync::free 内；登记与身份同区读（R5）
            let Some((tok, fp)) = (unsafe { current_identity_locked() }) else {
                return true; // 调度器未启动 → 无法判定
            };
            if e.task != tok {
                return false; // 明确非 owner
            }
            match (e.fingerprint, fp) {
                (0, _) | (_, 0) => true, // 任一侧未知 → 退回令牌判定
                (a, b) => a == b,        // 同令牌再用指纹排除地址复用
            }
        })
    }
}

static OWNERS: OwnerTable = OwnerTable::new();

#[cfg(test)]
thread_local! {
    /// host 测试无调度器：用线程本地令牌注入模拟“不同任务”。按线程隔离，
    /// cargo test 并行线程互不干扰。
    static TEST_TOKEN: core::cell::Cell<Option<usize>> = const { core::cell::Cell::new(None) };
    /// 与令牌同源的线程本地身份指纹；None = 未知（与目标端“无执行器”同一语义）。
    static TEST_FP: core::cell::Cell<Option<usize>> = const { core::cell::Cell::new(None) };
}

/// 当前执行上下文的身份 (任务令牌, 身份指纹)；无任务上下文 → None。
/// **调用方必须已持 `sync::free`**（本核 CURRENT 槽由本核调度器写，关中断即互斥）。
#[cfg(all(not(test), feature = "xtask_executor"))]
unsafe fn current_identity_locked() -> Option<(usize, usize)> {
    // SAFETY: 调用方已持本核临界区；只读本核 CURRENT 槽，单字读不撕裂。
    let p = unsafe { crate::task::scheduler::xworker::current_ptr() };
    if p.is_null() {
        None
    } else {
        // SAFETY: p 来自本核 CURRENT 槽 = 正在执行的任务，必然未释放；
        // task_fingerprint 只读其生命周期内不可变的字段。
        Some((p as usize, unsafe { task_fingerprint(p) }))
    }
}

#[cfg(test)]
unsafe fn current_identity_locked() -> Option<(usize, usize)> {
    // host 无调度器：令牌/指纹均由线程本地注入；绝不 deref 注入的令牌。
    TEST_TOKEN.with(|t| {
        t.get()
            .map(|tok| (tok, TEST_FP.with(|f| f.get()).unwrap_or(0)))
    })
}

#[cfg(all(not(test), not(feature = "xtask_executor")))]
unsafe fn current_identity_locked() -> Option<(usize, usize)> {
    None
}

/// 只读当前任务令牌（不 deref Task、不算指纹）：`with_transaction` 等只需要
/// “是不是同一执行上下文”的热路径使用，避免在 release 的临界区内做哈希。
#[cfg(all(not(test), feature = "xtask_executor"))]
unsafe fn current_token_locked() -> Option<usize> {
    // SAFETY: 调用方已持本核临界区；只读本核 CURRENT 槽，单字读不撕裂。
    let p = unsafe { crate::task::scheduler::xworker::current_ptr() };
    if p.is_null() {
        None
    } else {
        Some(p as usize)
    }
}
#[cfg(test)]
unsafe fn current_token_locked() -> Option<usize> {
    TEST_TOKEN.with(|t| t.get())
}
#[cfg(all(not(test), not(feature = "xtask_executor")))]
unsafe fn current_token_locked() -> Option<usize> {
    None
}

/// 当前任务令牌；语义与旧的 `current_task_token_locked` 完全一致。
unsafe fn current_task_token_locked() -> Option<usize> {
    // SAFETY: 调用方已持 sync::free
    unsafe { current_token_locked() }
}

/// 任务身份指纹：区分“同一地址上的不同任务”（地址复用）。
///
/// SAFETY: `p` 必须是**未释放的活 Task**。本函数只读任务生命周期内不可变的
/// 字段（name / stack / stack_size / entry / args / id），不写入、不产生引用逃逸。
/// 目标端唯一调用点 `current_identity_locked` 的 p 来自本核 CURRENT 槽（正在执行，
/// 必然存活）；cfg(test) 分支只对 `Task::new` 建出的活对象调用，绝不 deref 注入令牌。
#[cfg(any(test, feature = "xtask_executor"))]
unsafe fn task_fingerprint(p: *mut crate::task::Task) -> usize {
    const OFF: u64 = 0xcbf2_9ce4_8422_2325; // FNV-1a offset basis
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    // SAFETY: 见函数契约：p 是未释放的活 Task。
    let t = unsafe { &*p };
    let mut h = OFF;
    for &b in t.name.as_bytes() {
        h = (h ^ b as u64).wrapping_mul(PRIME);
    }
    // 栈地址是活任务之间唯一的判别位；stack_size/entry/args 增加碰撞代价。
    h = (h ^ t.stack as usize as u64).wrapping_mul(PRIME);
    h = (h ^ t.stack_size as u64).wrapping_mul(PRIME);
    h = (h ^ t.entry as usize as u64).wrapping_mul(PRIME);
    h = (h ^ t.args as usize as u64).wrapping_mul(PRIME);
    // id 不是代次（普通任务恒 1，仅 idle 为 0），只作额外搅动。
    h = (h ^ t.id as u64).wrapping_mul(PRIME);
    let h = h as usize;
    if h == 0 {
        1
    } else {
        h
    } // 0 保留为“未知”
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

#[cfg(test)]
fn set_test_fp(v: Option<usize>) {
    TEST_FP.with(|c| c.set(v));
}

/// owner 守卫：持有期间该 name 被本任务独占；`Drop` 释放。
///
/// **必须绑定并持有到任务/actor 生命周期结束**：`claim_owner(..)?;` 这种不绑定
/// 写法会在语句结束即释放。它应作为结构体**首个字段**声明（先于缓冲/设备状态
/// Drop，对应 §5.6「停 owner → close → 释放」）。`Drop` 从任意任务执行都视作
/// 撤销所有权。
///
/// 字段用原子而不是 `Cell`：守卫必须保持 `Send + Sync`，才能作为
/// `Device: Sync` 设备结构体的字段（v3 §5.1 硬约束）。
#[must_use = "claim_owner 守卫必须持有；Drop 即释放"]
pub struct OwnerGuard {
    name: &'static str,
    /// 绑定令牌（0 = 未绑定）。用原子而非 Cell：守卫必须保持 Sync，
    /// 才能作为 `Device: Sync` 设备结构体的字段（v3 §5.1 硬约束）。
    task: AtomicUsize,
    /// 绑定时刻的身份指纹（0 = 未知）
    fingerprint: AtomicUsize,
}

impl OwnerGuard {
    /// 登记名（诊断/测试用）
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// 任务身份令牌（诊断/测试用）；0 = 未绑定。
    pub fn task(&self) -> usize {
        self.task.load(Ordering::SeqCst)
    }

    /// 是否已绑定到具体任务（诊断/测试用）。
    pub fn is_bound(&self) -> bool {
        self.task() != 0
    }

    /// 由 **owner 任务自身**调用：把 claim(0) 绑定到真实任务。
    /// `task` 必须等于当前任务；本方法不提供“替别人绑定”的能力。
    pub fn bind(&self, task: usize) -> Result<(), DeviceError> {
        self.bind_inner(task, false)
    }

    /// 便捷：绑定到当前任务。
    pub fn bind_current(&self) -> Result<(), DeviceError> {
        let t = current_task_token().ok_or(DeviceError::InvalidInput)?;
        self.bind_inner(t, false)
    }

    /// 显式换绑（owner 重启/所有权移交）。新任务必须能访问本守卫。
    /// 换绑是信任边界操作：能拿到守卫的代码即 owner。
    pub fn rebind(&self, task: usize) -> Result<(), DeviceError> {
        self.bind_inner(task, true)
    }

    fn bind_inner(&self, task: usize, allow_rebind: bool) -> Result<(), DeviceError> {
        if task == 0 {
            return Err(DeviceError::InvalidInput);
        }
        // 身份读取 + 表写入 + 守卫原子写收进**同一段** sync::free：
        // `OwnerGuard` 是 Sync，同一守卫可被多个任务并发 bind/rebind；分两段会
        // 出现「表记 B、守卫记 A」的错配（release 匹配不到 → 槽永久泄漏）。
        // 嵌套的 sync::free 是可重入快路径（深度 +1/-1），SMP 由全局自旋串行化。
        sync::free(|_| {
            // 只允许任务绑定它自己：防冒名，并保证指纹 deref 的是活任务。
            // SAFETY: 刚进本核临界区；只读当前身份。
            let (cur, fp) = unsafe { current_identity_locked() }.ok_or(DeviceError::InvalidInput)?;
            if cur != task {
                return Err(DeviceError::InvalidInput);
            }
            let old_task = self.task.load(Ordering::SeqCst);
            let old_fp = self.fingerprint.load(Ordering::SeqCst);
            OWNERS.bind(self.name, old_task, old_fp, task, fp, allow_rebind)?;
            self.task.store(task, Ordering::SeqCst);
            self.fingerprint.store(fp, Ordering::SeqCst);
            Ok(())
        })
    }
}

impl Drop for OwnerGuard {
    fn drop(&mut self) {
        OWNERS.release(self.name, self.task.load(Ordering::SeqCst));
    }
}

/// claim 设备/总线的 owner。
///
/// - `name`：全局唯一登记名（通常取设备名或 `const OWNER`），不得为空；
/// - `task`：任务身份令牌（目标端取 [`current_task_token`]；0 表示 boot/init
///   上下文占位、不参与任务断言；host 测试可传任意显式非 0 令牌）；
/// - 同一 `name` 第二次 claim（含同一任务重复 claim）→ [`DeviceError::Busy`]；
/// - 表满 → [`DeviceError::Full`]；空名 → [`DeviceError::InvalidInput`]。
///
/// 只有 `task` 等于当前任务时才记录身份指纹（两次分开读会在抢占窗口里
/// 张冠李戴）；`task=0` 或读不到当前身份时记 0 = 未知。
pub fn claim_owner(name: &'static str, task: usize) -> Result<OwnerGuard, DeviceError> {
    if name.is_empty() {
        return Err(DeviceError::InvalidInput);
    }
    // 令牌与指纹必须同源读取：只有“入参就是当前任务”时才记录指纹。
    let fp = if task == 0 {
        0
    } else {
        // SAFETY: 刚进本核临界区；只读当前身份。
        sync::free(|_| unsafe { current_identity_locked() })
            .filter(|&(tok, _)| tok == task)
            .map(|(_, fp)| fp)
            .unwrap_or(0)
    };
    OWNERS.claim(name, task, fp)?;
    Ok(OwnerGuard {
        name,
        task: AtomicUsize::new(task),
        fingerprint: AtomicUsize::new(fp),
    })
}

/// 厂商 SDK 设备的 C2 入口越权断言（dev/host 生效）。
///
/// 用法：在驱动的 C2 方法首行 `crate::device::bus::assert_owner(OWNER);`。
/// 该 name 未 claim、task=0（init 占位）、调度器未启动或任一侧指纹未知时判为
/// 通过（无法判定）；绑定后才真正按 (令牌, 指纹) 判定。
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
    /// 事务持有者令牌（0 = 空闲；否则为任务身份令牌；`usize::MAX` = 无任务上下文）。
    /// 只用于区分「同一任务重入」（自锁 → panic）与「别的任务并发」（走共享
    /// 总线锁排队）——per-arbiter 布尔标志会把后者误报成嵌套。
    in_txn: AtomicUsize,
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
            in_txn: AtomicUsize::new(0),
        }
    }

    /// 在一个事务里执行 `f`（C2）。
    ///
    /// - **release 真实串行化**：持「同一总线名」共享的零分配任务阻塞锁；
    ///   同名 arbiter（同总线的多颗从机）互斥；
    /// - **不可嵌套**：同一任务在闭包内再调 `with_transaction` → panic（避免自锁）；
    ///   其它任务同时调同一个 arbiter 会阻塞在共享总线锁上（合法并发，不是嵌套）；
    ///   跨 arbiter 共享同一把总线锁时，同一任务在事务内重入也**立即 panic**
    ///   （拿锁前用只读 owner 快照识别；旧行为是永久自锁）；
    /// - **C2 约束**：不得在 `sync::free` / ISR 内调用（`assert_c2`）。
    pub fn with_transaction<R>(&self, f: impl FnOnce(&dyn BusDevice) -> R) -> R {
        sync::assert_c2("BusArbiter::with_transaction");
        /// 无任务上下文（调度器未启动 / 非 `xtask_executor` 构建）的哨兵令牌。
        const NO_TASK: usize = usize::MAX;
        let me = current_task_token().unwrap_or(NO_TASK);

        // 同一任务/上下文重入 = 自己等自己（普通 Mutex 不可重入）→ 立即 panic。
        // 别的任务持有同一个 arbiter 时走 `lock.lock()` 正常排队——共享总线锁
        // 的存在就是为了跨任务互斥，不能把这种合法并发误报成「嵌套」。
        // （旧实现是 per-arbiter 的 AtomicBool：T1 在事务内、或 T1 正等锁时 T2
        // 调用同一 arbiter 会被误判为嵌套而 panic——release 构建直接整机 panic。）
        assert!(
            self.in_txn.load(Ordering::SeqCst) != me,
            "BusArbiter::with_transaction 不可嵌套（同一任务重入）"
        );
        // 越权断言（dev/host）：owner 已 claim 且绑定任务时，调用者必须是该任务。
        assert_owner(self.owner);
        // 跨 arbiter 共享同一把总线锁时，同一任务重入 = 自己等自己（Mutex 不可重入）。
        // 拿锁前用只读 owner 快照识别：只有“本任务已持有”才 panic；别的任务持锁时
        // owner != me，仍走 lock() 正常排队（合法并发不受影响）。
        assert!(
            !self.lock.held_by_current(),
            "BusArbiter::with_transaction 不可嵌套：本任务已持有该总线的共享锁（跨 arbiter 同总线重入会自锁）"
        );

        struct Txn<'a>(&'a AtomicUsize, usize);
        impl Drop for Txn<'_> {
            fn drop(&mut self) {
                // 持锁期间不会有别人登记；CAS 只清本任务令牌（幂等/防御）。
                let _ =
                    self.0
                        .compare_exchange(self.1, 0, Ordering::SeqCst, Ordering::SeqCst);
            }
        }

        let _g = self.lock.lock();
        // 拿到共享总线锁 = 独占事务：登记持有者令牌（在等锁的任务尚未登记）。
        self.in_txn.store(me, Ordering::SeqCst);
        let _txn = Txn(&self.in_txn, me);
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
    use crate::task::Task;
    use core::ffi::c_void;
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

    /// §5.15.1 验收①的 host 半边：事务期间必须真实持有那把**共享总线锁**
    /// （release 串行化的唯一来源），事务结束必须释放。
    /// 真正的「两个任务争同一总线不交错」需要调度器，只能在目标/QEMU 上验证
    /// （host 无调度器，lock_core 的两方竞争会走 block() 而 host 无当前任务）。
    #[test]
    fn with_transaction_holds_shared_bus_lock() {
        let bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let lock = test_lock();
        let arb = BusArbiter::new(&bus, "bus.test.lock", lock);
        arb.with_transaction(|_| {
            assert!(
                lock.try_lock().is_none(),
                "事务闭包执行期间必须持有共享总线锁"
            );
        });
        assert!(
            lock.try_lock().is_some(),
            "事务结束（含 panic 展开）必须释放共享总线锁"
        );
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
            t.claim(n, 1, 0).unwrap();
        }
        assert!(matches!(
            t.claim(NAMES[OWNER_CAP], 1, 0),
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
        let e = r.unwrap_err();
        let msg = e
            .downcast_ref::<&str>()
            .map(|s| *s)
            .or_else(|| e.downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("");
        assert!(
            msg.contains("不可嵌套"),
            "必须是嵌套断言触发（host 的 park_locked 也可能 panic，不能只判 is_err）：{msg}"
        );
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

    // ==================== owner 绑定 / 身份代次 ====================
    // 每个用例用互不相同的登记名；线程本地 token/fp 用完复位（每个测试线程
    // 本来也是独立的，复位只为可读性与防御）。

    /// 未绑定（claim(0)）时无法判定 → 对任何令牌放行；释放后同样放行。
    #[test]
    fn unbound_claim_passes_any_task() {
        const N: &str = "bus.test.unbound";
        let g = claim_owner(N, 0).unwrap();
        set_test_token(Some(11));
        assert_owner(N);
        set_test_token(Some(22));
        assert_owner(N);
        drop(g);
        assert_owner(N); // 已释放 → 无法判定
        set_test_token(None);
    }

    /// 绑定后 assert_owner 与 with_transaction 都真正生效（非 owner 触发断言）。
    #[test]
    fn bind_enforces_owner() {
        const N: &str = "bus.test.bind-enforce";
        let bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let arb = BusArbiter::new(&bus, N, test_lock());
        set_test_token(Some(11));
        set_test_fp(Some(0xA1));
        let g = claim_owner(N, 0).unwrap();
        g.bind(11).unwrap();
        assert!(g.is_bound());
        assert_eq!(g.task(), 11);
        assert_owner(N);
        arb.with_transaction(|_| ());
        set_test_token(Some(22));
        let r = std::panic::catch_unwind(|| assert_owner(N));
        assert!(r.is_err(), "绑定后非 owner 必须触发越权断言");
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            arb.with_transaction(|_| ());
        }));
        assert!(r.is_err(), "绑定后非 owner 调用 C2 必须触发断言");
        set_test_token(None);
        set_test_fp(None);
        drop(g);
    }

    /// bind 只能绑定调用者自己；无任务上下文 / 别人的令牌 / 0 一律拒绝。
    #[test]
    fn bind_requires_current_task() {
        const N: &str = "bus.test.bind-current-task";
        let g = claim_owner(N, 0).unwrap();
        assert!(matches!(g.bind(11), Err(DeviceError::InvalidInput)));
        set_test_token(Some(12));
        assert!(matches!(g.bind(11), Err(DeviceError::InvalidInput)));
        assert!(matches!(g.bind(0), Err(DeviceError::InvalidInput)));
        g.bind(12).unwrap();
        set_test_token(None);
        drop(g);
    }

    /// 登记已被释放（不存在）时 bind 返回 Busy，且 assert_owner 放行（无法判定）。
    #[test]
    fn bind_rejects_released_claim() {
        const N: &str = "bus.test.bind-released";
        let g = claim_owner(N, 0).unwrap();
        OWNERS.release(N, 0);
        set_test_token(Some(11));
        assert!(matches!(g.bind(11), Err(DeviceError::Busy)));
        assert_owner(N);
        set_test_token(None);
        drop(g); // Drop 幂等
        assert_eq!(OWNERS.name_of(N), None);
    }

    /// rebind 把所有权移交给新任务；旧任务随即判非 owner。
    #[test]
    fn rebind_transfers_owner() {
        const N: &str = "bus.test.rebind";
        set_test_token(Some(11));
        set_test_fp(Some(0xA1));
        let g = claim_owner(N, 0).unwrap();
        g.bind(11).unwrap();
        set_test_token(Some(12));
        set_test_fp(Some(0xB2));
        g.rebind(12).unwrap();
        assert_owner(N);
        set_test_token(Some(11));
        let r = std::panic::catch_unwind(|| assert_owner(N));
        assert!(r.is_err(), "换绑后旧任务必须判非 owner");
        assert!(matches!(g.rebind(0), Err(DeviceError::InvalidInput)));
        set_test_token(Some(12));
        assert!(matches!(g.rebind(21), Err(DeviceError::InvalidInput)));
        set_test_token(None);
        set_test_fp(None);
        drop(g);
    }

    /// 同一令牌、不同指纹 = 地址复用：判非 owner；任一侧未知则退回令牌判定。
    #[test]
    fn fingerprint_detects_address_reuse() {
        const N: &str = "bus.test.addr-reuse";
        set_test_token(Some(11));
        set_test_fp(Some(0xA1));
        let g = claim_owner(N, 0).unwrap();
        g.bind(11).unwrap();
        assert_owner(N);
        set_test_fp(Some(0xA2));
        let r = std::panic::catch_unwind(|| assert_owner(N));
        assert!(r.is_err(), "同令牌不同指纹（地址复用）必须判非 owner");
        set_test_fp(None); // 当前侧未知 → 无法判定，放行
        assert_owner(N);
        set_test_token(None);
        drop(g);
    }

    /// 地址复用的任务即便拿到守卫调 bind 也不能洗白指纹。
    #[test]
    fn bind_rejects_address_reuse() {
        const N: &str = "bus.test.bind-reuse";
        set_test_token(Some(11));
        set_test_fp(Some(0xA1));
        let g = claim_owner(N, 0).unwrap();
        g.bind(11).unwrap();
        set_test_fp(Some(0xA2));
        assert!(matches!(g.bind(11), Err(DeviceError::Busy)));
        assert_eq!(g.task(), 11, "失败的 bind 不得改守卫");
        let r = std::panic::catch_unwind(|| assert_owner(N));
        assert!(r.is_err(), "失败的 bind 不得放宽登记指纹");
        set_test_token(None);
        set_test_fp(None);
        drop(g);
    }

    /// 同一令牌、同一指纹：重复 bind 幂等成功（bind_current 同款路径）。
    #[test]
    fn bind_idempotent_same_task() {
        const N: &str = "bus.test.bind-idem";
        set_test_token(Some(11));
        set_test_fp(Some(0xA1));
        let g = claim_owner(N, 0).unwrap();
        g.bind(11).unwrap();
        g.bind(11).unwrap();
        g.bind_current().unwrap();
        assert_owner(N);
        set_test_token(None);
        set_test_fp(None);
        drop(g);
    }

    /// 指纹区分活任务：同一任务两次相等、两个任务不等（地址复用判别的前提）。
    #[test]
    fn task_fingerprint_stable_and_distinct() {
        fn dummy_entry(_args: *mut c_void) {}
        // SAFETY: Task::new 返回未释放的活任务（host 端口 mock 保存上下文）；
        // 指纹只读创建后不可变的字段。
        let a = unsafe { Task::new("pi", 128, 8, dummy_entry, core::ptr::null_mut()) };
        let b = unsafe { Task::new("pi", 128, 8, dummy_entry, core::ptr::null_mut()) };
        let (fa1, fa2, fb) =
            unsafe { (task_fingerprint(a), task_fingerprint(a), task_fingerprint(b)) };
        assert_eq!(fa1, fa2, "同一任务两次指纹必须相等");
        assert_ne!(fa1, fb, "不同任务指纹必须不同（栈地址不同）");
        assert_ne!(fa1, 0, "0 保留为“未知”，活任务指纹不得为 0");
        // SAFETY: 回收 Task::new 建出的活任务（Task 的 Drop 会一并释放任务栈）。
        unsafe {
            drop(Box::from_raw(a));
            drop(Box::from_raw(b));
        }
    }

    // ==================== 跨 arbiter 共享总线锁的自嵌套拦截 ====================

    /// 两个 arbiter 共享同一把总线锁：任务在 A 事务内调 B 会立即 panic 而非自锁；
    /// 展开后共享锁已释放，B 之后可单独使用。
    #[test]
    fn cross_arbiter_shared_bus_lock_reentry_panics() {
        let a_bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let b_bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let lock = test_lock();
        let a = BusArbiter::new(&a_bus, "bus.test.shared.a", lock);
        let b = BusArbiter::new(&b_bus, "bus.test.shared.b", lock);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            a.with_transaction(|_| {
                b.with_transaction(|_| ());
            })
        }));
        let e = r.unwrap_err();
        let msg = e
            .downcast_ref::<&str>()
            .map(|s| *s)
            .or_else(|| e.downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("");
        assert!(
            msg.contains("不可嵌套"),
            "必须命中跨 arbiter 的持有者断言（host 的 park_locked 也可能 panic）：{msg}"
        );
        assert!(lock.try_lock().is_some(), "展开后共享总线锁必须已释放");
        b.with_transaction(|_| ()); // 之后单独调用正常
    }

    /// 不同的总线锁不得误判为嵌套。
    #[test]
    fn distinct_bus_locks_not_misjudged() {
        let a_bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let b_bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let a = BusArbiter::new(&a_bus, "bus.test.distinct.a", test_lock());
        let b = BusArbiter::new(&b_bus, "bus.test.distinct.b", test_lock());
        a.with_transaction(|_| {
            b.with_transaction(|_| ()); // 不同锁：合法，不 panic
        });
    }

    /// 共享锁的顺序复用（两段事务先后执行）不受影响。host 单线程，真并发
    /// 留给目标/QEMU 验证。
    #[test]
    fn shared_bus_lock_sequential_reuse_ok() {
        let a_bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let b_bus = MockBus {
            xfers: AtomicUsize::new(0),
        };
        let lock = test_lock();
        let a = BusArbiter::new(&a_bus, "bus.test.seq.a", lock);
        let b = BusArbiter::new(&b_bus, "bus.test.seq.b", lock);
        a.with_transaction(|_| ());
        b.with_transaction(|_| ());
        a.with_transaction(|_| ());
    }
}
