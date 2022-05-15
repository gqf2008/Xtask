//! 多生产者，多消费者队列

use alloc::{collections::VecDeque, sync::Arc};

use super::semaphore::*;
use crate::sync;
use bare_metal::Mutex;
use core::cell::RefCell;

#[derive(Clone)]
pub struct Queue<T> {
    list: Arc<Mutex<RefCell<VecDeque<T>>>>,
    sem: Semaphore,
}
unsafe impl<T> Send for Queue<T> {}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            list: Arc::new(Mutex::new(RefCell::new(VecDeque::new()))),
            sem: Semaphore::new(),
        }
    }
}

impl<T> Queue<T> {
    pub fn pop_front(&self) -> Option<T> {
        self.sem.wait();
        sync::free(|cs| self.list.borrow(*cs).borrow_mut().pop_front())
    }

    pub fn pop_back(&self) -> Option<T> {
        self.sem.wait();
        sync::free(|cs| self.list.borrow(*cs).borrow_mut().pop_back())
    }

    pub fn push_front(&self, item: T) {
        sync::free(|cs| {
            self.list.borrow(*cs).borrow_mut().push_front(item);
        });
        self.sem.post();
    }
    pub fn push_back(&self, item: T) {
        sync::free(|cs| {
            self.list.borrow(*cs).borrow_mut().push_back(item);
        });
        self.sem.post();
    }
}
