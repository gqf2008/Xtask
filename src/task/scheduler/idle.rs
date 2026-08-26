//! idle 任务:tickless 动态节拍引擎(第 29 章)+ 恒定节拍自旋兜底。

use core::ffi::c_void;

use crate::port::{Portable, Porting, MAX_HARTS};
use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::task::scheduler::xtask::{self, IDLE_TASKS};
use crate::task::scheduler;
use crate::{Task, IDLE_TASK_NAME};

/// tickless 空闲三态决策(第 29 章)——纯函数,host 可测:
/// 由"当前拍 + 最近期限"定出空闲动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IdleDecision {
    /// 无任何期限——停表睡到外部中断(tick 冻结)
    SleepForever,
    /// 期限已到——先处理到期(防御分支,理论不可达)
    ProcessNow,
    /// 距最近期限 delta 拍——一次性武装后睡
    SleepUntil(u64),
}

/// 由"当前拍 + 最近期限"决定空闲动作
#[inline]
fn decide_idle(now: u64, next_deadline: Option<u64>) -> IdleDecision {
    match next_deadline {
        None => IdleDecision::SleepForever,
        Some(d) if d <= now => IdleDecision::ProcessNow,
        Some(d) => IdleDecision::SleepUntil(d - now),
    }
}

/// 两路期限(延时队列队首 / 软定时器堆顶)取更近者
#[inline]
fn combine_deadline(a: Option<u64>, b: Option<u64>) -> Option<u64> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.min(y)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

/// 启动每核 idle 任务:参与调度的每核各一个(同一任务块不能在两核并发)。
/// 本核先以 idle 为当前任务——`start_scheduler` 恢复的第一帧就是它
pub(crate) fn start_idle_task() {
    fn idle_task(_args: *mut c_void) {
        loop {
            // tickless 引擎:单核(主核独占 tick,ch25 ⑤)+ 口支持 + 应用未关闭。
            // 三条件任一不满足,下面自旋 = 旧 `loop {}` 语义逐字不变
            if Porting::tickless_supported() && !crate::smp::enabled() && crate::tickless::enabled()
            {
                tickless_idle();
            }
            // 恒定节拍:原地自旋等中断(每拍 tick ISR 都要进来推动
            // 时间片/抢占;tickless 引擎的睡眠只发生在 tickless_idle 内部)
            core::hint::spin_loop();
        }
    }

    let n = Porting::core_count().min(MAX_HARTS as u16);
    for h in 0..n {
        let task = Task::new(IDLE_TASK_NAME, 128, 16, idle_task, core::ptr::null_mut());
        unsafe {
            IDLE_TASKS[h as usize] = task;
        }
    }
    unsafe {
        // 只 execute 本核的 idle(置 CURRENT[本核]);别核的 CURRENT
        // 由其进调度后的首个 do_schedule 自行装载
        let _ = xworker.execute(IDLE_TASKS[(Porting::hart_id() as usize).min(MAX_HARTS - 1)]);
    }
}

/// tickless 空闲引擎单轮:先看"有没有想跑的",再看"最近期限在哪",
/// 然后三选一(停表长眠 / 一次性武装深睡 / 补办到期)。全部状态读取在
/// 一个临界区内完成——DELAY/READYQ/软定时器堆与任何任务侧/ISR 侧操作
/// 都要同一把锁(ch25 假设三纪律);行动(武装/停表)在移植层内各自成临界区
fn tickless_idle() {
    // 恒定节拍下,idle 靠"下一拍"把就绪任务踢出;tickless 没有拍可等:
    // 就绪队列非空(例如 start() 之前已 spawn 的任务)必须主动让出,
    // 否则整机停在 idle 里等一个永远不会来的期限。
    unsafe {
        if xtask::has_ready() {
            crate::yield_now();
            return;
        }
    }
    let decision = sync::free(|_| unsafe {
        let now = crate::time::tick();
        let delay_next = xtask::next_delay_tick();
        #[cfg(feature = "timer")]
        let timer_next = crate::timer::next_timer_tick();
        #[cfg(not(feature = "timer"))]
        let timer_next = None;
        decide_idle(now, combine_deadline(delay_next, timer_next))
    });
    match decision {
        IdleDecision::SleepForever => {
            // 无期限可等:停表 + 深度睡——tick() 冻结是正确语义(
            // 运行时时钟,不是墙钟;墙钟走 Porting::systick()/Instant)。
            // 被外部中断(IPI/串口/任意已使能中断)唤醒,回外层循环重决
            Porting::tickless_stop_timer();
            Porting::tickless_wait();
        }
        IdleDecision::SleepUntil(delta) => {
            // 一次性武装:delta 拍整后一次节拍中断,中断路径实测时长
            // 跳账(TICKS += el)后照常摘到期任务——到点之间没有
            // 任何中间拍,这就是"动态节拍"的全部收益。
            // 早醒(睡眠期间来了 MSIP 等)时 ISR 测到 ~0 拍、清掉武装,
            // 无事发生,回循环重决——无害(见第 29 章踩坑:早醒零拍)
            Porting::tickless_arm_delta(delta);
            Porting::tickless_wait();
        }
        IdleDecision::ProcessNow => {
            // 防御分支:期限 ≤ now 而 tick 没跳——不变式"到点即摘"
            // 保证其理论不可达(跳账与摘取同在中断路径完成);保留为
            // 守卫,并让三态决策对全部输入有定义
            let _ = unsafe { scheduler::do_systick_now() };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{combine_deadline, decide_idle, IdleDecision};

    #[test]
    fn decide_idle_no_deadline_freezes() {
        assert_eq!(decide_idle(100, None), IdleDecision::SleepForever);
    }

    #[test]
    fn decide_idle_future_deadline_arms_exact_delta() {
        assert_eq!(decide_idle(100, Some(103)), IdleDecision::SleepUntil(3));
        assert_eq!(decide_idle(0, Some(1)), IdleDecision::SleepUntil(1));
        // 大间隔:delta 就是差值本身,没有 ±1 拍误差
        assert_eq!(decide_idle(5, Some(1000)), IdleDecision::SleepUntil(995));
    }

    #[test]
    fn decide_idle_past_or_equal_deadline_processes() {
        // 提前 1 拍:防御分支
        assert_eq!(decide_idle(100, Some(99)), IdleDecision::ProcessNow);
        // 恰好到点:也是"已到期"——由中断路径的摘取处理
        assert_eq!(decide_idle(100, Some(100)), IdleDecision::ProcessNow);
    }

    #[test]
    fn combine_deadline_takes_nearer() {
        assert_eq!(combine_deadline(Some(10), Some(7)), Some(7));
        assert_eq!(combine_deadline(Some(5), Some(9)), Some(5));
        assert_eq!(combine_deadline(Some(5), None), Some(5));
        assert_eq!(combine_deadline(None, Some(5)), Some(5));
        assert_eq!(combine_deadline(None, None), None);
        // 单边空 = 另一边;两边同值任取
        assert_eq!(combine_deadline(Some(7), Some(7)), Some(7));
    }
}
