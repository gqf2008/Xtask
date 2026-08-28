use crate::port::{Portable, Porting, MAX_HARTS};
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
        // 多核口此刻放行停泊的从核(就绪队列/每核 idle 已就绪);
        // 单核口是空操作
        Porting::start_secondary_cores();
        Porting::start_scheduler()
    }
    /// 提交一个任务进队列，待调度
    fn submit(&self, task: *mut Task) {
        sync::free(|_| unsafe { submit_task(task) });
    }

    fn do_systick(&self) -> bool {
        // 整段进全局临界区:SMP 下 tick ISR(主核)与别核任务侧并发,
        // DELAY/READYQ 的访问必须与任务侧同一把锁(ch25 失效清单三:
        // "ISR 裸访问"在 SMP 下集体失效——ISR 侧显式进锁)
        sync::free(|_| unsafe {
            //摘到期任务（队首起 wake_tick <= now 的连续段）重新提交调度。
            //队列按 wake_tick 升序，tick 开销 = O(到期数)，不再全队列扫描
            let now = crate::time::tick();
            let mut n_wake = 0usize;
            take_expired(&mut DELAY, now, |task| {
                n_wake += 1;
                submit_task(task)
            });
            if n_wake > 0 {
                crate::sprint!("W{}@{} ", n_wake, now); // DEBUG: 到期唤醒探针
            }

            // 检查尾零数，是否有比当前任务相等或更高优先级的任务
            // 如果想等优先级则时间片调度，否则就一直抢占着，直到任务主动挂起
            // TODO 需改进 ARM CLZ指令计算前导零
            let trailing_zero = READY_BITS.trailing_zeros();
            trailing_zero < 16 && (trailing_zero + 1) <= self.current().priority as u32
        })
    }
    // 找到一个就绪任务把当前任务切出去
    fn do_schedule(&self) {
        // 与 do_systick 同理进全局临界区;嵌套深度由 critical.rs 配平
        sync::free(|_| unsafe {
            //弹出一个就绪任务(本核无就绪则回本核 idle)
            let new = pop_ready();
            let cur = super::xworker::current_ptr();
            // 离开本核 idle 的边界:tickless 曾把节拍定时器拨成"一次性
            // 武装/停表",现在确有别的任务要跑——口侧按实测补账
            // (TICKS += el)并把节拍拨回恒定。否则"睡眠中被外部中断
            // 早醒 → 任务运行 → idle 重新武装"会把新武装锚在冻结的
            // TICKS 上,墙钟期限被每个清醒片段整体拖后;任务运行期也
            // 失去逐拍时间片/到期摘取。恒定节拍口/未睡眠的 idle 是
            // 空操作
            let me = (Porting::hart_id() as usize).min(MAX_HARTS - 1);
            if !cur.is_null() && cur == IDLE_TASKS[me] && new != cur {
                Porting::tickless_leave_idle();
            }
            // current_ptr 判空:从核首调度时本核 CURRENT 尚为 null
            if new != cur {
                // 切换判据:cur 不在跑(阻塞/退出/尚未首调度)必切;
                // cur 在跑时仅当新任务优先级不低(数字不大)于它才切——
                // SMP 下伪 IPI 天然存在(电平合并/选核广播),不能让
                // idle(16)或更低优先级任务顶掉在跑任务(否则每 tick 的
                // 公共出口都会把在跑任务踢回就绪队列,抖动且跨核迁移放大)
                let switch = match cur.as_mut() {
                    None => true,
                    Some(cur) => {
                        cur.state != State::Running
                            || (!new.is_null() && (*new).priority <= cur.priority)
                    }
                };
                if switch {
                    if let Some(new) = new.as_mut() {
                        crate::sprint!("P{:p}", new); // DEBUG: pop 到的任务指针
                        // DEBUG: 打印将恢复帧的控制槽 [13]spsr [14]lr [15]sp_svc
                        let fr = (*new).sp as *const u32;
                        if !fr.is_null() {
                            crate::sprint!(
                                "[{:x}|{:x}|{:x}]",
                                unsafe { fr.add(13).read_volatile() },
                                unsafe { fr.add(14).read_volatile() },
                                unsafe { fr.add(15).read_volatile() }
                            );
                        }
                        if let Some(old) = xworker.execute(new).and_then(|item| item.as_mut()) {
                            crate::sprint!("S"); // DEBUG: execute 返回(切换语义完成)
                            //检查是否栈溢出
                            old.stack_overflow();
                            submit_task(old);
                        }
                    }
                } else if new != IDLE_TASKS[(Porting::hart_id() as usize).min(MAX_HARTS - 1)] {
                    // 不切且弹出的不是 idle(idle 是捏造的,本就不在队列):
                    // 把弹出的任务放回就绪队列,否则它从调度器视野里丢失
                    push_ready(new);
                }
            }
        })
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
/// 如果任务队列里没有就绪任务，则返回本核 IDLE 任务
/// 不变式：`READY_BITS` 位 i 置位 ⟺ `READYQ[i]` 非空（push/pop 双侧维护，
/// 不一致时 pop 侧自清位并在 debug 下断言——F4：修前一致性纯靠约定）
/// 亲和性(SMP):本核只取"未绑核"或"绑到本核"的任务;队首绑往别核时
/// 跳过它继续扫同队,整队都绑往别核则留给对应核,本核降档试下一优先级
/// ——绑核任务不会被错核抢走,也不会堵死本核的更低优先级就绪任务
#[inline(always)]
unsafe fn pop_ready() -> *mut Task {
    let me = Porting::hart_id();
    let idle = IDLE_TASKS[(me as usize).min(MAX_HARTS - 1)];
    let mut bits = READY_BITS;
    while bits != 0 {
        let tz = bits.trailing_zeros() as usize;
        bits &= bits - 1; //本档取不到可跑任务时,降档试下一优先级
        let q = &mut READYQ[tz];
        if q.is_empty() {
            debug_assert!(false, "READY_BITS 位{tz}置位但队列为空——不变式被破坏");
            READY_BITS.set_bit(tz, false);
            continue;
        }
        let pos = q.iter().position(|&t| (*t).hwid.map_or(true, |h| h == me));
        if let Some(i) = pos {
            //position 已确认存在,remove 必成功
            let task = q.remove(i).expect("READYQ 元素在 position 后消失");
            if q.is_empty() {
                READY_BITS.set_bit(tz, false);
            }
            return task;
        }
    }
    idle
}

/// 推入就绪队列
#[track_caller]
/// 入队任务比某核当前任务优先级更高(数字更小)则向该核发 IPI 请求调度。
/// 调度器未启动(该核 CURRENT=null)的核跳过——任务已入队,
/// 该核 start()/首调度自然会调度到它。
/// SMP:遍历全部在线核选核投递(ch25 路线③)——idle 核(优先级 16)
/// 天然被任意任务"抢占",跨核唤醒由此闭环。
/// 亲和性:绑核任务只投给绑定核(pop_ready 在别核也会跳过它,
/// 投给别核只是空转一次调度)
pub(crate) unsafe fn request_preempt_if_higher(task: *mut Task) {
    let n = Porting::core_count().min(MAX_HARTS as u16);
    let pinned = (*task).hwid;
    for h in 0..n {
        if pinned.map_or(false, |p| p != h) {
            continue;
        }
        let cur = super::xworker::current_ptr_at(h);
        if cur.is_null() {
            continue;
        }
        if (*task).priority < (*cur).priority {
            Porting::irq_to(h);
        }
    }
}

/// 运行期改任务优先级(必须在 `sync::free` 内):把"优先级字段"和
/// "任务此刻所处队列"两个事实一起搬走——`READYQ` 的下标 = 优先级-1,
/// 只改字段不改桶会破坏"READY_BITS ⟺ 队列非空"不变式(pop 侧 debug 断言)。
/// 其余状态(Blocked 在延时队列/DELAY 按 wake_tick 排、Suspended 由
/// lock_core 按新优先级重排其锁等待队列)只改字段。
pub(crate) unsafe fn set_priority(task: *mut Task, new_prio: u8) {
    let t = &mut *task;
    debug_assert!((1..=16).contains(&new_prio), "非法优先级 {new_prio}");
    if t.priority == new_prio {
        return;
    }
    let old = t.priority;
    t.priority = new_prio;
    if t.state == State::Ready {
        let ptr = task;
        if let Some(from) = &mut t.queue {
            (*from).retain(|&x| x != ptr);
            if (*from).is_empty() && (old as usize) <= 16 {
                READY_BITS.set_bit(old as usize - 1, false);
            }
        }
        // SMP 在核竞态(wakeup 的 is_current_any 同款守卫):任务可能刚被别核
        // wakeup 置 Ready、却仍挂在某核 CURRENT 上(临界区内 block 与出区后
        // yield 让出之间的窗口)——此刻把它推进就绪队列,第三核会把它弹出
        // 并发执行同一个任务(>1 核下同任务双跑,整机挂死)。仍在核上的不入队:
        // 它让出时 do_schedule 的 old 路径(submit_task,state==Ready)会按
        // 新优先级把它补进正确的就绪桶,恰好一份
        if !super::xworker::is_current_any(task) {
            push_ready(task);
            // 抬升后的任务若比某核当前任务更急,立刻投 IPI——否则要等下个 tick
            // 才有调度机会,PI 的关键"尽快放锁"就打了折扣
            request_preempt_if_higher(task);
        }
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
        // DEBUG: 指认调用点(LR 在 panic! 展开前 = 调用者返回地址)
        let mut sp: usize = 0;
        let mut lr: usize = 0;
        unsafe {
            core::arch::asm!("mov {0}, sp", out(reg) sp);
            core::arch::asm!("mov {0}, lr", out(reg) lr);
        }
        crate::sprintln!("PUSH_READY(NULL) sp={:#x} lr={:#x}", sp, lr);
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

/// 每核空闲任务(按 mhartid 索引)——该核没有就绪任务时切到本核 idle;
/// 每核一个:同一 idle 任务块绝不能在两核并发运行(state 字段会撕裂)
pub(crate) static mut IDLE_TASKS: [*mut Task; MAX_HARTS] = [core::ptr::null_mut(); MAX_HARTS];

/// 1-16 优先级任务就绪队列（下标 = 优先级-1），数字越小优先级越高。
/// `VecDeque::new` 是 const——编译期初始化，运行期零惰性初始化
/// （F4+F5：修前是 16 个 `Option` 静态量 + `INITED` 惰性 init，
/// 32 处手写 match 臂、多核启动下有良性竞争，一并结构性消除）
pub(crate) static mut READYQ: [TaskQueue; 16] = [const { VecDeque::new() }; 16];

/// 延时队列——按 wake_tick 升序（push_delay 有序插入维护）
pub(crate) static mut DELAY: TaskQueue = VecDeque::new();

/// 延时队列队首的到期拍(tickless 空闲引擎取期限用;与 do_systick 同锁读)
#[inline]
pub(crate) unsafe fn next_delay_tick() -> Option<u64> {
    // SAFETY: 与 take_expired 同款访问约定——队内指针均有效,调用方持锁
    DELAY.front().map(|t| unsafe { (**t).wake_tick })
}

/// 是否有任务处于就绪态(tickless 空闲引擎在睡眠前先看这一眼——
/// 恒定节拍下 idle 靠"第一拍"被踢出,dynamic 下必须主动让出。
/// 调用约定:仅在 tickless 门控的单核语义下、由 tickless_idle 在其
/// 临界区内调用——与 tick ISR 一侧的写(`push_ready`/READY_BITS)
/// 互斥,读到的位图是决策时刻的一致快照)
#[inline]
pub(crate) unsafe fn has_ready() -> bool {
    // SAFETY: 调用方持临界区,与写侧(tick ISR 的 push_ready)同锁互斥;
    // 唯一调用点 tickless_idle 在同一临界区内先后调用,无并发写者
    unsafe { READY_BITS != 0 }
}

/// 最高优先级就绪桶下标(R5 口 irq_preempt_check 用:READY_BITS 尾零
/// 即最高优先级桶;空队列返回 16)。调用方需处于关中断上下文
#[inline]
pub(crate) unsafe fn highest_ready_prio() -> u32 {
    unsafe { READY_BITS.trailing_zeros() }
}

/// 测试清场(仅 host 单测):wakeup 会把就绪任务推进全局 READYQ——回收
/// 任务前必须清桶清位图,否则回收后的悬垂指针留在就绪队列里,任何后续
/// 驱动调度器的 host 测试都会解引用已释放的 Task。
#[cfg(test)]
pub(crate) unsafe fn clear_readyq_for_test() {
    for q in READYQ.iter_mut() {
        q.clear();
    }
    READY_BITS = 0;
}

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
