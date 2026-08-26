//! 互斥内核:锁的本质 = 持有者 + 优先级继承
//!
//! `Mutex<T>`/`ReentrantMutex<T>` 共用的内核。回答书稿第 8 章那个问题:
//! **互斥量为什么不能就是 max=1 的信号量?**——计数模型里没有"持有者"。
//! 没有持有者就做不了**优先级继承**(PI):高优先级任务 H 等锁时,把当前
//! 持锁的低优先级任务 L **临时抬到 H 的优先级**,让 L 尽快跑完临界区放锁;
//! 否则中优先级任务 M 会一直跑在 L 前面,L 永远没机会放锁,H 被无限期卡住
//! (1997 火星探路者事故;背景见书稿)。
//!
//! 因此互斥内核与信号量有三处分道,每处都是 PI 逼出来的:
//!
//! 1. **持有者账本**。`owner: Option<*mut Task>` 记录当前持锁任务(host/
//!    起跑前为 null 身份的哨兵 `Some(null)`)——认领、加深、清空都是引擎。
//! 2. **优先级序等待队列**。等待者按优先级降序(数字升序,同级 FIFO)入队,
//!    释放时唤醒队首即**最高优先级的等待者**。若按 FIFO 唤醒,换手后新
//!    持有者可能比余下等待者还低,而低出的那笔继承没有人再补——优先级
//!    反转会从"换手"的缝里钻回来(这是"优先级序队列"存在的全部理由)。
//! 3. **完整 PI:per-task 持锁集合 + 全链重算**。`Task` 记着自己的持锁
//!    集合(`held_locks`);每个触发点(等待者阻塞 / 释放 / 认领)都调
//!    [`recompute_inheritance`]:从"有效优先级 = max(出生值, 每把仍持锁的
//!    队首等待者优先级)"重算,并在"持有者又阻塞在另一把锁上"时沿
//!    `blocked_lock` 边上溯到不动点。升**和降**都走同一条路:
//!    释放 A 时,如果持有者还握着 B 且 B 有更高优先级等待者,那笔继承
//!    由重算保住(第一代"释放即回落出生值"在这里暂时丢继承——经典
//!    实现同款缺陷,本书第一版也是);同理,等待者的紧迫度消失(认领成功)
//!    时沿链上溯收回,链上每一层的继承都随之重算,而不是只动直接持有者。
//!    链的边由 `Task.blocked_lock` 记录——只在互斥锁阻塞时设置,普通
//!    信号量不设(信号量无持有者,PI 链止于此,与 FreeRTOS/Zephyr 一致)。
//!
//! 4. **可选 PCP(优先级天花板)**。`LockCore` 带一个 `ceiling` 字段:
//!    0 = PI 模式(默认),非 0 = 启用天花板协议——拿锁即升到天花板
//!    (规则 1,与第 3 点的重算框架合流:重算对天花板锁只看天花板)、
//!    只有"当前优先级严格优于所有**他人持锁**天花板"的任务才许拿
//!    空闲锁(规则 2,靠全局登记表 `PCP_HELD` 实现"别人手里"的视野;
//!    被拦者挂全局 `PCP_BLOCKED`, 任何天花板锁完全释放时唤醒重试)。
//!    按锁选择:同一把锁要么 PI 要么 PCP,混用则两套定理都不成立
//!    ——声明责任(天花板必须覆盖所有使用者)在调用方,书稿第 26 章。
//!
//! 已知局限只剩一个:持锁集合溢出(单任务同时持锁 > [`HELD_MAX`](crate::task::HELD_MAX),
//! 现实中不存在)时,溢出部分按"等待者重试补抬"的经典行为退化;
//! PCP 同款纪律:天花板锁同时被持超过 [`PCP_REG_CAP`] 把时规则 2 退化为
//! "查不到就不拦"(debug_assert 报警,现实中不存在)。

use crate::sync;
use crate::task::scheduler::xworker;
use crate::task::{Task, TaskQueue};
use alloc::collections::VecDeque;
use core::cell::UnsafeCell;
use core::ptr;

/// 一次认领的结局
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub(crate) enum Claim {
    /// 认领成功,成为持有者
    Held,
    /// 可重入:持有者就是当前任务,递归深度 +1
    Nested,
    /// 别人持有
    Busy,
}

