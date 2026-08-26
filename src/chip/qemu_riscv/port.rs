//! QEMU virt 机中断胶水:tick(mcause=7)与 yield(mcause=3)处理。

use super::QemuRiscvPorting;
use crate::port::Portable;
use crate::task::scheduler;
use core::arch::global_asm;

global_asm!(include_str!("port.S"), options(raw));

/// tick 处理:tickless 一次性到点 = 实测本次睡眠拍数跳账(TICKS += el)
/// 后照常摘到期;恒定节拍 = 重装 MTIMECMP + 逐拍账。两种路径共用
/// mcause=7 中断,tickless 的武装 flag 区分;电平源在两条路径都须
/// 重写 mtimecmp 防 mret 后立刻重入
#[no_mangle]
unsafe extern "C" fn TickHandler() {
    super::TICK_ISR_COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    let armed = super::TICKLESS_ARMED.get();
    if armed != 0 {
        // 一次性到点:el = 距武装时刻的整拍数(≥ delta——定时器不会
        // 早于 cmp 触发;ISR 延迟超过一拍时 el 会进位,语义仍正确)
        super::TICKLESS_ARMED.set(0);
        let now = QemuRiscvPorting::systick();
        const PERIOD: u64 = (super::SYSTICK_CLOCK_HZ / super::TICK_CLOCK_HZ) as u64;
        let el = now.wrapping_sub(armed) / PERIOD;
        scheduler::systick_jump(el.max(1));
        // 补一击"下一拍"(now + PERIOD):电平源若停在陈旧 cmp 上,
        // mret 后中断条件仍成立,会立即重入——重装即消
        super::reset_systick();
    } else {
        super::reset_systick();
        scheduler::systick();
    }
    // 需要切换时不必显式请求——trap 出口统一走切换路径
}

/// yield 处理:清 MSIP(电平源,防 mret 后立刻重入);
/// 调度统一走 port.S 公共出口的 switch_context
#[no_mangle]
unsafe extern "C" fn YieldHandler() {
    QemuRiscvPorting::disable_irq(); // MSIP 写 0
}

/// 软中断等价入口:port.S 公共出口调用,执行调度与切换
#[no_mangle]
unsafe extern "C" fn switch_context() {
    scheduler::schedule();
}

/// 所有任务的退出函数(port.S 首调蹦床以 `j task_exit` 引用——保持
/// 符号不改名)
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn task_exit() {
    scheduler::exit_current_task();
}
