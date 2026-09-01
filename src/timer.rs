//! 软定时器，应用场景为非关键业务辅助定时
//! 精度取决于系统节拍

use crate::chip::TIMER_STACK_SIZE_WORD;
use crate::executor::xworker;
use crate::executor::Executor;
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
use alloc::vec::Vec;
use core::cmp::{Ord, Ordering, Reverse};
use core::ffi::c_void;

static mut HEAP: Option<BinaryHeap<Box<TimerInner>>> = None;
static mut READY: Option<VecDeque<Box<TimerInner>>> = None;

/// 取消登记:句柄 drop 时目标定时器既不在 HEAP 也不在 READY 的第三种
/// 位置——正在被 timer_task 弹到局部变量里执行回调(in-flight)。
/// drop 的 retain 对它必落空,登记到这里;drain_ready_one 回调返回后
/// 消费:不复活并入 TimerInner::drop 回收闭包。条目是瞬态的(消费它的
/// 那次 drain 就在 drop 之后紧随的同一轮 READY 消费里)
static mut CANCELLED: Vec<usize> = Vec::new();

static mut TIMER_TASK: *mut Task = core::ptr::null_mut();

pub(crate) fn start_timer_task() {
    log::info!("start_timer_task");
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
                    while drain_ready_one(q) {}
                }
                let task = xworker.current();
                task.block();
            });
            //block() 只改状态不切换任务，必须显式让出 CPU，
            //否则高优先级的定时任务会空转饿死其他任务
            yield_now();
        }
    }
}

/// 消费 READY 队首一个到期定时器,返回是否消费了一个:
/// 执行回调后,周期定时器按"取消登记"决定复活还是回收——回调期间
/// 句柄被 drop 的定时器此刻躺在本函数局部变量里(in-flight),不在
/// HEAP/READY 任何容器,Timer::drop 的 retain 必落空、只能登记到
/// CANCELLED;这里消费登记:不重新入堆,并让 TimerInner::drop 回收
/// 闭包(修前无条件复活,句柄销毁后永远关不掉、闭包永久泄漏)
unsafe fn drain_ready_one(q: &mut VecDeque<Box<TimerInner>>) -> bool {
    let Some(mut t) = q.pop_front() else {
        return false;
    };
    (t.entry)(t.args);
    let addr = t.args.addr();
    if let Some(pos) = CANCELLED.iter().position(|&a| a == addr) {
        CANCELLED.swap_remove(pos);
        drop(t); // 句柄已在回调期间 drop:不复活,回收闭包
    } else if t.period > 0 {
        t.next_tick = time::tick() + t.period as u64;
        submit(t);
    }
    true
}

