use crate::port::{Portable, Porting};
use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::task::State;
use crate::task::{scheduler::Scheduler, Task, TaskQueue};
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use bit_field::BitField;

use super::idle::start_idle_task;

pub(super) type XTaskScheduler = ();

impl Scheduler for XTaskScheduler {
    fn name(&self) -> &'static str {
        "XTaskScheduler"
    }

    /// 启动调度器
    fn start(&self) -> ! {
        start_idle_task();
        Porting::start_scheduler()
    }
    /// 提交一个任务进队列，待调度
    fn submit(&self, task: *mut Task) {
        sync::free(|_| unsafe { submit_task(task) });
    }

    fn do_systick(&self) -> bool {
        unsafe {
            //摘到期任务（队首起 wake_tick <= now 的连续段）重新提交调度。
            //队列按 wake_tick 升序，tick 开销 = O(到期数)，不再全队列扫描
            let now = crate::time::tick();
            take_expired(&mut DELAY, now, |task| submit_task(task));

            // 检查尾零数，是否有比当前任务相等或更高优先级的任务
            // 如果想等优先级则时间片调度，否则就一直抢占着，直到任务主动挂起
            // TODO 需改进 ARM CLZ指令计算前导零
            let trailing_zero = READY_BITS.trailing_zeros();
            trailing_zero < 16 && (trailing_zero + 1) <= self.current().priority as u32
        }
    }
    // 找到一个就绪任务把当前任务切出去
    fn do_schedule(&self) {
        unsafe {
            //弹出一个就绪任务
            let new = pop_ready();
            if new != xworker.current() {
                if let Some(new) = new.as_mut() {
                    if let Some(old) = xworker.execute(new).and_then(|item| item.as_mut()) {
                        //检查是否栈溢出
                        old.stack_overflow();
                        submit_task(old);
                    }
                }
            }
        }
    }
}

/// 从延时队列队首摘下所有到期任务（`wake_tick <= now`），逐个回调。
/// 队列按 wake_tick 升序维护（push_delay 有序插入），所以到期者必是队首的
/// 一段连续前缀，摘到首个未到期即停——tick 中断的开销从"每 tick O(n) 全队列
/// 递减 + 两次堆分配"降为 O(到期数)、零分配（F3）。
/// 同刻到期的任务按入队先后（FIFO）依次摘下，唤醒次序与相对递减时代一致。
#[inline(always)]
pub(crate) fn take_expired(
    delay: &mut TaskQueue,
    now: u64,
    mut on_expired: impl FnMut(*mut Task),
) {
    loop {
        let expired = match delay.front() {
            // SAFETY: 延时队列中的任务指针入队时均为有效 Task；此函数只在临界区
            // （ISR 关中断）或 host 单线程测试下被调用，无二度可变别名
            Some(&head) => unsafe { (*head).wake_tick <= now },
            None => break,
        };
        if !expired {
            break;
        }
        // 刚判过非空，pop_front 必有值
        if let Some(task) = delay.pop_front() {
            unsafe {
                // SAFETY: 刚从队首取出的有效任务指针。任务已出队，清掉 queue 字段，
                // 避免随后 bind 入就绪队列时对延时队列做一次无用的 O(n) retain 扫描
                (*task).queue = None;
                (*task).state = State::Ready;
            }
            on_expired(task);
        }
    }
}

/// 任务入队列
#[track_caller]
#[inline(always)]
pub(crate) unsafe fn submit_task(task: *mut Task) {
    if let Some(task) = task.as_mut() {
        match task.state {
            State::Ready => {
                push_ready(task);
                // 抢占式语义(修前缺失,QEMU 执行测试抓出):入队任务优先级
                // 更高(数字更小)时请求调度——否则 spawn 高优先级任务、
                // post/notify/unlock 唤醒高优先级 waiter 都要干等下个 tick。
                // MSIP 只是"请求":本函数常在 sync::free 临界区内被调,
                // 软中断 pending 到退出临界区(mret)后生效——语义正确
                request_preempt_if_higher(task);
            }
            State::Blocked => {
                push_delay(task);
            }
            State::Suspended => {
                // task.bind_none();
                //push_blocked(task);
            }
            State::Terminated => {
                //就地删除
                let _ = Box::from_raw(task as *mut Task);
            }
            State::Running => {
                task.ready();
                push_ready(task);
            }
        }
    }
}

