use crate::port::{Portable, Porting, MAX_HARTS};
use crate::task::executor::Executor;
use crate::task::Task;

/// 每核当前任务指针表(按 mhartid 索引)。
/// 符号地址即数组基址 = 槽 0 的地址——单核口的汇编按原样
/// `lw t0, CURRENT_TASK_PTR` 读到的正是槽 0,零改动;多核口(qemu_riscv)
/// 在汇编里按 `mhartid*4` 偏移寻址本核槽位(ch25 改造路线②)
#[cfg(feature = "xtask_executor")]
#[export_name = "CURRENT_TASK_PTR"]
static mut CURRENT_TASK: [*mut Task; MAX_HARTS] = [core::ptr::null_mut(); MAX_HARTS];

/// 单硬件线程默认实现
#[cfg(feature = "xtask_executor")]
pub(crate) type XTaskExecutor = ();

/// 本核槽下标(mhartid;恒 < MAX_HARTS——_max_hart_id 闸保证)
#[cfg(feature = "xtask_executor")]
#[inline(always)]
fn hart() -> usize {
    debug_assert!((Porting::hart_id() as usize) < MAX_HARTS);
    Porting::hart_id() as usize
}

/// 本核当前任务裸指针(不 unwrap——调度器启动前为 null,调用方须判空)。
/// submit_task 的抢占检查用它:spawn 阶段 CURRENT_TASK 尚为 null
#[cfg(feature = "xtask_executor")]
pub(crate) unsafe fn current_ptr() -> *mut Task {
    current_ptr_at(Porting::hart_id() as u16)
}

/// 指定核的当前任务裸指针——SMP 抢占路由(irq_to 选核)要读别核的槽。
/// 对齐字读写单拷贝原子,读到稍旧值只会让 IPI 多发/漏发一次,不撕裂
#[cfg(feature = "xtask_executor")]
pub(crate) unsafe fn current_ptr_at(hart: u16) -> *mut Task {
    CURRENT_TASK[(hart as usize).min(MAX_HARTS - 1)]
}

impl Executor for XTaskExecutor {
    fn threads() -> u16 {
        Porting::core_count()
    }
    fn current(&self) -> &'static mut Task {
        unsafe { CURRENT_TASK[hart()].as_mut().unwrap() }
    }
    fn execute(&self, task: *mut Task) -> Option<*mut Task> {
        unsafe {
            if let Some(task) = task.as_mut() {
                task.run();
                Some(core::mem::replace(&mut CURRENT_TASK[hart()], task))
            } else {
                None
            }
        }
    }
    fn halt(&self) {
        panic!("halt")
    }
}