/// 互斥内核:持有者 + 递归深度 + 优先级序等待队列(+ 可选优先级天花板)。
/// 所有方法都必须在 `sync::free` 临界区内调用(见各方法注释;包装方负责)。
pub(crate) struct LockCore {
    /// 当前持有者;`None` = 空闲。host 单上下文身份是 null,
    /// `Some(null)` 同样记作"已持有"(与可重入锁的账本同款哨兵约定)
    owner: Option<*mut Task>,
    /// 递归深度(可重入锁用;普通互斥锁恒为 1)
    depth: usize,
    /// 等待者队列:**优先级降序(数字升序),同级 FIFO**——释放时队首即最高
    /// 优先级等待者,新持有者的优先级必然不低于余下等待者
    waiters: TaskQueue,
    /// 优先级天花板(PCP):会用本锁的任务的最高优先级(数字最小),
    /// 声明期钉死、运行期只读——PCP 规则 1 与规则 2 的唯一输入。
    /// `0` = 未启用(纯 PI 模式);非 0 = 启用 PCP:拿锁即升到天花板,
    /// 且"当前优先级严格优于(数字严格小于)所有他人持锁的天花板"
    /// 才许拿空闲锁(见 [`pcp_ceiling_blocked`])。
    ceiling: u8,
}

impl LockCore {
    /// 常量构造:`VecDeque::new` 是 const,引擎零运行时初始化、零堆分配
    /// (信号量门闩方案的 Arc+分配被结构性消除)。PCP 未启用(纯 PI)。
    pub(crate) const fn new() -> Self {
        Self {
            owner: None,
            depth: 0,
            waiters: VecDeque::new(),
            ceiling: 0,
        }
    }

    /// PCP 构造:`ceiling` 是本锁的天花板(取值 1..=16,优先级 1 最高)——
    /// 必须**覆盖所有实际使用者**(任何使用者优先级数字 ≥ ceiling),
    /// 漏标的后果是协议两条定理失效,引擎按声明行事、不纠正声明
    /// (书稿第 26 章"声明责任")。0 用 [`new`](Self::new)(纯 PI)。
    pub(crate) const fn with_ceiling(ceiling: u8) -> Self {
        assert!(
            ceiling >= 1 && ceiling <= 16,
            "PCP 天花板取值域 1..=16(数字小=优先级高,与调度器一致)"
        );
        Self {
            owner: None,
            depth: 0,
            waiters: VecDeque::new(),
            ceiling,
        }
    }

    /// 当前执行上下文身份(裸指针):调度器起跑前/host 为 null——"唯一上下文"
    #[inline]
    pub(crate) fn me() -> *mut Task {
        unsafe { xworker::current_ptr() }
    }

    fn claim_locked(&mut self, me: *mut Task, reentrant: bool) -> Claim {
        match self.owner {
            None => {
                self.owner = Some(me);
                self.depth = 1;
                // 完整 PI 的记账:新持有者把本锁压入持锁集合(可重入加深不重复)
                if !me.is_null() {
                    // SAFETY: 认领成功的 me 是当前执行身份,持有者语义有效
                    unsafe { (*me).held_push(self as *mut LockCore) };
                }
                // PCP 规则 1 的登记:天花板锁进全局登记表(规则 2 的"别人手里"
                // 视野);PI 锁内部判断 ceiling==0 直接跳过——不触碰全局
                // SAFETY: 本函数运行在 sync::free 临界区内(调方契约),
                // 登记表的一切访问都在同一临界区,无并发借用
                unsafe { pcp_register(self as *mut LockCore, me) };
                Claim::Held
            }
            Some(o) if o == me && reentrant => {
                self.depth += 1;
                Claim::Nested
            }
            Some(o) if o == me => Claim::Busy, // 普通互斥锁:嵌套即自死锁(可重入锁解决)
            _ => Claim::Busy,
        }
    }

    /// 阻塞路径(调用方在 `sync::free` 内,`claim_locked` 刚失败):
    /// PI 重算 + 按优先级入队 + 挂起。三步与认领的失败判定**同一临界区**,
    /// 不存在"判定失败后、入队前对方释放"的窗口——与信号量"试计数+入队
    /// 挂起同区"是同一纪律(书稿踩坑 2 的丢失唤醒在这条纪律下不可能发生)。
    /// 入队**先于**重算:新等待者可能是队首(最高优先级),持有者的
    /// 继承优先级要从队列现状重算。
    unsafe fn park_locked(&mut self, me: *mut Task) {
        debug_assert!(!me.is_null(), "阻塞路径只能在真任务上下文中走(host 单身份永远可重入)");
        if let Some(owner) = self.owner {
            (*me).blocked_lock = self as *mut LockCore;
            push_priority(&mut self.waiters, me);
            recompute_inheritance(owner);
            (*me).block();
        }
        // owner 为 None 不可达:claim 失败只可能因 Busy,而释放也走本临界区,
        // 同区内 owner 不会从 Some 翻回 None
    }

