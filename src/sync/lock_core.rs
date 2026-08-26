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
//! 3. **PI 链**。阻塞时沿"等锁链"把每个持有者抬到不低于等待者的优先级:
//!    等待者 →(在等 LockCore)持有者 →(持有者若也在等别的锁)那个锁的
//!    持有者 → …传播值取途中最急者(数字最小)。链的边由 `Task.blocked_lock`
//!    记录——只在互斥锁阻塞时设置,普通信号量不设(信号量无持有者,
//!    PI 链止于此,与 FreeRTOS/Zephyr 一致)。
//!
//! **已知局限**(与 FreeRTOS/Zephyr 经典实现同款,书稿第 8 章有专述):
//! 释放时持有者优先级回落到 `base_priority`,若它此时还持有**另一把**锁
//! 且那锁有更高优先级的等待者,这笔继承会被暂时丢掉(等待者只会在被
//! 唤醒重试时才补抬)。完整语义需要 per-task 持锁集合 + 全链重算,
//! 是留给读者的延伸题。

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

/// 互斥内核:持有者 + 递归深度 + 优先级序等待队列。
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
}

impl LockCore {
    /// 常量构造:`VecDeque::new` 是 const,引擎零运行时初始化、零堆分配
    /// (信号量门闩方案的 Arc+分配被结构性消除)
    pub(crate) const fn new() -> Self {
        Self {
            owner: None,
            depth: 0,
            waiters: VecDeque::new(),
        }
    }

    /// 当前执行上下文身份(裸指针):调度器起跑前/host 为 null——"唯一上下文"
    #[inline]
    pub(crate) fn me() -> *mut Task {
        unsafe { xworker::current_ptr() }
    }

