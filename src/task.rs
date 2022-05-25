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
}

impl<'a> TaskBuilder<'a> {
    pub fn new() -> Self {
        Self {
            stack_size: 256,
            name: "",
            priority: 8,
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
    pub(crate) delay_ticks: usize,
    pub(crate) id: u16,
    pub(crate) priority: u8,
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
            delay_ticks: 0,
            name: name.to_string(),
            id: 1,
            priority,
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
    /// 这个函数如果在用户任务里调用需要临界区保护
    pub(crate) fn wakeup(&mut self) {
        if self.state == State::Suspended {
            self.state = State::Ready;
            unsafe { scheduler::xtask::submit_task(self) };
        }
    }
    //任务退出，立即立刻中断
    pub(crate) fn exit(&mut self) {
        self.state = State::Terminated;
        yield_now();
    }

    #[track_caller]
    #[inline(always)]
    pub(crate) fn bind(&mut self, target: &'static mut TaskQueue) {
        let ptr = self as *mut Task;
        if let Some(from) = &mut self.queue {
            if *from != target {
                (*from).retain(|item| *item != ptr);
            }
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
    /// 滴答
    /// 总tick累加，延时tick累减，延时等于0且
    /// 任务状态为阻塞时任务状态变更为就绪状态
    #[inline(always)]
    pub(crate) fn tick(&mut self) -> bool {
        if self.delay_ticks > 0 {
            self.delay_ticks -= 1;
            if self.delay_ticks == 0 {
                self.state = State::Ready;
            }
        } else {
            self.ticks += 1;
        }
        self.state == State::Ready
    }

    /// 暂停一定tick数，状态变更为阻塞状态
    /// 触发软中断切换任务
    /// 当前任务等待，在当前任务调用
    #[inline]
    pub(crate) fn wait(&mut self, ticks: usize) {
        self.state = State::Blocked;
        self.delay_ticks = ticks;
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
            "Task(id: {}, name: {}, ticks: {}, delay_ticks: {}, priority: {}, state: {:?})",
            self.id, self.name, self.ticks, self.delay_ticks, self.priority, self.state
        )
    }
}
