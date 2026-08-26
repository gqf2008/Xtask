//! 移植层定义&配置

#[cfg(all(feature = "gd32vf103", not(test)))]
pub use crate::chip::gd32vf103::Gd32vf103Porting as Porting;

#[cfg(all(feature = "stm32f4", not(test)))]
pub use crate::chip::stm32f4::STM32F4Porting as Porting;

#[cfg(all(feature = "stm32f1", not(test)))]
pub use crate::chip::stm32f1::STM32F1Porting as Porting;

#[cfg(all(feature = "rp2040", not(test)))]
pub use crate::chip::rp2040::RP2040Porting as Porting;

#[cfg(all(feature = "stm32h7", not(test)))]
pub use crate::chip::stm32h7::STM32H7Porting as Porting;

#[cfg(all(feature = "cm32m4", not(test)))]
pub use crate::chip::cm32m4::CM32M4Porting as Porting;
#[cfg(all(feature = "ch32v307", not(test)))]
pub use crate::chip::ch32v307::Ch32v307Porting as Porting;
#[cfg(all(feature = "ch32v203", not(test)))]
pub use crate::chip::ch32v203::Ch32v203Porting as Porting;
#[cfg(all(feature = "ch32v103", not(test)))]
pub use crate::chip::ch32v103::Ch32v103Porting as Porting;
#[cfg(all(feature = "esp32c3", not(test)))]
pub use crate::chip::esp32c3::Esp32c3Porting as Porting;
#[cfg(all(feature = "qemu_riscv", not(test)))]
pub use crate::chip::qemu_riscv::QemuRiscvPorting as Porting;

#[cfg(all(
    not(test),
    not(any(
        feature = "gd32vf103",
        feature = "stm32f4",
        feature = "stm32f1",
        feature = "rp2040",
        feature = "stm32h7",
        feature = "cm32m4",
        feature = "ch32v307",
        feature = "ch32v203",
        feature = "ch32v103",
        feature = "esp32c3",
        feature = "qemu_riscv"
    ))
))]
pub use DefaultPorting as Porting;

// host 测试环境：提供一个可运行的 Porting mock，让纯逻辑（信号量、队列、总线、延时队列）
// 能在 `cargo test` 下被驱动。单线程语义下临界区只是一个标记，无需真实关中断。
#[cfg(test)]
pub use HostPorting as Porting;

use crate::task::Task;
use bare_metal::CriticalSection;

/// 内核数组定界用的核数上限——实际参与调度的核数由 `Porting::core_count()`
/// 运行期决定(≤ MAX_HARTS)。CURRENT_TASK/IDLE_TASKS/临界区深度等
/// 每核数组一律按此定界(ch25 改造路线②)
pub(crate) const MAX_HARTS: usize = 16;

/// 移植层接口定义
pub trait Portable {
    /// 完全内存屏障
    /// 保证在屏障之前的任何存储操作先于屏障之后的代码执行。
    fn barrier();
    /// 临界区保护函数
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R;
    /// 开全局中断
    fn enable_interrupt();
    /// 关全局中断
    fn disable_interrupt();
    /// 启动调度器
    fn start_scheduler() -> !;
    /// 软中断
    fn irq();
    /// 关闭软中断
    fn disable_irq();
    /// 获取systick
    fn systick() -> u64;
    /// 硬件延时，单位us
    fn delay_us(us: u64);
    /// 保存任务环境到任务栈
    fn save_context(task: &mut Task);

    // ---- SMP 扩展面(第 25 章改造路线②③)----
    // 三个方法全部带单核默认实现:现有各口零改动、行为逐字不变;
    // 多核口按需覆盖。见 book/src/ch25-smp.md。

    /// 当前核(hart)ID——SMP 口按 mhartid;单核口恒 0(默认)
    #[inline]
    fn hart_id() -> u16 {
        0
    }
    /// 参与调度的物理核数——单核恒 1(默认)。
    /// 注意:这是"内核调度会使用的核数",不是硅片上的核数;
    /// hart0-only 双核起跑阶段仍返回 1
    #[inline]
    fn core_count() -> u16 {
        1
    }
    /// 向指定核发软中断(IPI)——默认退化为本核 `irq()`;
    /// SMP 口按目标核寻址(CLINT MSIP 本就是 per-hart 寄存器)
    #[inline]
    fn irq_to(hart: u16) {
        let _ = hart;
        Self::irq();
    }
    /// 启动从核参与调度——单核默认空操作;多核口唤醒停泊的从核
    /// (在 `start_scheduler` 之前由调度器调用,此刻就绪队列/每核 idle 已就绪)
    #[inline]
    fn start_secondary_cores() {}