/// 查找并弹出就绪任务
/// 如果任务队列里没有就绪任务，则返回IDLE任务
/// 不变式：`READY_BITS` 位 i 置位 ⟺ `READYQ[i]` 非空（push/pop 双侧维护，
/// 不一致时 pop 侧自清位并在 debug 下断言——F4：修前一致性纯靠约定）
#[inline(always)]
unsafe fn pop_ready() -> *mut Task {
    let tz = READY_BITS.trailing_zeros() as usize;
    if tz >= 16 {
        return IDLE_TASK;
    }
    let q = &mut READYQ[tz];
    match q.pop_front() {
        Some(task) => {
            if q.is_empty() {
                READY_BITS.set_bit(tz, false);
            }
            task
        }
        None => {
            debug_assert!(false, "READY_BITS 位{tz}置位但队列为空——不变式被破坏");
            READY_BITS.set_bit(tz, false);
            IDLE_TASK
        }
    }
}

/// 推入就绪队列
#[track_caller]
/// 入队任务比当前任务优先级更高(数字更小)则请求调度。
/// 调度器未启动(spawn 阶段 CURRENT_TASK=null)时不触发——任务已入队,
/// start() 自然会调度到它
unsafe fn request_preempt_if_higher(task: *mut Task) {
    let cur = super::xworker::current_ptr();
    if cur.is_null() {
        return;
    }
    if (*task).priority < (*cur).priority {
        Porting::irq();
    }
}

unsafe fn push_ready(task: *mut Task) {
    if let Some(task) = task.as_mut() {
        let idx = (task.priority - 1) as usize;
        debug_assert!(idx < 16, "非法优先级 {}", task.priority);
        if idx < 16 {
            task.bind(&mut READYQ[idx]);
            READY_BITS.set_bit(idx, true);
        }
    } else {
        panic!("put_task, illegal task {:p}", task);
    }
}

/// 推入延时队列——按 wake_tick 升序有序插入（同刻到期的排已有之后，
/// 保持 FIFO 唤醒次序）；tick 中断侧因此只需从队首摘到期段（F3）
#[inline(always)]
unsafe fn push_delay(task: *mut Task) {
    if let Some(task) = task.as_mut() {
        // 与 bind 同款先去重：任务已在某队列中时直接插入会重复入队
        let ptr = task as *mut Task;
        if let Some(from) = &mut task.queue {
            (*from).retain(|item| *item != ptr);
        }
        let pos = DELAY
            .iter()
            .position(|&t| (*t).wake_tick > task.wake_tick)
            .unwrap_or(DELAY.len());
        DELAY.insert(pos, task);
        task.queue = Some(&mut DELAY);
    }
}
static mut READY_BITS: u16 = 0;

/// 空闲任务，没有就绪任务时就切到这个任务
pub(crate) static mut IDLE_TASK: *mut Task = core::ptr::null_mut();

/// 1-16 优先级任务就绪队列（下标 = 优先级-1），数字越小优先级越高。
/// `VecDeque::new` 是 const——编译期初始化，运行期零惰性初始化
/// （F4+F5：修前是 16 个 `Option` 静态量 + `INITED` 惰性 init，
/// 32 处手写 match 臂、多核启动下有良性竞争，一并结构性消除）
pub(crate) static mut READYQ: [TaskQueue; 16] = [const { VecDeque::new() }; 16];

/// 延时队列——按 wake_tick 升序（push_delay 有序插入维护）
pub(crate) static mut DELAY: TaskQueue = VecDeque::new();

#[cfg(test)]
mod tests {
    use super::take_expired;
    use crate::task::{State, Task, TaskQueue};
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use core::ffi::c_void;

    fn dummy_entry(_args: *mut c_void) {}

