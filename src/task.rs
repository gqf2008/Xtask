pub(crate) mod executor;
pub(crate) mod scheduler;

use crate::ms2ticks;
use crate::port::{Portable, Porting};
use crate::task::executor::{xworker, Executor};
use crate::task::scheduler::{schedulee, Scheduler};
use alloc::collections::VecDeque;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{boxed::Box, string::String};
use core::ffi::c_void;
use core::fmt::Display;
use core::ptr;

pub static IDLE_TASK_NAME: &str = "@idle";
pub static TIMER_TASK_NAME: &str = "@timer";
pub static DEBUG_TIMER_NAME: &str = "@debug";

/// 外部调用，创建任务
pub fn spawn<F: FnOnce() + Send + 'static>(f: F) {
    TaskBuilder::new().spawn(f)
}

/// 毫秒级任务延时，如果小于tick周期，则不处理
/// 禁止在中断服务中调用
#[inline]
pub fn sleep_ms(ms: usize) {
    xworker.current().sleep_ms(ms);
}

/// 中断当前任务
/// 中断服务和用户程序里都可以调用
#[track_caller]
#[inline]
pub fn yield_now() {
    Porting::irq();
}

/// 硬件延时
#[inline]
pub fn delay_us(us: u64) {
    Porting::delay_us(us);
}

pub struct TaskBuilder<'a> {
    stack_size: usize,
    name: &'a str,
    priority: u8,
    hwid: Option<u16>,
}

impl<'a> TaskBuilder<'a> {
    pub fn new() -> Self {
        Self {
            stack_size: 256,
            name: "",
            priority: 8,
            hwid: None,
        }
    }
    pub fn stack_size(mut self, size: usize) -> Self {
        assert!(size > 64);
        self.stack_size = size;
        self
    }
    pub fn priority(mut self, priority: u8) -> Self {
        assert!(priority > 0);
        self.priority = priority;
        self
    }
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    /// 绑核(亲和性):任务只在指定 hart 上被调度,SMP 下获得确定性放置。
    /// 单核口只有 hart 0。绑到不在线的核 = 任务永远饥饿(调度器不代为纠正)
    pub fn affinity(mut self, hart: u16) -> Self {
        assert!((hart as usize) < crate::port::MAX_HARTS);
        self.hwid = Some(hart);
        self
    }

    pub fn spawn<F: FnOnce() + Send + 'static>(self, f: F) {
        fn entry(args: *mut c_void) {
            unsafe {
                let b = Box::from_raw(args as *mut Box<dyn FnOnce()>);
                b();
            }
        }
        let f: Box<Box<dyn FnOnce() + Send + 'static>> = Box::new(Box::new(f));
        let args = &*f as *const _ as *mut c_void;
        let task = Task::new(self.name, self.stack_size, self.priority, entry, args);
        unsafe {
            (*task).hwid = self.hwid;
            debug_assert!(
                self.hwid.map_or(true, |h| h < Porting::core_count()),
                "绑核目标 {} 不在线(在线核数 {})——任务将永远饥饿",
                self.hwid.unwrap_or(0),
                Porting::core_count()
            );
        }
        core::mem::forget(f);
        schedulee.submit(task);
    }
}

/// 任务状态
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ready,      //就绪，待调度/正在运行
    Running,    //运行状态
    Suspended,  //挂起，在等待一个恢复信号
    Blocked,    //阻塞，延时时间到了会自动回复到ready状态，信号量wait也会挂起
    Terminated, //终止，延时时间到了会自动回复到ready状态
}

/// 任务函数
pub type Func = fn(*mut c_void);

/// 一个任务最多同时持有的互斥锁数目(持锁集合数组容量)。
/// 完整 PI 重算以集合为输入;嵌套深度超过 8 层的任务现实中不存在,
/// 一旦触顶 debug 断言,超出部分不入账(该部分锁上的继承会退化为
/// "等待者重试时再补抬"的经典行为)。
pub(crate) const HELD_MAX: usize = 8;

/// 任务队列
pub type TaskQueue = VecDeque<*mut Task>;

/// 栈围栏标志，用于检测是否存在栈溢出
/// 如果这个值改遍了那么可能栈溢出了，系统
/// 应该尽快介入处理，以防止发生更严重的错误
/// 这个方法只有在任务切换时才会检测到，存在一定的滞后性
/// 所以关于如何防止栈溢出最好是根据业务情况合理的分配空间大小
/// 在开发、测试过程中要确定最坏的情况
pub static STACK_FENCE: usize = 0xFE_CE;