    /// 释放一层(调用方在 `sync::free` 内):深度 -1;减到 0 才真正释放——
    /// 从持锁集合摘掉这把锁、账本清空(若是天花板锁,同步注销 PCP 登记表),
    /// 再由**全链重算**让旧持有者的继承优先级落位,最后唤醒队首(最高优先级
    /// 等待者)。**先落位再唤醒**:唤醒动作里的抢占请求(`submit_task` →
    /// `request_preempt_if_higher`)拿"当下优先级"做比较——若先唤醒再落位,
    /// 旧持有者还带着继承来的高优先级,请求被误判为"不必抢"而吞掉,
    /// 醒来的高优先级等待者要干等下个 tick(submit_task 的抢占纪律正是为
    /// 消灭这种"unlock 换手干等"而设)。重算不是"释放即回落到出生值":
    /// 若它还持有别的锁且那锁有更高优先级的等待者,这笔继承由重算从剩余
    /// 持锁的队首等待者里取回(完整 PI)。
    /// 返回是否彻底释放(可重入锁据此决定是否唤醒)。
    unsafe fn release_locked(&mut self, me: *mut Task) -> bool {
        let Some(owner) = self.owner else {
            unreachable!("guard 存活期间账本必然记着持有者");
        };
        debug_assert_eq!(owner, me, "锁只能由持有者自己释放(guard 不可跨任务移交)");
        self.depth -= 1;
        if self.depth > 0 {
            return false; // 仍持有(外层还在锁内),不摘集合不唤醒
        }
        if !owner.is_null() {
            (*owner).held_remove(self as *mut LockCore);
        }
        self.owner = None;
        if self.ceiling != 0 {
            pcp_deregister(self as *mut LockCore);
        }
        // 重算先于唤醒:旧持有者的优先级按"剩余持锁"落位(是否仍有更高
        // 等待者取决于别的锁)——落位后唤醒动作里的抢占比较才是准的
        if !owner.is_null() {
            recompute_inheritance(owner);
        }
        if let Some(waiter) = self.waiters.pop_front() {
            (*waiter).wakeup();
        }
        // PCP 规则 3:天花板锁的完全释放可能清除"天花板阻塞"条件(规则 2
        // 只随"某把已持锁被释放"改善)——唤醒全部被规则 2 拦住的任务,
        // 它们重试时由 acquire 重新裁决(可能转正/转常规等锁/再被拦)。
        if self.ceiling != 0 {
            pcp_wake_blocked();
        }
        true
    }

    /// 测试专用:读当前递归深度(0 = 空闲)。host 单身份永远可重入,
    /// 深度探针直接读账本以观察"是否已释放"。
    #[cfg(test)]
    pub(crate) fn test_depth(&self) -> usize {
        self.depth
    }
}

/// 加锁循环的内核一步:同一临界区内"认领或(PI + 入队 + 挂起)"。
/// 返回 true = 已持有;false = 本任务已挂起,调用方 `yield_now` 后重试。
pub(crate) fn acquire(core: &UnsafeCell<LockCore>, reentrant: bool) -> bool {
    let me = LockCore::me();
    sync::free(|_| unsafe {
        let c = &mut *core.get();
        // PCP 规则 2:锁空闲时也要先过资格检查——"别人手里"的天花板锁比
        // 我还紧(数字更小或相等)就拦住,哪怕 L 是空的(PCP 与 PI 的分水岭;
        // 书稿第 26 章)。拦住的任务挂全局天花板阻塞队列,等任何天花板锁
        // 完全释放时被唤醒重试。host 单身份(null)恒放行(与单身份恒可重入同约)。
        if pcp_admission_denied(c, me) {
            // me 必非空:null 身份在 pcp_ceiling_blocked 入口即放行,走不到这里
            (*me).blocked_lock = core.get();
            PCP_BLOCKED.push_back(me);
            (*me).block();
            return false;
        }
        match c.claim_locked(me, reentrant) {
            Claim::Held | Claim::Nested => {
                // 拿到锁 = 不再等任何人:清掉 PI 链的边。host(null)无任务可清
                if !me.is_null() {
                    (*me).blocked_lock = ptr::null_mut();
                    // 完整 PI + PCP 规则 1:认领可能发生在"释放唤醒队首、队首
                    // 还没认领"的挥舞窗口(barging)——新持有者必须被抬到队首的
                    // 级别,否则反转从换手缝钻回。重算以持锁集合为输入
                    // (PI 锁看队首等待者、天花板锁看天花板),其余场景幂等空转
                    recompute_inheritance(me);
                }
                true
            }
            Claim::Busy => {
                c.park_locked(me);
                false
            }
        }
    })
}

/// 非阻塞认领(try_lock 用):只在临界区内尝试,失败**不挂起**直接返回 false。
pub(crate) fn try_acquire(core: &UnsafeCell<LockCore>, reentrant: bool) -> bool {
    let me = LockCore::me();
    sync::free(|_| unsafe {
        let c = &mut *core.get();
        // PCP 规则 2 的非阻塞版:资格不够即失败(不登记、不挂起——try 语义)
        if pcp_admission_denied(c, me) {
            return false;
        }
        if let Claim::Held | Claim::Nested = c.claim_locked(me, reentrant) {
            if !me.is_null() {
                (*me).blocked_lock = ptr::null_mut();
                recompute_inheritance(me);
            }
            true
        } else {
            false
        }
    })
}

