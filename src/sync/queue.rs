//! 多生产者，多消费者队列
//! 不能在中断服务中使用

use alloc::collections::VecDeque;
use alloc::sync::Arc;

use super::semaphore::*;
use crate::sync;
use core::cell::RefCell;

#[derive(Clone)]
pub struct Queue<T> {
    list: Arc<RefCell<VecDeque<T>>>,
    sem: Semaphore,
}

unsafe impl<T> Send for Queue<T> {}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            list: Arc::new(RefCell::new(VecDeque::new())),
            sem: Semaphore::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            list: Arc::new(RefCell::new(VecDeque::new())),
            sem: Semaphore::with_max_value(capacity),
        }
    }
}

impl<T> Queue<T> {
    pub fn pop_front(&self) -> Option<T> {
        self.sem.wait();
        sync::free(|_| self.list.borrow_mut().pop_front())
    }

    pub fn pop_back(&self) -> Option<T> {
        self.sem.wait();
        sync::free(|_| self.list.borrow_mut().pop_back())
    }

    pub fn push_front(&self, item: T) {
        sync::free(|_| {
            self.list.borrow_mut().push_front(item);
        });
        self.sem.post();
    }
    pub fn push_back(&self, item: T) {
        sync::free(|_| {
            self.list.borrow_mut().push_back(item);
        });
        self.sem.post();
    }

    pub fn push_front_isr(&self, item: T) -> nb::Result<(), sync::Error> {
        self.list.borrow_mut().push_front(item);
        match self.sem.post_isr() {
            Ok(_) => Ok(()),
            Err(_) => Err(nb::Error::Other(sync::Error::QueueFull)),
        }
    }
    pub fn push_back_isr(&self, item: T) -> nb::Result<(), sync::Error> {
        self.list.borrow_mut().push_back(item);
        match self.sem.post_isr() {
            Ok(_) => Ok(()),
            Err(_) => Err(nb::Error::Other(sync::Error::QueueFull)),
        }
    }
}
