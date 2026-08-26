//! 多生产者，多消费者队列
//! 不能在中断服务中使用

use alloc::collections::VecDeque;
// use alloc::sync::Arc;
use super::arc::Arc;
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
    pub fn len(&self) -> usize {
        sync::free(|_| self.list.borrow().len())
    }
    pub fn capacity(&self) -> usize {
        sync::free(|_| self.list.borrow().capacity())
    }
    /// 截断到前 len 个元素。计数与"实际保留数量"对齐（不变量：sem 计数 == 元素数，
    /// 理由同 clear）。不能在中断服务中使用。
    pub fn trancate(&self, len: usize) {
        sync::free(|_| {
            let mut list = self.list.borrow_mut();
            let len = len.min(list.len());
            list.truncate(len);
            self.sem.reset_signal(len);
        })
    }
    /// 清空队列。清空后信号量计数同步归零——Queue 的"sem 计数 == 元素数"不变量
    /// 必须保持，否则残留计数会在空列表上放行 sem.wait()，pop_front 返回幽灵 None
    /// 且计数从此漂移（修前遗留问题 #7）。不能在中断服务中使用。
    pub fn clear(&self) {
        sync::free(|_| {
            self.list.borrow_mut().clear();
            self.sem.reset_signal(0);
        })
    }

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

    /// 中断侧入队。SMP(ch25 ⑥):入队+post+失败回滚整体进全局锁——
    /// 任务侧 push/pop 的借用全在 sync::free 内,ISR 裸借用 VecDeque 会与
    /// 别核并发(借位标志非原子,UB);三段合一还消掉"已入队未 post"的
    /// 跨核可见窗口。post_isr 内层走嵌套临界区,深度配平
    pub fn push_front_isr(&self, item: T) -> nb::Result<(), sync::Error> {
        sync::free(|_| {
            self.list.borrow_mut().push_front(item);
            match self.sem.post_isr() {
                Ok(_) => Ok(()),
                Err(_) => {
                    self.list.borrow_mut().pop_front();
                    Err(nb::Error::Other(sync::Error::QueueFull))
                }
            }
        })
    }
    /// 同 push_front_isr,尾部入队
    pub fn push_back_isr(&self, item: T) -> nb::Result<(), sync::Error> {
        sync::free(|_| {
            self.list.borrow_mut().push_back(item);
            match self.sem.post_isr() {
                Ok(_) => Ok(()),
                Err(_) => {
                    self.list.borrow_mut().pop_back();
                    Err(nb::Error::Other(sync::Error::QueueFull))
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 回归：#7 阳性对照——clear 后信号量计数必须归零。
    /// 修前 clear 只清 VecDeque、sem 停在 2：空列表上 pop_front 的 sem.wait()
    /// 被放行，返回幽灵 None 且计数与列表长度从此脱钩。
    #[test]
    fn clear_resets_semaphore_count() {
        let q = Queue::with_capacity(4);
        q.push_back(1);
        q.push_back(2);
        assert_eq!(q.sem.signal_count(), 2, "入队两个,计数应为 2");
        q.clear();
        assert_eq!(q.sem.signal_count(), 0, "清空后计数必须归零(修前恒为 2)");
        assert_eq!(q.len(), 0);
    }

    /// 回归：#7 伴随用例——trancate 后计数与实际保留数量一致；
    /// 截断长度超过元素数时按"保留 0"处理。
    #[test]
    fn trancate_resets_semaphore_count() {
        let q = Queue::with_capacity(4);
        q.push_back(1);
        q.push_back(2);
        q.push_back(3);
        assert_eq!(q.sem.signal_count(), 3);
        q.trancate(1);
        assert_eq!(q.sem.signal_count(), 1, "保留 1 个,计数应同步为 1");
        q.trancate(99);
        assert_eq!(q.sem.signal_count(), 1, "超长截断=保留全部(1 个),计数不变");
    }
}
