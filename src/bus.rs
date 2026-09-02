//! 总线
use crate::sync;
use crate::sync::arc::Arc;

use alloc::vec::Vec;
use alloc::{boxed::Box, collections::BTreeMap};
use core::cell::{Cell, RefCell};

/// 回调统一包进 Arc:publish/call 先在**临界区内**快照克隆、解除 RefCell
/// 借用,再在**临界区外**逐个执行——回调里重入 subscribe/unsubscribe/publish/
/// register 不再撞上借用冲突(修前持 borrow_mut 执行回调,重入即
/// BorrowMutError panic = abort 停机);回调中退订自己/他人也由 Arc 保活,
/// 绝不执行已释放的闭包。
///
/// 回调在临界区外执行是本模块的关键纪律:回调是任意用户代码,绝不在
/// 关中断(单核)/持全局自旋锁(SMP)下运行——否则拖垮中断延迟(本内核
/// `Mutex` 文档明令"关中断几毫秒会打死 systick")、SMP 下持锁跑用户码还
/// 会阻塞别核一切临界区。一切 RefCell 借用(快照/注册/注销)都收进
/// `sync::free`(ISR 侧同规,ch25 ⑥),与 semaphore/queue/timer 的
/// ISR 路径一致——SMP 下不存在裸借用竞争。
type Callback<'a, E> = Arc<Box<dyn Fn(&'a str, E) + Send + 'static>>;

pub struct Bus<'a, E> {
    subscribers: RefCell<BTreeMap<&'a str, Vec<(usize, Callback<'a, E>)>>>,
    callee: RefCell<BTreeMap<&'a str, Callback<'a, E>>>,
    next_token: Cell<usize>,
}

unsafe impl<'a, E> Send for Bus<'a, E> {}

unsafe impl<'a, E> Sync for Bus<'a, E> {}

impl<'a, E> Bus<'a, E> {
    pub const fn new() -> Self {
        Self {
            subscribers: RefCell::new(BTreeMap::new()),
            callee: RefCell::new(BTreeMap::new()),
            next_token: Cell::new(0),
        }
    }
}

pub type Token<'a> = (&'a str, usize);

