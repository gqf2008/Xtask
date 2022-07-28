use core::mem::ManuallyDrop;
use core::ptr;
use core::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use crossbeam::epoch::{self as epoch, Atomic, Owned};

#[derive(Debug)]
pub struct TreiberStack<T> {
    head: Atomic<Node<T>>,
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
        }
    }
    pub fn push(&self, t: T) {
        let mut n = Owned::new(Node {
            data: ManuallyDrop::new(t),
            next: Atomic::null(),
        });

        let guard = unsafe { epoch::unprotected() }; // 标记当前线程为活跃
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
        let guard = unsafe { epoch::unprotected() }; // 标记当前线程为活跃
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
                            guard.defer_destroy(head); // 将垃圾加入列表
                            return Some(ManuallyDrop::into_inner(ptr::read(&(*h).data)));
                            // 返回节点中的数据
                        }
                    }
                }
                None => return None,
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        let guard = unsafe { epoch::unprotected() };
        self.head.load(Acquire, &guard).is_null()
    }
}
impl<T> Drop for TreiberStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