    /// 认领/加深(调用方在 `sync::free` 内)。
    fn claim_locked(&mut self, me: *mut Task, reentrant: bool) -> Claim {
        match self.owner {
            None => {
                self.owner = Some(me);
                self.depth = 1;
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
    /// PI 抬链 + 按优先级入队 + 挂起。三步与认领的失败判定**同一临界区**,
    /// 不存在"判定失败后、入队前对方释放"的窗口——与信号量"试计数+入队
    /// 挂起同区"是同一纪律(书稿踩坑 2 的丢失唤醒在这条纪律下不可能发生)。
    unsafe fn park_locked(&mut self, me: *mut Task) {
        debug_assert!(!me.is_null(), "阻塞路径只能在真任务上下文中走(host 单身份永远可重入)");
        if let Some(owner) = self.owner {
            (*me).blocked_lock = self as *mut LockCore;
            inherit_chain(owner, (*me).priority);
            push_priority(&mut self.waiters, me);
            (*me).block();
        }
        // owner 为 None 不可达:claim 失败只可能因 Busy,而释放也走本临界区,
        // 同区内 owner 不会从 Some 翻回 None
    }

    /// 释放一层(调用方在 `sync::free` 内):深度 -1;减到 0 才真正释放——
    /// 持有者优先级回落到出生值、账本清空、唤醒队首(最高优先级等待者)。
    /// 返回是否彻底释放(可重入锁据此决定是否唤醒)。
    unsafe fn release_locked(&mut self, me: *mut Task) -> bool {
        let Some(owner) = self.owner else {
            unreachable!("guard 存活期间账本必然记着持有者");
        };
        debug_assert_eq!(owner, me, "锁只能由持有者自己释放(guard 不可跨任务移交)");
        self.depth -= 1;
        if self.depth > 0 {
            return false; // 仍持有(外层还在锁内),不回落不唤醒
        }
        // 继承回落:恢复出生优先级。**已知局限**:持有者若还持有另一把锁且
        // 那锁有更高优先级的等待者,这里回落会暂时丢掉那笔继承——等待者
        // 重试拿锁时才是下一轮抬升(书稿"互斥量与信号量"节有专述)。
        if !owner.is_null() {
            (*owner).priority = (*owner).base_priority;
        }
        self.owner = None;
        if let Some(waiter) = self.waiters.pop_front() {
            (*waiter).wakeup();
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
        match c.claim_locked(me, reentrant) {
            Claim::Held | Claim::Nested => {
                // 拿到锁 = 不再等任何人:清掉 PI 链的边。host(null)无任务可清
                if !me.is_null() {
                    (*me).blocked_lock = ptr::null_mut();
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
        if let Claim::Held | Claim::Nested = c.claim_locked(me, reentrant) {
            if !me.is_null() {
                (*me).blocked_lock = ptr::null_mut();
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

/// PI 链:把从持有者开始的每个链节点抬到不低于传播优先级 `p`。
/// 链:持有者 →(持有者若也阻塞在另一把互斥锁上)那个锁的持有者 → …
/// 传播值取途中最急者(`p = min(p, 各节点现有优先级)`)——节点可能已被别的
/// 等待者抬得更高,上传它就够了(这比 FreeRTOS 经典实现的"只抬直接持有者"
/// 多走一步,传递阻塞链(H→M→L)因此不会掉链)。
/// 步数上界 64:死锁环等病态链下每步至多改一次优先级字段,幂等,终止即可。
unsafe fn inherit_chain(mut cur: *mut Task, mut p: u8) {
    for _ in 0..64 {
        if cur.is_null() {
            return;
        }
        raise_one(cur, p);
        p = (*cur).priority; // 节点可能本就更高(数字更小):上传它的
        let next = (*cur).blocked_lock;
        if next.is_null() {
            return;
        }
        cur = (*(next as *mut LockCore)).owner.unwrap_or(ptr::null_mut());
    }
}

/// 抬高单个任务到 `p`(数字更小者更高;反则不动)。
/// 优先级变化的连锁反应有两处,都在这里处理:
/// - Ready(在就绪队列里):`READYQ` 下标 = 优先级-1,同一临界区换桶
///   (`set_priority`,破坏"位图⟺队列非空"不变式的窗口被临界区碾平);
/// - Suspended 且在锁等待队列里:按新优先级重排(队首必须是最高)。
unsafe fn raise_one(t: *mut Task, p: u8) {
    if (*t).priority > p {
        crate::task::scheduler::xtask::set_priority(t, p);
        if (*t).state == crate::task::State::Suspended && !(*t).blocked_lock.is_null() {
            let lc = &mut *(*t).blocked_lock;
            let ptr = t;
            let q = &mut lc.waiters;
            if q.iter().any(|&x| ptr::eq(x, ptr)) {
                q.retain(|&x| !ptr::eq(x, ptr));
                push_priority(q, ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Task;
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
            core.owner = Some(task);
            core.depth = 1;
            (*task).priority = 2; // 模拟被高优先级等待者抬到 2
            assert!(core.release_locked(task));
            assert_eq!((*task).priority, 6, "释放后必须回落到出生优先级 6");
        }
        unsafe { reclaim(&[task]) };
    }

    /// 回归(核心):PI 链沿"等锁传链"上传——持有者若也阻塞在另一把锁上,
    /// 那把锁的持有者一样被抬。H→M→L 三级是经典传递场景:
    /// H 等 M 的锁,M(持着单锁)又在等 L 的锁——L 必须被抬到 H 的级别,
    /// 否则 L 永远排在无关任务后面,H 被无限期卡住(火星探路者事故形态)。
    #[test]
    fn inherit_chain_walks_transitive_edges() {
        let h = make_task(1); // 等待者(链的源头)
        let m = make_task(5); // 中间层:持有 core1,阻塞在 core2 上
        let l = make_task(8); // 底层:持有 core2
        let mut core1 = LockCore::new();
        let mut core2 = LockCore::new();
        unsafe {
            core1.owner = Some(m);
            // 链上节点不落就绪队列:避免 set_priority 的 Ready 换桶把它们
            // 送进全局 READYQ(回收后成悬垂)。Running/Suspended 只改字段
            (*m).state = crate::task::State::Suspended;
            (*m).blocked_lock = &mut core2;
            core2.owner = Some(l);
            (*l).state = crate::task::State::Running;
            // 只有 h 是"刚阻塞在 core1 上"的等待者,传入 h 的优先级
            inherit_chain(core1.owner.unwrap(), (*h).priority);
            assert_eq!((*m).priority, 1, "直接持有者应被抬到 H 的优先级");
            assert_eq!((*l).priority, 1, "传递链上的底层持有者同样要被抬");
            // 清零边,避免回收后悬垂指针
            (*m).blocked_lock = ptr::null_mut();
        }
        unsafe { reclaim(&[h, m, l]) };
    }
}