impl<'a, E: Clone> Bus<'a, E> {
    /// 订阅事件
    /// 不能在中断服务中调用
    pub fn subscribe<F: Fn(&'a str, E) + Send + 'static>(&self, topic: &'a str, f: F) -> Token<'a> {
        sync::free(|_| {
            let mut subscribers = self.subscribers.borrow_mut();
            //token 用单调递增的唯一 id，不能用闭包地址：
            //零大小（不捕获环境）闭包的 Box 指针是 dangling 对齐地址，彼此相同会互相误删；
            //地址在退订释放后还可能被新订阅复用（ABA），过期 token 会误删无辜订阅。
            let id = self.next_token.get();
            self.next_token.set(id + 1);
            let list = subscribers.entry(topic).or_default();
            list.push((id, Arc::new(Box::new(f))));
            (topic, id)
        })
    }
    /// 取消订阅
    /// 不能在中断服务中调用
    pub fn unsubscribe(&self, token: Token<'a>) {
        sync::free(|_| {
            let mut subscribers = self.subscribers.borrow_mut();
            if let Some(list) = subscribers.get_mut(token.0) {
                list.retain(|(id, _)| *id != token.1);
            }
        });
    }

    /// 取某 topic 的本轮订阅快照(Arc 克隆)。**只在临界区内借 RefCell、
    /// 出区即还**——回调派发放到锁外。SMP(ch25 ⑥):ISR 侧也走同一把全局锁,
    /// 与任务侧 subscribe/unsubscribe 的 `borrow_mut` 互斥,无裸借用竞争。
    fn snapshot(&self, topic: &'a str) -> Vec<Callback<'a, E>> {
        sync::free(|_| {
            let subscribers = self.subscribers.borrow();
            match subscribers.get(topic) {
                Some(list) => list.iter().map(|(_, f)| f.clone()).collect(),
                None => Vec::new(),
            }
        })
    }

    /// 发送事件(任务/中断上下文通用:快照在临界区内,回调在临界区外执行)。
    pub fn publish(&self, topic: &'a str, event: E) -> &Self {
        // 快照语义与主流 pub-sub 一致:回调中退订的自己/他人本轮仍触发
        // (Arc 保活),新订阅的本轮不触发。回调是任意用户代码,允许重入本
        // 总线的任何操作——且在临界区外执行,不关中断/不持全局自旋跑用户码
        for f in self.snapshot(topic) {
            (*f)(topic, event.clone());
        }
        self
    }

    /// 发送事件(中断服务里用;与 `publish` 同一实现——快照进锁、回调出锁,
    /// 两种上下文都安全。ISR 侧回调必须绝不停留:只做唤醒/通知)
    pub fn publish_isr(&self, topic: &'a str, event: E) -> &Self {
        self.publish(topic, event)
    }

    pub fn register<F: Fn(&'a str, E) + Send + 'static>(&self, name: &'a str, f: F) -> &Self {
        sync::free(|_| self.register_isr(name, f))
    }

    pub fn unregister(&self, name: &'a str) {
        sync::free(|_| self.unregister_isr(name))
    }

    pub fn register_isr<F: Fn(&'a str, E) + Send + 'static>(&self, name: &'a str, f: F) -> &Self {
        let f: Callback<'a, E> = Arc::new(Box::new(f));
        // ch25 ⑥:借用收进临界区(嵌套安全),与任务侧 register 同一把锁
        sync::free(|_| {
            self.callee.borrow_mut().insert(name, f);
        });
        self
    }

    pub fn unregister_isr(&self, name: &'a str) {
        sync::free(|_| {
            self.callee.borrow_mut().remove(name);
        });
    }

    pub fn call(&self, name: &'a str, event: E) -> &Self {
        // 直接委托:快照/回调的临界区内切分由 call_isr 完成,这里不再多包
        // 一层 free(否则回调又被裹进外层临界区,失去"回调出锁"的意义)
        self.call_isr(name, event)
    }

    pub fn call_isr(&self, name: &'a str, event: E) -> &Self {
        // 与 publish 同款:临界区内只克隆 Arc(解除借用),回调在临界区外
        // 执行——回调里重入 register/unregister/call 不再撞上借用冲突,也不
        // 在持锁/关中断下跑用户码
        let f = sync::free(|_| self.callee.borrow().get(name).cloned());
        if let Some(f) = f {
            (*f)(name, event);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Bus;
    use alloc::sync::Arc;
    use core::sync::atomic::{AtomicUsize, Ordering};

    /// 回归：unsubscribe 用 token 必须能精确移除目标订阅（bug #4）。
    /// 修复前 subscribe 把闭包二次装箱，token 取的是内层地址、unsubscribe 比的是外层地址，
    /// 两者永不相等 → 退订静默失效、回调泄漏。修复后同一口径，退订精确生效。
    #[test]
    fn unsubscribe_removes_only_the_target() {
        let bus = Bus::<u32>::new();
        let a = Arc::new(AtomicUsize::new(0));
        let b = Arc::new(AtomicUsize::new(0));

        // 第一个订阅走 or_default 新建分支，第二个走已有列表分支，
        // 修复前两分支装箱口径还不一致，这里两条都要覆盖。
        let ta = bus.subscribe("topic", {
            let a = a.clone();
            move |_, _| {
                a.fetch_add(1, Ordering::SeqCst);
            }
        });
        bus.subscribe("topic", {
            let b = b.clone();
            move |_, _| {
                b.fetch_add(1, Ordering::SeqCst);
            }
        });

        bus.unsubscribe(ta);
        bus.publish("topic", 1);

        assert_eq!(a.load(Ordering::SeqCst), 0, "已退订的回调不应再触发");
        assert_eq!(b.load(Ordering::SeqCst), 1, "未退订的回调应正常触发");
    }

    /// 回归：重复退订不 panic、不影响其他订阅。
    #[test]
    fn unsubscribe_is_idempotent() {
        let bus = Bus::<u32>::new();
        let hits = Arc::new(AtomicUsize::new(0));
        let token = bus.subscribe("t", {
            let h = hits.clone();
            move |_, _| {
                h.fetch_add(1, Ordering::SeqCst);
            }
        });
        bus.unsubscribe(token);
        bus.unsubscribe(token); // 二次退订
        bus.publish("t", 0);
        assert_eq!(hits.load(Ordering::SeqCst), 0);
    }

    /// 回归：零大小（不捕获环境）闭包的 token 也必须唯一。
    /// 按地址取 token 时，ZST 闭包的 Box 不分配内存、指针是相同的 dangling 对齐地址，
    /// 两个 ZST 订阅 token 相同，退订一个会把另一个也删掉。id token 下必须互不误删。
    #[test]
    fn unsubscribe_distinguishes_zero_sized_callbacks() {
        static HITS_A: AtomicUsize = AtomicUsize::new(0);
        static HITS_B: AtomicUsize = AtomicUsize::new(0);
        let bus = Bus::<u32>::new();
        let ta = bus.subscribe("t", |_, _| {
            HITS_A.fetch_add(1, Ordering::SeqCst);
        });
        bus.subscribe("t", |_, _| {
            HITS_B.fetch_add(1, Ordering::SeqCst);
        });

        bus.unsubscribe(ta);
        bus.publish("t", 1);

        assert_eq!(
            HITS_A.load(Ordering::SeqCst),
            0,
            "已退订的 ZST 回调不应再触发"
        );
        assert_eq!(
            HITS_B.load(Ordering::SeqCst),
            1,
            "未退订的 ZST 回调不应被误删"
        );
    }

    /// 回归:回调重入总线不得 panic。修前 publish_isr 持 RefCell 可变
    /// 借用执行回调,回调里 subscribe/unsubscribe/publish 立即
    /// BorrowMutError——no_std 目标上 panic=abort,内核停机。
    /// 快照修复后:重入安全;回调里退订自己,下一轮不再触发;
    /// 同 topic 的其他订阅者本轮照常触发。
    #[test]
    fn publish_callback_may_reenter_bus() {
        static SELF_ID: AtomicUsize = AtomicUsize::new(0);
        static SELF_HITS: AtomicUsize = AtomicUsize::new(0);
        static OTHER: AtomicUsize = AtomicUsize::new(0);
        let bus: &'static Bus<u32> = Box::leak(Box::new(Bus::new()));

        // 回调里"退订自己 + 向别的 topic 再发一个事件"(pub-sub 典型用法)
        // (id 从 0 开始,存 t.1+1 避免哨兵 0 与首个合法 token 撞车)
        let t = bus.subscribe("a", move |_, _| {
            SELF_HITS.fetch_add(1, Ordering::SeqCst);
            let id = SELF_ID.swap(0, Ordering::SeqCst);
            if id != 0 {
                bus.unsubscribe(("a", id - 1)); // 退订自己(修前此处 panic)
            }
            bus.publish("b", 7); // 重入发布(修前此处 panic)
        });
        SELF_ID.store(t.1 + 1, Ordering::SeqCst);
        bus.subscribe("a", |_, _| {
            OTHER.fetch_add(1, Ordering::SeqCst);
        });
        let heard_b = Arc::new(AtomicUsize::new(0));
        bus.subscribe("b", {
            let h = heard_b.clone();
            move |_, e| {
                assert_eq!(e, 7);
                h.fetch_add(1, Ordering::SeqCst);
            }
        });

        bus.publish("a", 1); // 第一次:自退订回调 + other,并转发 b
        bus.publish("a", 1); // 第二次:自退订回调已摘除,只剩 other

        assert_eq!(SELF_HITS.load(Ordering::SeqCst), 1, "自退订后不应再触发");
        assert_eq!(OTHER.load(Ordering::SeqCst), 2, "other 两次都应在");
        assert_eq!(heard_b.load(Ordering::SeqCst), 1, "b 只被转发一次");
    }

    /// 回归:call 的回调里重入 register/unregister 不得 panic
    /// (修前 call_isr 持 borrow() 执行回调,重入 borrow_mut 即 panic)
    #[test]
    fn call_callback_may_reenter_bus() {
        let bus: &'static Bus<u32> = Box::leak(Box::new(Bus::new()));
        let hits = Arc::new(AtomicUsize::new(0));
        bus.register("x", {
            let h = hits.clone();
            move |_, _| {
                h.fetch_add(1, Ordering::SeqCst);
                bus.unregister("y"); // 重入:借用冲突在修前直接 panic
            }
        });
        bus.register("y", |_, _| {});
        bus.call("x", 0);
        bus.call("y", 0); // y 已被 unregister,不应 panic
        assert_eq!(hits.load(Ordering::SeqCst), 1);
    }

    /// 回归(审查修复):回调必须在临界区**外**执行。修前 `publish`/`call` 把
    /// 整个派发(含全部回调)裹进 `sync::free`——回调里 `depth_now() >= 1`,
    /// 任意用户代码在关中断(单核)/持全局自旋(SMP)下运行,拖垮中断延迟。
    /// 修后快照在锁内、回调在锁外:回调里临界区深度必须恒为 0,且回调可安全
    /// 调用任何需进临界区的内核原语。
    #[test]
    fn callbacks_run_outside_critical_section() {
        use crate::sync::critical;
        let bus = Bus::<u32>::new();
        static PUB_DEPTH: AtomicUsize = AtomicUsize::new(usize::MAX);
        bus.subscribe("t", move |_, _| {
            PUB_DEPTH.fetch_min(critical::depth_now(), Ordering::SeqCst);
        });
        bus.publish("t", 1);
        assert_eq!(
            PUB_DEPTH.load(Ordering::SeqCst),
            0,
            "publish 回调必须在临界区外执行(修前被 publish 的 free 裹住,深度 >= 1)"
        );

        static CALL_DEPTH: AtomicUsize = AtomicUsize::new(usize::MAX);
        bus.register("x", move |_, _| {
            CALL_DEPTH.fetch_min(critical::depth_now(), Ordering::SeqCst);
        });
        bus.call("x", 0);
        assert_eq!(
            CALL_DEPTH.load(Ordering::SeqCst),
            0,
            "call 回调同样必须在临界区外执行"
        );
    }
}