/// 释放一层(guard 析构调用)。内部临界区,持有者身份检查含在内。
pub(crate) fn release(core: &UnsafeCell<LockCore>) {
    sync::free(|_| unsafe {
        let c = &mut *core.get();
        c.release_locked(LockCore::me());
    });
}

// ==================== PCP(优先级天花板协议)基础设施 ====================
// 规则 2 需要"别人手里"的全球视野:每把锁只看自己,看不到别的锁——
// 于是天花板锁在认领成功时进全局登记表、彻底释放时注销;规则 2 的
// 判定与"天花板阻塞者"的唤醒都围绕它。全部访问都在 `sync::free` 内。

/// 登记表容量:同时被持有的天花板锁上限。32 把同步临界区在现实内核里
/// 极其罕见;溢出时 debug_assert 并放弃登记(规则 2 退化为"查不到就不拦"
/// ——与"持锁集合溢出"同一纪律:不可能发生,发生即声明纪律被破坏)。
const PCP_REG_CAP: usize = 32;

/// 登记项:锁 + 当前持有者。持有者用于"排除自己的锁"——规则 2 只查
/// 别人持有的锁(自己刚拿的锁不算,否则每把锁都把自己拦死)。
#[derive(Clone, Copy)]
struct PcpEntry {
    core: *mut LockCore,
    owner: *mut Task,
}

/// 当前被持有的天花板锁(ceiling != 0)登记表
static mut PCP_HELD: [PcpEntry; PCP_REG_CAP] =
    [PcpEntry { core: ptr::null_mut(), owner: ptr::null_mut() }; PCP_REG_CAP];
/// 登记表已用格数
static mut PCP_HELD_COUNT: usize = 0;

/// 天花板阻塞者队列:被规则 2 拦住的任务(锁空闲但资格不够)挂这里,
/// **不属于任何锁的等待队列**——它等的条件在别人的锁上。"唤醒点":
/// 任何天花板锁被完全释放时清空本队列(见 `release_locked`)。
static mut PCP_BLOCKED: TaskQueue = VecDeque::new();

/// 登记:认领成功(深度 0→1)的天花板锁入表——规则 2 视野的"写"。
unsafe fn pcp_register(core: *mut LockCore, owner: *mut Task) {
    if (*core).ceiling == 0 {
        return; // PI 锁不参与 PCP:不碰全局(host 并发跑的既有测试零影响)
    }
    if PCP_HELD_COUNT >= PCP_REG_CAP {
        debug_assert!(false, "PCP 登记表溢出(> {PCP_REG_CAP} 把天花板锁同时被持有)");
        return;
    }
    PCP_HELD[PCP_HELD_COUNT] = PcpEntry { core, owner };
    PCP_HELD_COUNT += 1;
}

/// 注销:彻底释放(深度减到 0)的天花板锁出表。swap_remove,顺序无关。
unsafe fn pcp_deregister(core: *mut LockCore) {
    for i in 0..PCP_HELD_COUNT {
        if ptr::eq(PCP_HELD[i].core, core) {
            PCP_HELD_COUNT -= 1;
            PCP_HELD[i] = PCP_HELD[PCP_HELD_COUNT];
            PCP_HELD[PCP_HELD_COUNT] = PcpEntry { core: ptr::null_mut(), owner: ptr::null_mut() };
            return;
        }
    }
}

/// 规则 2 判定:`me` 想拿一把**空闲的**天花板锁,此刻是否存在"别的任务
/// 持有"的天花板锁,其天花板**严格优于**(数字 **小于或等于**,严格版规则:
/// "必须低于所有他人持锁天花板"的否定形式 = 存在 ≤)me 的当前优先级。
/// 存在 → 拦住(天花板阻塞,哪怕目标锁是空的)。只数天花板锁(ceiling != 0);
/// 自己持的锁不算;host 单身份(null)恒放行。
/// 登记表不变式:`0..PCP_HELD_COUNT` 内恒为"非空锁 + ceiling != 0"的
/// 登记项(register 只写天花板锁且先写格后计数;deregister swap_remove
/// 只清尾格)——判定扫描不再逐项复核空值与天花板取值。
unsafe fn pcp_ceiling_blocked(me: *mut Task) -> bool {
    if me.is_null() {
        return false;
    }
    let p = (*me).priority;
    for i in 0..PCP_HELD_COUNT {
        let e = &PCP_HELD[i];
        if !ptr::eq(e.owner, me) && (*e.core).ceiling <= p {
            return true;
        }
    }
    false
}

/// 规则 2 的准入裁决(acquire 的挂起版与 try_acquire 的失败版共用同一道闸):
/// 目标锁空闲、启用天花板、且规则 2 判定被拦——三个条件缺一不可。
/// (锁非空闲走常规等锁;未启用天花板是纯 PI;资格够则放行。)
unsafe fn pcp_admission_denied(c: &LockCore, me: *mut Task) -> bool {
    c.owner.is_none() && c.ceiling != 0 && unsafe { pcp_ceiling_blocked(me) }
}