/// 任务定义
#[repr(C)]
#[derive(Debug)]
pub struct Task {
    pub(crate) sp: usize,                             //任务栈顶指针
    pub(crate) stack: *mut usize,                     //栈空间，指向栈底地址
    pub(crate) entry: Func,                           //任务入口
    pub(crate) args: *mut c_void,                     //任务参数
    pub(crate) queue: Option<&'static mut TaskQueue>, // 当前任务队列队列
    pub(crate) name: String,
    pub(crate) stack_size: usize,
    pub(crate) ticks: usize,
    /// 绝对到期时刻（tick 计）——延时队列按它升序排列，tick 中断只查队首，
    /// 不再每 tick 全队列递减（F3：消掉每 tick 的 O(n) 扫描与 ISR 内分配）
    pub(crate) wake_tick: u64,
    pub(crate) id: u16,
    pub(crate) priority: u8,
    /// **出生优先级**(spawn 时的值)。优先级继承(PI)把本任务临时抬高后,
    /// 释放锁时回落的目标——"继承的要还"。
    pub(crate) base_priority: u8,
    /// 我在等的互斥锁内核(仅互斥类锁的阻塞路径设置,认领成功清除)。
    /// 优先级继承链靠它上传:"等锁 → 谁在持锁 → 持锁者又在等谁"。
    /// 普通信号量阻塞不设(PI 链止于非互斥原语,与 FreeRTOS/Zephyr 一致)。
    pub(crate) blocked_lock: *mut crate::sync::lock_core::LockCore,
    /// **持锁集合**:本任务当前持有的所有互斥锁内核(认领成功压入,深度归零
    /// 释放时摘出)。完整 PI 语义的根据——释放一把锁时,继承优先级从"剩余
    /// 每把锁的队首等待者"重算,而不是一把锁一把锁地临时回落。
    pub(crate) held_locks: [*mut crate::sync::lock_core::LockCore; HELD_MAX],
    /// 持锁集合当前长度(0..=HELD_MAX;溢出时 debug 断言,超出部分不记账)
    pub(crate) held_count: u8,
    pub(crate) hwid: Option<u16>,
    pub(crate) state: State,
}

impl Task {
    pub fn id(&self) -> u16 {
        self.id
    }
    pub fn state(&self) -> State {
        self.state
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// 暂停一会儿
    pub(crate) fn sleep_ms(&mut self, ms: usize) {
        if ms > 0 {
            let ticks = ms2ticks(ms);
            if ticks > 0 {
                self.wait(ticks);
                yield_now();
            }
        }
    }
}

impl Task {
    pub(crate) fn new<S: ToString>(
        name: S,
        stack_size: usize,
        priority: u8,
        entry: Func,
        args: *mut c_void,
    ) -> *mut Task {
        //从堆上分配任务空间
        let mut task = Box::new(Task {
            sp: 0,
            stack: ptr::null_mut(),
            entry,
            args,
            queue: None,
            stack_size,
            ticks: 0,
            wake_tick: 0,
            name: name.to_string(),
            id: 1,
            priority,
            base_priority: priority,
            blocked_lock: ptr::null_mut(),
            held_locks: [ptr::null_mut(); HELD_MAX],
            held_count: 0,
            hwid: None,
            state: State::Ready,
        });
        //从堆上分配任务栈空间，多申请一个字用于保存栈围栏标志
        let mut stack: Vec<usize> = Vec::with_capacity(stack_size + 1);
        unsafe {
            //任务栈指针
            let stack_ptr = stack.as_mut_slice().as_mut_ptr();
            //写入围栏标志
            stack_ptr.write_volatile(STACK_FENCE);
            task.stack = stack_ptr;
            //泄漏栈空间，以便绕过RUST所有权机制，任务结束时需要手动释放
            core::mem::forget(stack);
        }
        if task.name == IDLE_TASK_NAME {
            task.id = 0;
        }
        //调用移植层保存任务状态到任务栈
        Porting::save_context(task.as_mut());
        //泄漏任务，绕过RUST所有权机制，任务结束时需要手动释放
        let raw = Box::into_raw(task);
        raw
    }
}
impl Task {
    /// 挂起任务，立即立刻中断
    /// 这段代码需要临界区保护，禁止在中断里调用
    pub(crate) fn block(&mut self) {
        self.state = State::Suspended;
        self.queue = None;
    }
    /// 唤醒任务，进入就绪队列待调度
    /// 内部已含临界区（检查+改状态+入队同一区）：裸任务上下文、ISR、
    /// 已在临界区内的调用方都可直接调用——临界区可重入，嵌套安全（F1 修复：
    /// 修前互斥靠注释约定，漏包临界区的调用方会与 ISR 并发撕队列）
    pub(crate) fn wakeup(&mut self) {
        let ptr = self as *mut Task;
        crate::sync::free(|_| unsafe {
            // SAFETY: ptr 来自上面的 &mut self，唤醒语义保证任务未释放
            if (*ptr).state == State::Suspended {
                (*ptr).state = State::Ready;
                // SMP 关键：任务可能仍挂在某核 CURRENT 上（"临界区内 block()
                // 入队挂起"与"出区后 yield 让出 CPU"之间的窗口——semaphore
                // wait/post 等原语的标准形态）。此刻把它推进就绪队列，第三个
                // 核会把它弹出并发执行同一个任务（>2 核下必现的整机挂死）。
                // 仍在核上的不入队：它让出时 do_schedule 的 old 路径
                // （submit_task(old)，state==Ready）会把它补入就绪队列，恰好一份
                if !scheduler::xworker::is_current_any(ptr) {
                    scheduler::xtask::submit_task(ptr);
                }
            }
        });
    }
    //任务退出，立即立刻中断
    pub(crate) fn exit(&mut self) {
        self.state = State::Terminated;
        yield_now();
    }