    // ---- tickless 动态节拍扩展面(第 29 章)----
    // 四个方法全部带"恒定节拍"默认实现:现有各口零改动、行为逐字不变
    // (idle 会自旋等中断,与旧 `loop {}` 等价);要省电的口按需覆盖。
    // 见 book/src/ch29-tickless.md。

    /// 本口是否支持 tickless 动态节拍(一次性节拍定时器 + wfi 等待)
    #[inline]
    fn tickless_supported() -> bool {
        false
    }
    /// 把节拍定时器重装为"delta 拍后触发一次"的一次性模式:
    /// 到时产生一次节拍中断(与恒定节拍的重装相对),由中断路径实测
    /// 时长跳账(`scheduler::systick_jump`)。默认实现按恒定节拍语义
    /// 忽略——节拍恒在,无"武装"概念
    #[inline]
    fn tickless_arm_delta(_delta_ticks: u64) {}
    /// 停掉节拍定时器(无期限可睡时):冻结 tick 计数——tick() 是运行时
    /// 时钟,冻结是正确的语义(墙钟仍走 `systick()`);被外部中断唤醒
    /// 后由 idle 重新决策并再次武装
    #[inline]
    fn tickless_stop_timer() {}
    /// 睡眠等待中断(实现 wfi/wfe):被任意已使能中断唤醒即返回。
    /// 默认实现空操作——调用方(恒定节拍口)不会走到这里
    #[inline]
    fn tickless_wait() {}
}

/// host 测试用移植层 mock。
/// 仅在 `cfg(test)` 下编译。单线程测试语义下：
/// - `irq`/`save_context` 等为空调用，因为 host 不做真实任务切换；
/// - `systick`/`delay_us` 给确定值，避免测试依赖真实时钟。
#[cfg(test)]
pub struct HostPorting;

/// host 测试临界区锁。`cargo test` 默认多线程并行跑测试，仅靠"测试不并发访问全局状态"
/// 的口头约定太脆：一旦将来某个测试驱动了全局队列/TICKS，就会在测试线程间静默数据竞争。
/// 用进程内互斥锁给临界区提供真实互斥。注意 std Mutex 不可重入——
/// 测试代码不要嵌套调用 sync::free（free 里再 free 会死锁）。
#[cfg(test)]
static HOST_TEST_CS_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
impl Portable for HostPorting {
    fn barrier() {}
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        // SAFETY: host 上无真实中断可屏蔽，CriticalSection 仅为 API 形状匹配的标记；
        // 互斥由上面的进程内锁提供。锁被 panic 毒化后继续取用，避免连锁 panic 掩盖首个失败。
        let _guard = HOST_TEST_CS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        f(unsafe { &CriticalSection::new() })
    }
    fn enable_interrupt() {}
    fn disable_interrupt() {}
    fn start_scheduler() -> ! {
        unimplemented!("host 测试不启动调度器")
    }
    fn irq() {}
    fn disable_irq() {}
    fn systick() -> u64 {
        0
    }
    fn delay_us(_us: u64) {}
    fn save_context(_task: &mut Task) {}
}

/// 移植层默认实现
pub type DefaultPorting = ();

/// 默认实现
impl Portable for DefaultPorting {
    /// 完全内存屏障
    /// 保证在屏障之前的任何存储操作先于屏障之后的代码执行。
    fn barrier() {
        unimplemented!()
    }
    fn free<F, R>(_f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        unimplemented!()
    }

    /// 开全局中断
    fn enable_interrupt() {
        unimplemented!()
    }
    /// 关全局中断
    fn disable_interrupt() {
        unimplemented!()
    }
    /// 启动调度器
    fn start_scheduler() -> ! {
        unimplemented!()
    }
    /// 开启软中断
    fn irq() {
        unimplemented!()
    }
    /// 关闭软中断
    fn disable_irq() {
        unimplemented!()
    }
    /// 获取rtc tick
    fn systick() -> u64 {
        unimplemented!()
    }
    /// 硬件延时，单位us
    fn delay_us(_us: u64) {
        unimplemented!()
    }
    /// 保存任务环境到任务栈
    fn save_context(_task: &mut Task) {
        unimplemented!()
    }
}