/// 规则 3 的唤醒点:任何天花板锁完全释放都可能清除天花板阻塞条件——
/// 清空全局阻塞队列、全员唤醒;重试时由 acquire 重新裁决
/// (转正/转常规等锁/再被拦,三种结局由重试天然消化)。
unsafe fn pcp_wake_blocked() {
    while let Some(t) = PCP_BLOCKED.pop_front() {
        (*t).wakeup();
    }
}

/// 按优先级降序(数字升序)插入:插到第一个**更低**(数字更大)的等待者
/// 前面——队首恒为最高;没有更低者则排尾。同级保持 FIFO(后到者排已有之后)。
unsafe fn push_priority(q: &mut TaskQueue, t: *mut Task) {
    let p = (*t).priority;
    let pos = q
        .iter()
        .position(|&x| unsafe { (*x).priority > p })
        .unwrap_or(q.len());
    q.insert(pos, t);
}

/// 等锁链级联步数上界(老 `inherit_chain` 的 64 同款纪律):死锁环等病态链
/// 下优先级字段每次落位后要么收敛要么被后续步覆写,幂等,到界即停;
/// 正常链远小于此(n 个任务等锁链至多 n 层,每层至多重算几次)
const MAX_INHERIT_STEPS: usize = 256;

/// 一个任务此刻的**有效优先级** = max(出生优先级, 仍持有的每一把锁的
/// 抬升来源)。PI 锁:队首等待者优先级(队首 = 最高优先级等待者,
/// 等待队列按优先级降序,等待者的 `priority` 字段由不变式保证等于它自己
/// 的有效优先级——传递继承因此自动折叠进"队首扫描",不需要单独走链);
/// **PCP 锁:只看天花板,不看谁在等**——任何使用者的优先级都不会优于
/// 天花板(声明责任,书稿第 26 章),规则 1 的"拿锁即升"由此与 PI 的
/// 重算框架合流:认领成功/释放时同一条 `recompute_inheritance` 路径
/// 按持锁集合逐把取来源。两种来源取"更优"(数字更小)者。
unsafe fn compute_effective(t: *mut Task) -> u8 {
    let mut p = (*t).base_priority;
    let n = ((*t).held_count as usize).min((*t).held_locks.len());
    for i in 0..n {
        let lc = &*(*t).held_locks[i];
        if lc.ceiling != 0 {
            let c = lc.ceiling;
            if c < p {
                p = c;
            }
        } else if let Some(&head) = lc.waiters.front() {
            let hp = (*head).priority;
            if hp < p {
                p = hp;
            }
        }
    }
    p
}

/// 把任务优先级落位为 `p`(升、降都可以——完整 PI 的"重算"既抬也落),
/// 并处理两个连带事实:Ready 换就绪桶(`set_priority`,READY_BITS 不变式);
/// Suspended 且在锁等待队列里 → 按新优先级重排(队首恒为最高——
/// 它作为"在等者"的紧迫度变了,所有权者要按新现状重算)。
unsafe fn place_priority(t: *mut Task, p: u8) {
    if (*t).priority == p {
        return;
    }
    crate::task::scheduler::xtask::set_priority(t, p);
    if (*t).state == crate::task::State::Suspended && !(*t).blocked_lock.is_null() {
        let lc = &mut *(*t).blocked_lock;
        let q = &mut lc.waiters;
        if q.iter().any(|&x| ptr::eq(x, t)) {
            q.retain(|&x| !ptr::eq(x, t));
            push_priority(q, t);
        }
    }
}

