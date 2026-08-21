//! 总线
use crate::sync;

use alloc::vec::Vec;
use alloc::{boxed::Box, collections::BTreeMap};
use core::cell::RefCell;
use core::ffi::c_void;

pub struct Bus<'a, E> {
    subscribers: RefCell<BTreeMap<&'a str, Vec<Box<dyn Fn(&'a str, E)>>>>,
    callee: RefCell<BTreeMap<&'a str, Box<dyn Fn(&'a str, E)>>>,
}

unsafe impl<'a, E> Send for Bus<'a, E> {}

unsafe impl<'a, E> Sync for Bus<'a, E> {}

impl<'a, E> Bus<'a, E> {
    pub const fn new() -> Self {
        Self {
            subscribers: RefCell::new(BTreeMap::new()),
            callee: RefCell::new(BTreeMap::new()),
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
            //只箱一次，token 取存入列表的那个 trait 对象指针，
            //保证 unsubscribe 用同一口径能匹配到
            let f: Box<dyn Fn(&'a str, E)> = Box::new(f);
            let ptr = (f.as_ref() as *const _ as *const c_void).addr();
            let list = subscribers.entry(topic).or_default();
            list.push(f);
            (topic, ptr)
        })
    }
    /// 取消订阅
    /// 不能在中断服务中调用
    pub fn unsubscribe(&self, token: Token<'a>) {
        sync::free(|_| {
            let mut subscribers = self.subscribers.borrow_mut();
            if let Some(list) = subscribers.get_mut(token.0) {
                list.retain(|item| {
                    let optr = (item.as_ref() as *const _ as *const c_void).addr();
                    optr != token.1
                });
            }
        });
    }

    /// 发送事件
    /// 不能在中断服务中调用，中断服务中调用请用event_isr
    pub fn publish(&self, topic: &'a str, event: E) -> &Self {
        sync::free(|_| self.publish_isr(topic, event))
    }

    /// 发送事件
    /// 只能在中断服务中调用
    pub fn publish_isr(&self, topic: &'a str, event: E) -> &Self {
        let mut subscribers = self.subscribers.borrow_mut();
        if let Some(list) = subscribers.get_mut(topic) {
            list.iter().for_each(|f| f(topic, event.clone()));
        }
        self
    }

    pub fn register<F: Fn(&'a str, E) + Send + 'static>(&self, name: &'a str, f: F) -> &Self {
        sync::free(|_| self.register_isr(name, f))
    }

    pub fn unregister(&self, name: &'a str) {
        sync::free(|_| self.unregister_isr(name))
    }

    pub fn register_isr<F: Fn(&'a str, E) + Send + 'static>(&self, name: &'a str, f: F) -> &Self {
        let f = Box::new(f);
        let mut callee = self.callee.borrow_mut();
        callee.insert(name, f);
        self
    }

    pub fn unregister_isr(&self, name: &'a str) {
        let mut callee = self.callee.borrow_mut();
        callee.remove(name);
    }

    pub fn call(&self, name: &'a str, event: E) -> &Self {
        sync::free(|_| self.call_isr(name, event))
    }

    pub fn call_isr(&self, name: &'a str, event: E) -> &Self {
        let callee = self.callee.borrow();
        if let Some(f) = callee.get(name) {
            f(name, event);
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
}