    /// 构造一个阻塞态、wake_tick 为指定绝对时刻的任务（模拟 sleep 到点）。
    fn blocked_task(wake_tick: u64) -> *mut Task {
        let t = Task::new("t", 128, 8, dummy_entry, core::ptr::null_mut());
        // SAFETY: 刚创建的独占任务，直接设置字段模拟"延时中"状态
        unsafe {
            (*t).wake_tick = wake_tick;
            (*t).state = State::Blocked;
        }
        t
    }

    /// 回收任务，避免测试泄漏（Task 的 Drop 会一并释放任务栈）
    unsafe fn reclaim(ptrs: &[*mut Task]) {
        for &p in ptrs {
            drop(Box::from_raw(p));
        }
    }

    /// 驱动 take_expired 并把到期任务收集成 Vec（host 测试里分配无妨——
    /// 生产路径走回调零分配）。
    fn collect_expired(q: &mut TaskQueue, now: u64) -> Vec<*mut Task> {
        let mut out = Vec::new();
        take_expired(q, now, |t| out.push(t));
        out
    }

    /// 回归：多个任务同一时刻到期时必须全部取出（原 bug #1：相对递减时代
    /// 升序 remove(i) 下标错位漏唤醒；有序队列时代为队首连摘）。
    #[test]
    fn take_expired_removes_all_without_index_slippage() {
        let mut q = TaskQueue::new();
        let ptrs: Vec<*mut Task> = (0..3).map(|_| blocked_task(10)).collect();
        for &p in &ptrs {
            q.push_back(p);
        }

        let expired = collect_expired(&mut q, 10);

        assert_eq!(expired.len(), 3, "3 个到期任务都应被取出");
        assert!(q.is_empty(), "延时队列应被清空");
        // 同刻到期按 FIFO 次序摘下（与相对递减时代的唤醒次序一致）
        assert!(expired.iter().zip(ptrs.iter()).all(|(a, b)| core::ptr::eq(*a, *b)));
        unsafe { reclaim(&ptrs) };
    }

    /// 回归：未到期的任务必须留在队列里，且队列保持升序。
    #[test]
    fn take_expired_keeps_unexpired() {
        let mut q = TaskQueue::new();
        let expired_one = blocked_task(10); // now=10 到期
        let pending = blocked_task(14); // 未到期
        q.push_back(expired_one);
        q.push_back(pending);

        let expired = collect_expired(&mut q, 10);

        assert_eq!(expired.len(), 1);
        assert_eq!(q.len(), 1, "未到期任务必须保留");
        assert!(core::ptr::eq(q[0], pending));
        unsafe { reclaim(&[expired_one, pending]) };
    }

    /// 阳性对照：队首未到期时，后面的任务即使"看起来近"也绝不被摘
    /// （有序队列只查队首——若误摘中段，有序性不变式即被破坏）。
    #[test]
    fn take_expired_stops_at_first_unexpired() {
        let mut q = TaskQueue::new();
        let head = blocked_task(20); // 未到期
        let tail = blocked_task(30);
        q.push_back(head);
        q.push_back(tail);

        let expired = collect_expired(&mut q, 10);

        assert!(expired.is_empty(), "队首未到期则一个都不摘");
        assert_eq!(q.len(), 2);
        unsafe { reclaim(&[head, tail]) };
    }

    /// 回归：到期任务出队后状态回到 Ready、queue 字段清空
    /// （不清 queue 会让随后 bind 入就绪队列时对延时队列白扫一次 retain）。
    #[test]
    fn take_expired_marks_ready_and_detaches() {
        let mut q = TaskQueue::new();
        let t = blocked_task(5);
        q.push_back(t);

        let expired = collect_expired(&mut q, 5);

        assert_eq!(expired.len(), 1);
        unsafe {
            assert_eq!((*t).state, State::Ready);
            assert!((*t).queue.is_none(), "出队任务必须摘掉 queue 回指");
            reclaim(&[t]);
        }
    }

    /// 回归：空队列安全。
    #[test]
    fn take_expired_empty_queue() {
        let mut q = TaskQueue::new();
        assert!(collect_expired(&mut q, 100).is_empty());
    }
}
