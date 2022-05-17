//! 软定时器，应用场景为非关键业务辅助定时
//! 精度取决于系统节拍

use crate::chip::TIMER_STACK_SIZE_WORD;
use crate::executor::xworker;
use crate::executor::Executor;
use crate::sprintln;
use crate::sync;
use crate::task::Func;
use crate::task::TIMER_TASK_NAME;
use crate::time;
use crate::yield_now;
use crate::State;
use crate::Task;
use alloc::boxed::Box;
use alloc::collections::BinaryHeap;
use alloc::collections::VecDeque;
use core::cmp::{Ord, Ordering, Reverse};
use core::ffi::c_void;

static mut HEAP: Option<BinaryHeap<Box<TimerInner>>> = None;
static mut READY: Option<VecDeque<Box<TimerInner>>> = None;

static mut TIMER_TASK: *mut Task = core::ptr::null_mut();

pub(crate) fn start_timer_task() {
    sprintln!("start_timer_task");
    unsafe {
        if HEAP.is_none() {
            HEAP = Some(BinaryHeap::new());
            READY = Some(VecDeque::new());
        }

        let task = Task::new(
            TIMER_TASK_NAME,
            TIMER_STACK_SIZE_WORD,
            1,
            timer_task,
            core::ptr::null_mut(),
        );
        if let Some(task) = task.as_mut() {
            task.state = State::Suspended;
        }
        core::ptr::replace(&mut TIMER_TASK, task);
    }

    fn timer_task(_args: *mut c_void) {
        loop {
            sync::free(|_cs| unsafe {
                if let Some(q) = &mut READY {
                    loop {
                        if let Some(mut t) = q.pop_front() {
                            (t.entry)(t.args);
                            if t.period > 0 {
                                t.next_tick = time::tick() + t.period as u64;
                                submit(t);
                            }
                        } else {
                            break;
                        }
                    }
                }
                let task = xworker.current();
                task.block();
            });
        }
    }
}

/// 扫描堆顶是否有超时定时任务
/// 有则唤醒工作任务，触发软中断
#[inline]
pub(crate) fn do_tick(ticks: u64) {
    unsafe {
        if let Some(heap) = &mut HEAP {
            let mut ready = false;
            if let Some(timer) = heap.peek() {
                if ticks >= timer.next_tick {
                    ready = true;
                }
            }
            if ready {
                if let Some(timer) = heap.pop() {
                    if let Some(q) = &mut READY {
                        q.push_back(timer);
                    } else {
                        let mut q = VecDeque::new();
                        q.push_back(timer);
                        READY = Some(q)
                    }
                }
                if let Some(task) = TIMER_TASK.as_mut() {
                    task.wakeup();
                }
                yield_now();
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Eq, Clone)]
struct TimerInner {
    entry: Func,       //入口函数
    args: *mut c_void, //参数
    period: usize,     //周期
    next_tick: u64,    //下次触发时间
}

impl Drop for TimerInner {
    fn drop(&mut self) {
        if self.period > 0 {
            unsafe {
                let _ = Box::from_raw(self.args as *mut Box<dyn Fn()>);
            }
        }
    }
}

impl TimerInner {
    fn after<F: FnOnce() + Send + 'static>(ms: usize, f: F) {
        fn entry(args: *mut c_void) {
            unsafe {
                let b = Box::from_raw(args as *mut Box<dyn FnOnce()>);
                b();
            }
        }
        let f: Box<Box<dyn FnOnce() + Send + 'static>> = Box::new(Box::new(f));
        let args = &*f as *const _ as *mut c_void;
        let after = time::ms2ticks(ms);

        let timer = Box::new(Self {
            entry: entry,
            args: args,
            period: 0,
            next_tick: time::tick() + after as u64,
        });

        core::mem::forget(f);
        sync::free(|_| unsafe { submit(timer) });
    }

    fn period<F: Fn() + Send + 'static>(period_ms: usize, f: F) -> usize {
        fn entry(args: *mut c_void) {
            unsafe {
                let b = Box::from_raw(args as *mut Box<dyn Fn()>);
                b();
                core::mem::forget(b);
            }
        }
        let f: Box<Box<dyn Fn() + Send + 'static>> = Box::new(Box::new(f));
        let args = &*f as *const _ as *mut c_void;
        let period = time::ms2ticks(period_ms);
        let timer = Box::new(Self {
            entry,
            args,
            period,
            next_tick: time::tick() + period as u64,
        });
        core::mem::forget(f);
        let addr = timer.args.addr();
        sync::free(|_| unsafe { submit(timer) });
        addr
    }
}

unsafe fn submit(timer: Box<TimerInner>) {
    if let Some(heap) = &mut HEAP {
        heap.push(timer);
    } else {
        let mut heap = BinaryHeap::new();
        heap.push(timer);
        HEAP = Some(heap);
    }
}

pub struct Timer(usize);

impl Timer {
    pub fn after<F: FnOnce() + Send + 'static>(ms: usize, f: F) {
        TimerInner::after(ms, f)
    }

    pub fn period<F: Fn() + Send + 'static>(period_ms: usize, f: F) -> Timer {
        Timer(TimerInner::period(period_ms, f))
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        unsafe {
            sync::free(|_| {
                if let Some(heap) = &mut HEAP {
                    heap.retain(|item| item.args.addr() != self.0);
                }
            });
        }
    }
}

impl PartialEq for TimerInner {
    fn eq(&self, other: &Self) -> bool {
        self.next_tick.eq(&other.next_tick)
    }
}

impl PartialOrd for TimerInner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Reverse(self.next_tick).partial_cmp(&Reverse(other.next_tick))
    }
}

impl Ord for TimerInner {
    fn cmp(&self, other: &Self) -> Ordering {
        self.next_tick.cmp(&other.next_tick)
    }
}
