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
            if let Some(list) = subscribers.get_mut(topic) {
                let f = Box::new(f);
                let ptr = (f.as_ref() as *const _ as *mut c_void).addr();
                list.push(Box::new(f));
                (topic, ptr)
            } else {
                let f = Box::new(f);
                let ptr = (f.as_ref() as *const _ as *mut c_void).addr();
                let mut list: Vec<Box<dyn Fn(&'a str, E)>> = Vec::new();
                list.push(f);
                subscribers.insert(topic, list);
                (topic, ptr)
            }
        })
    }
    /// 取消订阅
    /// 不能在中断服务中调用
    pub fn unsubscribe(&self, token: Token<'a>) {
        sync::free(|_| {
            let mut subscribers = self.subscribers.borrow_mut();
            if let Some(list) = subscribers.get_mut(token.0) {
                list.retain(|item| {
                    let optr = (item.as_ref() as *const _ as *mut c_void).addr();
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