/// 完整 PI:从 `start` 开始按持锁集合重算有效优先级,并在等锁链上级联
/// 直到不动点。每次落位都意味着"这个任务在它等的锁里作为等待者的紧迫度
/// 变了",那把锁的持有者必须重算(可能连带自己的持有者…),如此沿
/// `blocked_lock` 边上溯;某层重算结果没变即不动点,上层不受影响。
/// 升、降都走这条路:等待者阻塞(升)、锁释放(落)、认领成功(落,
/// 等待者的紧迫度从队列消失)都从这里进。
unsafe fn recompute_inheritance(start: *mut Task) {
    let mut cur = start;
    for _ in 0..MAX_INHERIT_STEPS {
        if cur.is_null() {
            return;
        }
        let p = compute_effective(cur);
        if p == (*cur).priority {
            return; // 不动点:本层没变,上层无感
        }
        place_priority(cur, p);
        let up = (*cur).blocked_lock;
        if up.is_null() {
            return;
        }
        cur = (*(up as *mut LockCore)).owner.unwrap_or(ptr::null_mut());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::scheduler::xtask::clear_readyq_for_test;
    use crate::task::{State, Task};
    use alloc::boxed::Box;
    use core::ffi::c_void;

    fn dummy_entry(_args: *mut c_void) {}

    fn make_task(prio: u8) -> *mut Task {
        Task::new("pi", 128, prio, dummy_entry, core::ptr::null_mut())
    }

    /// 回收任务,避免测试泄漏(Task 的 Drop 会一并释放任务栈)
    unsafe fn reclaim(ptrs: &[*mut Task]) {
        for &p in ptrs {
            drop(Box::from_raw(p));
        }
    }

    /// READYQ/READY_BITS 全局就绪态的测试串行化:会唤醒等待者的 PI 测试
    /// 共享全局就绪队列,cargo test 并行跑会并发推队列(与 PCP_TEST_GUARD
    /// 同款纪律;中毒容忍——一条测试失败不级联到兄弟测试)。
    static READYQ_TEST_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// 回归:等待队列按"优先级降序(数字升序)、同级 FIFO"排——这是 PI 的
    /// 前提(释放时队首 = 最高优先级等待者;若 FIFO 先进先出,换手后新持有者
    /// 可能低于余下等待者,反转从换手缝里钻回)。
    #[test]
    fn push_priority_orders_highest_first() {
        let a = make_task(8);
        let b = make_task(2); // 更高
        let c = make_task(8); // 与 a 同级
        let mut q = TaskQueue::new();
        unsafe {
            push_priority(&mut q, a);
            push_priority(&mut q, b);
            push_priority(&mut q, c);
        }
        let order: Vec<u8> = q.iter().map(|&t| unsafe { (*t).priority }).collect();
        assert_eq!(order, vec![2, 8, 8], "队首必须最高,同级保 FIFO");
        unsafe { reclaim(&[a, b, c]) };
    }

    /// 回归:兼容 host 单身份(null)的账本纪律——空闲认领、同身份普通锁拒绝
    /// 重入、同身份可重入锁加深、释放退层。
    #[test]
    fn claim_release_accounting_null_identity() {
        let mut core = LockCore::new();
        assert_eq!(core.claim_locked(ptr::null_mut(), false), Claim::Held);
        assert_eq!(core.claim_locked(ptr::null_mut(), false), Claim::Busy, "普通锁不许重入");
        assert_eq!(core.claim_locked(ptr::null_mut(), true), Claim::Nested, "可重入锁加深");
        assert_eq!(core.depth, 2);
        unsafe { assert!(!core.release_locked(ptr::null_mut()), "只退一层:还在最外层锁内") };
        assert_eq!(core.depth, 1);
        unsafe { assert!(core.release_locked(ptr::null_mut()), "退到 0 才真正释放") };
        assert_eq!(core.depth, 0);
        assert!(core.owner.is_none(), "两层都放完才真正空闲");
    }

    /// 阳性对照:彻底释放必须把继承来的优先级**还回去**(回落到出生值)。
    /// host 上优先级字段可直改模拟"已被抬",释放后必须还原。
    #[test]
    fn release_demotes_owner_to_base_priority() {
        let task = make_task(6);
        let mut core = LockCore::new();
        unsafe {
            (*task).state = crate::task::State::Running; // 不落就绪队列(host 无调度器)
            core.owner = Some(task);
            core.depth = 1;
            (*task).priority = 2; // 模拟被高优先级等待者抬到 2
            assert!(core.release_locked(task));
            assert_eq!((*task).priority, 6, "释放后必须回落到出生优先级 6");
        }
        unsafe { reclaim(&[task]) };
    }

    /// 回归(核心):PI 重算沿"等锁链"上传——持有者若也阻塞在另一把锁上,
    /// 那把锁的持有者一样被抬。H→M→L 三级是经典传递场景:
    /// H 等 M 的锁,M(持着单锁)又在等 L 的锁——L 必须被抬到 H 的级别,
    /// 否则 L 永远排在无关任务后面,H 被无限期卡住(火星探路者事故形态)。
    #[test]
    fn recompute_cascades_transitive_edges() {
        let h = make_task(1); // 等待者(链的源头)
        let m = make_task(5); // 中间层:持有 core1,阻塞在 core2 上
        let l = make_task(8); // 底层:持有 core2
        let mut core1 = LockCore::new();
        let mut core2 = LockCore::new();
        unsafe {
            (*h).state = State::Suspended;
            (*m).state = State::Suspended;
            (*l).state = State::Running;
            core1.owner = Some(m);
            core2.owner = Some(l);
            // 持锁集合与等待队列按不变式摆好:h ∈ core1 队、m ∈ core2 队
            (*m).held_push(&mut core1);
            (*l).held_push(&mut core2);
            (*h).blocked_lock = &mut core1;
            push_priority(&mut core1.waiters, h);
            (*m).blocked_lock = &mut core2;
            push_priority(&mut core2.waiters, m);
            // h 刚阻塞在 core1 上:从持有者起重算并上溯
            recompute_inheritance(core1.owner.unwrap());
            assert_eq!((*m).priority, 1, "直接持有者应被抬到 H 的优先级");
            assert_eq!((*l).priority, 1, "传递链上的底层持有者同样要被抬");
            // 清零边,避免回收后悬垂指针
            (*m).blocked_lock = ptr::null_mut();
            (*h).blocked_lock = ptr::null_mut();
        }
        unsafe { reclaim(&[h, m, l]) };
    }

    /// 完整 PI(第一代缺陷的回归):释放一把锁时,从**剩余持锁集合**的队首
    /// 等待者重算继承——持有者先拿 A 再拿 B,WA(2)/WB(3) 分别阻塞在
    /// 两把锁上;释放 A 后必须停在 3(B 的队首),而不是直接回落到出生值 6
    /// (经典实现在此处暂时丢掉 B 上的继承——书稿专述的已知局限)。
    #[test]
    fn release_keeps_inheritance_from_remaining_holds() {
        let _guard = READYQ_TEST_GUARD.lock().unwrap_or_else(|e| e.into_inner());
        let t = make_task(6);
        let wa = make_task(2);
        let wb = make_task(3);
        let mut a = LockCore::new();
        let mut b = LockCore::new();
        unsafe {
            (*t).state = State::Running;
            (*wa).state = State::Suspended;
            (*wb).state = State::Suspended;
            a.owner = Some(t);
            a.depth = 1;
            b.owner = Some(t);
            b.depth = 1;
            (*t).held_push(&mut a);
            (*t).held_push(&mut b);
            // WA 阻塞在 A 上(模拟 park_locked 的入队+重算)
            (*wa).blocked_lock = &mut a;
            push_priority(&mut a.waiters, wa);
            recompute_inheritance(t);
            assert_eq!((*t).priority, 2, "A 的队首 WA(2) 应把持有者抬到 2");
            // WB 阻塞在 B 上:持有者保持 2(A 的队首更急)
            (*wb).blocked_lock = &mut b;
            push_priority(&mut b.waiters, wb);
            recompute_inheritance(t);
            assert_eq!((*t).priority, 2);
            // 释放 A:唤醒 WA,重算——B 上还挂着 WB(3),继承必须停在 3
            let _ = a.release_locked(t);
            assert_eq!((*t).priority, 3, "释放 A 后:B 的队首 WB(3) 仍要继承——完整 PI 不许掉到 6");
            // 释放 B:再无持锁,才真正回落到出生值
            let _ = b.release_locked(t);
            assert_eq!((*t).priority, 6, "全部释放后必须回落到出生优先级 6");
            (*wa).blocked_lock = ptr::null_mut();
            (*wb).blocked_lock = ptr::null_mut();
        }
        // release_locked 唤醒的 WA/WB 在 host 上被推进了全局 READYQ——
        // 回收前清桶,别把悬垂指针留给后续驱动调度器的测试
        unsafe { clear_readyq_for_test() };
        unsafe { reclaim(&[t, wa, wb]) };
    }

    /// 完整 PI:继承"被收回"必须沿链级联——T(持 B、阻塞在 N 上)被
    /// WB(3) 抬起来后,Y(N 的持有者)跟着抬到 3;T 释放 B 后 T 落回 5,
    /// Y 必须跟着落到 5。"等锁链"每一层都按各自队首现状重算,而不是
    /// 只动直接持有者(经典实现的链式抬升只会上抬,下落靠等待者重试补抬,
    /// 会滞留到下一次假事件)。
    #[test]
    fn demote_cascades_up_blocked_chain() {
        let _guard = READYQ_TEST_GUARD.lock().unwrap_or_else(|e| e.into_inner());
        let t = make_task(5);
        let y = make_task(6);
        let wb = make_task(3);
        let mut b = LockCore::new();
        let mut n = LockCore::new();
        unsafe {
            (*t).state = State::Suspended;
            (*y).state = State::Running;
            (*wb).state = State::Suspended;
            b.owner = Some(t);
            b.depth = 1;
            n.owner = Some(y);
            n.depth = 1;
            (*t).held_push(&mut b);
            (*y).held_push(&mut n);
            // T 阻塞在 N 上(T 在 N 的等待者里)
            (*t).blocked_lock = &mut n;
            push_priority(&mut n.waiters, t);
            // WB 阻塞在 B 上:抬 T → 级联抬 Y
            (*wb).blocked_lock = &mut b;
            push_priority(&mut b.waiters, wb);
            recompute_inheritance(t);
            assert_eq!((*t).priority, 3, "B 的队首 WB(3) 应把 T 抬到 3");
            assert_eq!((*y).priority, 3, "T 是 N 的队首——N 的持有者 Y 应级联抬到 3");
            // T 释放 B(WB 被唤醒):T 的有效优先级重算为 5(无持锁),
            // 这回落必须沿链上传到 Y
            let _ = b.release_locked(t);
            assert_eq!((*t).priority, 5, "T 释放 B 后回落出生值 5");
            assert_eq!((*y).priority, 5, "Y 的抬升来自 T 的紧迫度——T 落了,Y 必须跟着落");
            (*t).blocked_lock = ptr::null_mut();
            (*wb).blocked_lock = ptr::null_mut();
        }
        // 同上:release_locked 唤醒的 WB 在 host 上进了全局 READYQ——先清桶再回收
        unsafe { clear_readyq_for_test() };
        unsafe { reclaim(&[t, y, wb]) };
    }

    // ---------- PCP(优先级天花板协议) ----------

    /// PCP 全局状态(登记表 PCP_HELD / 阻塞队列 PCP_BLOCKED)的测试串行化:
    /// 两条触碰登记表的测试共享全局,cargo test 并行跑必然互相干扰。
    /// (既有 PI 测试不触碰:ceiling == 0 的锁在 register/判定处即返回。)
    static PCP_TEST_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// 回归(规则 2):"别人持有"的天花板锁比我还紧(数字 ≤)就拦住——
    /// 哪怕目标锁是空的;自己持的锁不算(规则 2 只查别人);我更优时放行;
    /// 注销(彻底释放)后放行。语义依据:严格版规则"当前优先级必须严格
    /// 低于所有他人持锁天花板",其否定形式 = 存在一把他人持锁的
    /// 天花板 ≤ 我的优先级。
    #[test]
    fn pcp_rule2_blocks_when_other_ceiling_tighter() {
        // 上一条 PCP 测试若中途 panic,登记表会残留悬垂项(守卫也因此中毒)——
        // 容忍中毒、清空登记表再开始:一条失败不级联成两条
        let _guard = PCP_TEST_GUARD.lock().unwrap_or_else(|e| e.into_inner());
        unsafe { PCP_HELD_COUNT = 0 };
        let t = make_task(5);
        let holder = make_task(2);
        let mut s = LockCore::with_ceiling(2);
        unsafe {
            (*t).state = State::Running;
            s.owner = Some(holder);
            s.depth = 1;
            pcp_register(&mut s as *mut LockCore, holder);
            assert!(
                pcp_ceiling_blocked(t),
                "他人持天花板 2、我在 5:2 ≤ 5 → 必须拦(目标锁是空的也一样)"
            );
            assert!(
                !pcp_ceiling_blocked(holder),
                "持有者本人不受自己锁的天花板约束(规则 2 只查别人)"
            );
            (*t).priority = 1; // 我更优(数字 1 < 2)
            assert!(!pcp_ceiling_blocked(t), "我严格优于他人天花板 → 放行");
            (*t).priority = 5;
            pcp_deregister(&mut s as *mut LockCore);
            assert!(!pcp_ceiling_blocked(t), "无人持天花板锁 → 放行");
        }
        unsafe { reclaim(&[t, holder]) };
    }

    /// 回归(规则 1 + 登记往返):认领天花板锁 → 有效优先级 = 天花板;
    /// 彻底释放 → 回落到出生值,登记同步注销。走真实 claim/release 路径。
    #[test]
    fn pcp_claim_raises_release_falls() {
        // 同上:中毒容忍 + 进测试前清空登记表
        let _guard = PCP_TEST_GUARD.lock().unwrap_or_else(|e| e.into_inner());
        unsafe { PCP_HELD_COUNT = 0 };
        let t = make_task(6);
        let mut c = LockCore::with_ceiling(3);
        unsafe {
            (*t).state = State::Running;
            assert_eq!(c.claim_locked(t, false), Claim::Held);
            recompute_inheritance(t);
            assert_eq!((*t).priority, 3, "规则 1:拿锁即升到天花板 3");
            assert!(c.release_locked(t), "彻底释放");
            assert_eq!((*t).priority, 6, "释放后回落到出生值 6");
            assert!(!pcp_ceiling_blocked(t), "登记已注销:无人持天花板锁");
        }
        unsafe { reclaim(&[t]) };
    }

    /// 回归(规则 1 语义):天花板锁的有效优先级**只看天花板**——等待者
    /// 不参与(声明责任:任何使用者优先级数字 ≥ 天花板;漏标 = 协议前提
    /// 失守,引擎按声明行事、不纠正声明——书稿第 26 章"声明责任")。
    #[test]
    fn compute_effective_prefers_ceiling_over_waiters() {
        let t = make_task(6);
        let w = make_task(4);
        let mut c = LockCore::with_ceiling(3);
        unsafe {
            (*t).state = State::Running;
            (*w).state = State::Suspended;
            c.owner = Some(t);
            c.depth = 1;
            (*t).held_push(&mut c);
            (*w).blocked_lock = &mut c;
            push_priority(&mut c.waiters, w);
            assert_eq!(compute_effective(t), 3, "天花板(3)比队首等待者(4)更优——取天花板");
            (*w).blocked_lock = ptr::null_mut();
        }
        unsafe { reclaim(&[t, w]) };
    }
}
