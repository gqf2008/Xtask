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
        let el = now.wrapping_sub(armed) / super::TICK_PERIOD;
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

/// 机器外部中断(mcause=11):PLIC 门控的全部外部中断走这里。
/// 现状只接 UART RX(ch29 章末练习 1):claim 后按源 id 分派,完成后
/// 写回 claim(PLIC 握手)。UART 侧清空接收 FIFO(RBR 读即出栈),再调
/// 例程回调(唤醒/通知——回调必须绝不停留)
#[no_mangle]
unsafe extern "C" fn ExternalIrqHandler() {
    let claim = (super::PLIC_HART0_CLAIM as *mut u32).read_volatile();
    if claim != 0 {
        if claim == super::UART0_IRQ_ID {
            let rbr = (super::UART0_BASE + 0) as *mut u8;
            let lsr = (super::UART0_BASE + 5) as *const u8;
            while lsr.read_volatile() & 1 != 0 {
                rbr.read_volatile();
            }
            let cb = super::UART_RX_CALLBACK.load(core::sync::atomic::Ordering::Relaxed);
            if cb != 0 {
                // SAFETY: 槽里只存过经 uart_set_rx_callback 写入的 fn() 指针,
                // 存/取都以 usize 作位模式搬运(fn 指针与 usize 同宽)
                let cb: fn() = core::mem::transmute(cb);
                cb();
            }
        }
        (super::PLIC_HART0_CLAIM as *mut u32).write_volatile(claim);
    }
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
