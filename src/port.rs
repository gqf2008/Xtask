//! 移植层定义&配置

#[cfg(feature = "gd32vf103")]
pub use crate::chip::gd32vf103::Gd32vf103Porting as Porting;

#[cfg(feature = "stm32f4")]
pub use crate::chip::stm32f4::STM32F4Porting as Porting;

#[cfg(feature = "stm32f1")]
pub use crate::chip::stm32f1::STM32F1Porting as Porting;

#[cfg(feature = "rp2040")]
pub use crate::chip::rp2040::RP2040Porting as Porting;

#[cfg(feature = "stm32h7")]
pub use crate::chip::stm32h7::STM32H7Porting as Porting;

#[cfg(feature = "cm32m4")]
pub use crate::chip::cm32m4::CM32M4Porting as Porting;

#[cfg(not(any(
    feature = "gd32vf103",
    feature = "stm32f4",
    feature = "stm32f1",
    feature = "rp2040",
    feature = "stm32h7",
    feature = "cm32m4"
)))]
pub use DefaultPorting as Porting;

use crate::task::Task;
use bare_metal::CriticalSection;

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