    /// 持锁记账:认领成功压入(可重入加深不重复压——同一把锁只记一次)。
    /// 只在 `sync::free` 内被 lock_core 调用。
    pub(crate) fn held_push(&mut self, core: *mut crate::sync::lock_core::LockCore) {
        if (self.held_count as usize) < HELD_MAX {
            self.held_locks[self.held_count as usize] = core;
            self.held_count += 1;
        } else {
            debug_assert!(
                false,
                "任务 {} 同时持锁超过 {HELD_MAX} 把——集合溢出,这部分继承只按经典行为补抬",
                self.name
            );
        }
    }

    /// 持锁记账:深度归零的真正释放时摘出(swap_remove,集合无序——
    /// 完整 PI 重算只看"还持有谁",不看顺序)。返回是否摘到了。
    pub(crate) fn held_remove(&mut self, core: *mut crate::sync::lock_core::LockCore) -> bool {
        for i in 0..self.held_count as usize {
            if ptr::eq(self.held_locks[i], core) {
                let last = self.held_count as usize - 1;
                self.held_locks[i] = self.held_locks[last];
                self.held_locks[last] = ptr::null_mut();
                self.held_count -= 1;
                return true;
            }
        }
        false
    }

    #[track_caller]
    #[inline(always)]
    pub(crate) fn bind(&mut self, target: &'static mut TaskQueue) {
        let ptr = self as *mut Task;
        if let Some(from) = &mut self.queue {
            //无条件先去重：即使 from == target 也要先移除，
            //否则任务仍在队列中时再次 bind 会同任务入队两次，
            //导致重复调度甚至二次释放
            (*from).retain(|item| *item != ptr);
        }
        target.push_back(self);
        self.queue = Some(target);
    }

    #[inline(always)]
    pub(crate) fn ready(&mut self) {
        if self.state == State::Running {
            self.state = State::Ready;
        }
    }

    #[inline(always)]
    pub(crate) fn run(&mut self) {
        if self.state == State::Ready {
            self.state = State::Running;
            self.queue = None;
        }
    }
    /// 暂停一定tick数，状态变更为阻塞状态
    /// 触发软中断切换任务
    /// 当前任务等待，在当前任务调用
    #[inline]
    pub(crate) fn wait(&mut self, ticks: usize) {
        self.state = State::Blocked;
        // 记绝对到期时刻：延时队列按 wake_tick 升序插入（push_delay），
        // tick 中断侧从队首摘取，同刻到期保持 FIFO 唤醒次序
        self.wake_tick = crate::time::tick() + ticks as u64;
    }

    /// 栈围栏标志是否被修改
    /// 如果与围栏标志不一致，很可能内存已经被污染，
    /// 系统恐慌进入异常处理流程
    #[inline(always)]
    pub(crate) fn stack_overflow(&self) {
        unsafe {
            if self.stack.read_volatile() != STACK_FENCE {
                panic!(
                    "stack overflow `{}` stack addr:{:p} sp->0x{:08x}",
                    self.name(),
                    self.stack,
                    self.sp
                )
            }
        }
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        //获取任务栈所有权，离开本方法编译器自动释放
        let _: Vec<usize> =
            unsafe { Vec::from_raw_parts(self.stack, self.stack_size + 1, self.stack_size + 1) };
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Task(id: {}, name: {}, ticks: {}, wake_tick: {}, priority: {}, state: {:?})",
            self.id, self.name, self.ticks, self.wake_tick, self.priority, self.state
        )
    }
}