/// 扫描堆顶是否有超时定时任务
/// 有则唤醒工作任务，触发软中断
#[inline]
pub(crate) fn do_tick(ticks: u64) {
    // SMP(ch25 ⑥):ISR 侧的堆操作也必须进全局锁——任务侧(after/period/
    // Drop/timer_task 的 READY 消费)全部在 sync::free 内,ISR 裸操作堆会
    // 与别核任务侧并发撕 BinaryHeap 内部数组。trap 上下文持锁安全:持区者
    // 所在核的中断已关(ISR 不会在同核与其并发),别核持区者短临界区有界
    // 自旋即得——与 do_systick/do_schedule 整段进锁同款纪律(假设三)。
    let ready = sync::free(|_| unsafe {
        let mut ready = false;
        if let Some(heap) = &mut HEAP {
            //同一个 tick 可能到期多个定时器，必须全部挪进 READY，
            //只弹堆顶一个会让其余到期定时器各多延迟一个 tick
            while let Some(timer) = heap.peek() {
                if ticks < timer.next_tick {
                    break;
                }
                ready = true;
                if let Some(timer) = heap.pop() {
                    if let Some(q) = &mut READY {
                        q.push_back(timer);
                    } else {
                        let mut q = VecDeque::new();
                        q.push_back(timer);
                        READY = Some(q)
                    }
                }
            }
            if ready {
                if let Some(task) = TIMER_TASK.as_mut() {
                    task.wakeup();
                }
            }
        }
        ready
    });
    if ready {
        //让出须在临界区外:pending 的软中断出区(mret)后生效,语义不变
        yield_now();
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

/// 软定时器堆顶的下次触发拍(堆空/未启动 = None)。
/// 供 tickless 空闲引擎汇集期限——deadline 必须来自堆顶,新建的
/// `Timer::after/period` 已在任务侧入堆,无需唤醒 timer 任务即可被
/// 武装到点。调用方(空闲引擎)在 sync::free 内读取,与任务侧提交同锁
#[inline]
pub(crate) fn next_timer_tick() -> Option<u64> {
    // SAFETY: 调用方持全局锁;读取堆顶只借不可变引用
    unsafe { HEAP.as_ref().and_then(|h| h.peek().map(|t| t.next_tick)) }
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
                let mut found = false;
                if let Some(heap) = &mut HEAP {
                    heap.retain(|item| {
                        let keep = item.args.addr() != self.0;
                        found |= !keep;
                        keep
                    });
                }
                //到期定时器可能已被 do_tick 挪进 READY 队列，这里也必须清理，
                //否则取消失效：周期定时器会继续触发并重新入堆，句柄 drop 后也永远关不掉
                if let Some(q) = &mut READY {
                    q.retain(|item| {
                        let keep = item.args.addr() != self.0;
                        found |= !keep;
                        keep
                    });
                }
                // 第三种位置:正在被 timer_task 执行回调(in-flight,不在
                // 任何容器,retain 必落空)——登记取消,由 drain_ready_one
                // 在回调返回后消费;否则周期定时器被无条件复活,句柄销毁后
                // 再无取消手段
                if !found {
                    CANCELLED.push(self.0);
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

#[cfg(test)]
mod tests {
    use super::{do_tick, drain_ready_one, Timer, TimerInner, CANCELLED, HEAP, READY};
    use alloc::boxed::Box;
    use alloc::collections::{BinaryHeap, VecDeque};
    use alloc::sync::Arc;
    use core::ffi::c_void;
    use core::sync::atomic::{AtomicUsize, Ordering};

    fn dummy(_args: *mut c_void) {}

    /// 一次性定时器（period=0，Drop 不会回收 args，null 指针安全）
    fn one_shot(next_tick: u64) -> Box<TimerInner> {
        Box::new(TimerInner {
            entry: dummy,
            args: core::ptr::null_mut(),
            period: 0,
            next_tick,
        })
    }

    /// 回归：同一 tick 到期的多个定时器必须全部挪进 READY（修复前每个 tick 只弹堆顶一个，
    /// 其余到期定时器各多延迟一个 tick）；且句柄 drop 必须能取消已在 READY 队列里的
    /// 定时器（修复前只清理堆，READY 里的周期定时器会继续触发并重新入堆，永远关不掉）。
    /// 注：这是唯一触碰 HEAP/READY 全局的测试，与其他测试无共享状态，可并行运行。
    #[test]
    fn do_tick_drains_all_expired_and_timer_drop_cancels_ready() {
        unsafe {
            HEAP = Some(BinaryHeap::new());
            READY = Some(VecDeque::new());

            let heap = HEAP.as_mut().unwrap();
            heap.push(one_shot(50));
            heap.push(one_shot(60));
            heap.push(one_shot(200));

            do_tick(100);
            assert_eq!(
                READY.as_ref().unwrap().len(),
                2,
                "同一 tick 到期的定时器都应进入 READY"
            );
            assert_eq!(HEAP.as_ref().unwrap().len(), 1, "未到期的必须留在堆里");

            // READY 队列里的周期定时器也能被句柄 drop 取消
            let f: Box<Box<dyn Fn() + Send + 'static>> = Box::new(Box::new(|| {}));
            let args = &*f as *const _ as *mut c_void;
            core::mem::forget(f);
            let timer = Box::new(TimerInner {
                entry: dummy,
                args,
                period: 5,
                next_tick: 10,
            });
            let handle = Timer(timer.args.addr());
            READY.as_mut().unwrap().push_back(timer);
            drop(handle);
            assert_eq!(
                READY.as_ref().unwrap().len(),
                2,
                "READY 里的定时器应被句柄 drop 取消"
            );

            // 第三种位置:in-flight(正被 drain 弹到局部变量里执行回调)
            // 的周期定时器,回调里 drop 句柄也必须取消生效——修前 retain
            // 落空后回调返回仍被无条件重新入堆:永久复活、句柄销毁后再无
            // 取消手段、闭包泄漏。修复后经 CANCELLED 登记:不复活且
            // TimerInner::drop 回收闭包
            fn call_fn(args: *mut c_void) {
                // 与 TimerInner::period 的 entry 同款:调用闭包但不消耗它
                unsafe {
                    let b = Box::from_raw(args as *mut Box<dyn Fn()>);
                    b();
                    core::mem::forget(b);
                }
            }
            static HANDLE: std::sync::Mutex<Option<Timer>> = std::sync::Mutex::new(None);

            // 先清空上半场遗留的到期一次性定时器,让 in-flight 场景从队首开始
            while drain_ready_one(READY.as_mut().unwrap()) {}
            assert!(READY.as_ref().unwrap().is_empty());

            // 周期定时器,回调里 drop 自己的句柄;闭包持 Arc 计数验证回收
            let mark = Arc::new(AtomicUsize::new(0));
            let f: Box<Box<dyn Fn() + Send + 'static>> = Box::new(Box::new({
                let mark = mark.clone();
                move || {
                    mark.fetch_add(1, Ordering::SeqCst);
                    if let Some(h) = HANDLE.lock().unwrap().take() {
                        drop(h); // 回调中取消自己
                    }
                }
            }));
            let args = &*f as *const _ as *mut c_void;
            core::mem::forget(f);
            let timer = Box::new(TimerInner {
                entry: call_fn,
                args,
                period: 5,
                next_tick: 0,
            });
            *HANDLE.lock().unwrap() = Some(Timer(timer.args.addr()));
            READY.as_mut().unwrap().push_back(timer);

            // 消费:回调执行(记 1 次)并自 drop → 登记 → drain 消费,不复活
            assert!(drain_ready_one(READY.as_mut().unwrap()));
            assert_eq!(mark.load(Ordering::SeqCst), 1, "回调应执行一次");
            assert_eq!(
                HEAP.as_ref().unwrap().len(),
                1,
                "in-flight drop 后周期定时器不得复活入堆(堆里只剩上半场未到期的一次性定时器)"
            );
            assert!(CANCELLED.is_empty(), "取消登记必须被当轮消费");
            assert_eq!(
                Arc::strong_count(&mark),
                1,
                "取消后 TimerInner::drop 必须回收闭包(Arc 计数回落)"
            );

            // 阳性对照:句柄未 drop 的周期定时器照常复活入堆
            let g: Box<Box<dyn Fn() + Send + 'static>> = Box::new(Box::new(|| {}));
            let gargs = &*g as *const _ as *mut c_void;
            core::mem::forget(g);
            READY.as_mut().unwrap().push_back(Box::new(TimerInner {
                entry: call_fn,
                args: gargs,
                period: 5,
                next_tick: 0,
            }));
            assert!(drain_ready_one(READY.as_mut().unwrap()));
            assert_eq!(
                HEAP.as_ref().unwrap().len(),
                2,
                "未取消的周期定时器必须复活(上半场遗留 1 + 复活 1)"
            );
            assert!(CANCELLED.is_empty(), "阳性对照不得误挂取消登记");

            //清理现场，避免污染其他测试
            HEAP = None;
            READY = None;
        }
    }
}
