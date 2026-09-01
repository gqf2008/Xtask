use core::mem::ManuallyDrop;
use core::ptr;
use core::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use crossbeam::epoch::{self as epoch, Atomic, Collector, Owned};

#[derive(Debug)]
pub struct TreiberStack<T> {
    head: Atomic<Node<T>>,
    /// 独立纪元收集器:no_std 下 crossbeam 的默认收集器与 `epoch::pin()`
    /// 是 std feature 专属,本仓库 crossbeam 只开 alloc——每栈一个
    /// Collector,操作经 `register().pin()` 钉住纪元(register 有一次性
    /// 堆分配;对无锁栈的一次操作而言可忽略)
    collector: Collector,
}

#[derive(Debug)]
struct Node<T> {
    data: ManuallyDrop<T>, // 告诉编译器该变量不需要自动Drop
    next: Atomic<Node<T>>,
}
impl<T> TreiberStack<T> {
    pub fn new() -> TreiberStack<T> {
        TreiberStack {
            head: Atomic::null(),
            collector: Collector::new(),
        }
    }
    /// 钉住当前线程纪元:pin 存活期内,任何被并发 pop 退役的节点都不会
    /// 真正释放/复用——load 到的地址永远有效,ABA 无从发生
    #[inline]
    fn pin(&self) -> (epoch::LocalHandle, epoch::Guard) {
        let handle = self.collector.register();
        let guard = handle.pin();
        (handle, guard)
    }
    pub fn push(&self, t: T) {
        let mut n = Owned::new(Node {
            data: ManuallyDrop::new(t),
            next: Atomic::null(),
        });

        let (_handle, guard) = self.pin();
        loop {
            let head = self.head.load(Relaxed, &guard);
            n.next.store(head, Relaxed);
            match self
                .head
                .compare_exchange(head, n, Release, Relaxed, &guard)
            {
                // CAS
                Ok(_) => break,
                Err(e) => n = e.new,
            }
        }
    }
    pub fn pop(&self) -> Option<T> {
        let (_handle, guard) = self.pin();
        loop {
            let head = self.head.load(Acquire, &guard);
            match unsafe { head.as_ref() } {
                Some(h) => {
                    let next = h.next.load(Relaxed, &guard);
                    if self
                        .head
                        .compare_exchange(head, next, Relaxed, Relaxed, &guard) // CAS
                        .is_ok()
                    {
                        unsafe {
                            // 先取数据再退役:对未钉住的 guard 调 defer_destroy
                            // 会立即释放节点(crossbeam 文档 "simply be
                            // executed immediately"),修前的「先销毁后读」
                            // 是确定性 read-after-free;pin + 先读后退役
                            // 同时堵住 UAF 与 ABA
                            let data = ManuallyDrop::into_inner(ptr::read(&(*h).data));
                            guard.defer_destroy(head); // 将垃圾加入列表
                            return Some(data);
                            // 返回节点中的数据
                        }
                    }
                }
                None => return None,
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        let (_handle, guard) = self.pin();
        self.head.load(Acquire, &guard).is_null()
    }
}
impl<T> Drop for TreiberStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::TreiberStack;

    /// 回归:修前 pop 在 unprotected guard 上先 defer_destroy(立即释放节点)
    /// 再 ptr::read 数据 = 确定性 read-after-free,单线程 push+pop 即触发
    /// (Miri/ASan 直接报 UAF)。修复后 LIFO 语义与数据完整性必须保持。
    #[test]
    fn pop_returns_pushed_values_in_lifo_order() {
        let s = TreiberStack::new();
        s.push(1u64);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
    }

    /// 含堆数据的节点:pop 出的 String 必须完好(读的是活内存,不是释放后的残影)
    #[test]
    fn pop_preserves_heap_payload() {
        let s = TreiberStack::new();
        s.push(alloc::string::String::from("hello-xtask"));
        assert_eq!(s.pop().as_deref(), Some("hello-xtask"));
    }
}
