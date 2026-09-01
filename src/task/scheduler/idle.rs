//! idle 任务:tickless 动态节拍引擎(第 28 章)+ 恒定节拍自旋兜底。

use core::ffi::c_void;

use crate::port::{Portable, Porting, MAX_HARTS};
use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::task::scheduler::xtask::{self, IDLE_TASKS};
use crate::task::scheduler;
use crate::{Task, IDLE_TASK_NAME};

/// tickless 空闲三态决策(第 28 章)——纯函数,host 可测:
/// 由"当前拍 + 最近期限"定出空闲动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IdleDecision {
    /// 无任何期限——停表睡到外部中断(tick 冻结)
    SleepForever,
    /// 期限已到——立即按"到点处理"语义走一次完整 tick(记 1 拍 + 摘到期)
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
            } else {
                // 恒定节拍:原地自旋等中断(每拍 tick ISR 都要进来推动
                // 时间片/抢占;tickless 引擎的睡眠只发生在 tickless_idle 内部)。
                // 但"停表长眠"期间若有人把开关关掉(SleepForever 的唤醒
                // 之外,ISR 侧 set_enabled(false) 可直接发生),定时中断仍
                // 被 mask——先补回恒定节拍(mie/cmp),否则自旋等不到任何
                // 时钟推动,整机饿死。口侧自检幂等:未停表时零成本返回
                Porting::tickless_resume_periodic();
                core::hint::spin_loop();
            }
        }
    }

    let n = Porting::core_count().min(MAX_HARTS as u16);
    for h in 0..n {
        let task = Task::new(IDLE_TASK_NAME, 512, 16, idle_task, core::ptr::null_mut());
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
/// 然后三选一(停表长眠 / 一次性武装深睡 / 到点处理)。**决策+行动+
/// 入眠全程一个临界区**:不是为防 ISR 并发改内核状态(那由同一把锁保证),
/// 而是关死"决策→行动"之间的状态漂移窗口——窗口内到达的中断一律
/// pending,出区才被取走,ISR 看到的总是决策当时的一致状态;不会出现
/// "MSIP 已把某任务标就绪、idle 却按旧状态停表长眠/按旧期限武装"的
/// 竞态。wfi 在区内执行是合法惯例(RISC-V:已使能的中断 pending 即
/// 唤醒 wfi,与全局 mstatus.MIE 无关;pending 的中断出区即 trap,不丢)
fn tickless_idle() {
    sync::free(|_| unsafe {
        // 恒定节拍下,idle 靠"下一拍"把就绪任务踢出;tickless 没有拍可等:
        // 就绪队列非空(例如 start() 之前已 spawn 的任务)必须主动让出,
        // 否则整机停在 idle 里等一个永远不会来的期限。
        if xtask::has_ready() {
            crate::yield_now();
            return;
        }
        let now = crate::time::tick();
        let delay_next = xtask::next_delay_tick();
        #[cfg(feature = "timer")]
        let timer_next = crate::timer::next_timer_tick();
        #[cfg(not(feature = "timer"))]
        let timer_next = None;
        match decide_idle(now, combine_deadline(delay_next, timer_next)) {
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
                // 早醒(睡眠期间来了外部中断等)时 trap 出口 do_schedule
                // 会对"停留 idle"也调 tickless_leave_idle:按实测补账
                // (TICKS += el)再回本函数重决——delta 随实测收缩,期限
                // 不漂移(修前早醒无任务就绪时不补账,冻结 TICKS 重武装
                // 会把期限无限推后)
                Porting::tickless_arm_delta(delta);
                Porting::tickless_wait();
            }
            IdleDecision::ProcessNow => {
                // 期限已到而当前拍尚未跳(可达输入:Timer::after(0) 未拦、
                // 延时入队时出现空档等)。按"到点处理"的真实语义走一次
                // 完整 tick:记 1 拍账 + 摘延时队列 + 驱动软定时器堆 +
                // 抢占检查(即 scheduler::systick)——之后过期期限必被
                // 清理,下一轮决策按新状态进行。绝不空转
                scheduler::systick();
            }
        }
    });
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
        // 提前 1 拍:已到期,按到点处理走一次完整 tick
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
