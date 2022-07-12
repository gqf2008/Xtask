/// 移植环境配置参数

/// 定时控制器基地址
#[cfg(feature = "gd32vf103")]
pub const TIMER_CTRL_ADDR: usize = 0xD100_0000;
/// 中断控制器基地址
#[cfg(feature = "gd32vf103")]
pub const ECLIC_CTRL_ADDR: usize = 0xD200_0000;
/// CPU时钟频率
#[cfg(feature = "gd32vf103")]
pub const CPU_CLOCK_HZ: usize = 108_000_000;
/// SYSTICK时钟频率
#[cfg(feature = "gd32vf103")]
pub const SYSTICK_CLOCK_HZ: usize = 108_000_000 / 4;

/// 定时控制器基地址
#[cfg(feature = "cm32m4")]
pub const TIMER_CTRL_ADDR: usize = 0xE002_0000;
/// 中断控制器基地址
#[cfg(feature = "cm32m4")]
pub const ECLIC_CTRL_ADDR: usize = 0xE001_0000;
/// CPU时钟频率
#[cfg(feature = "cm32m4")]
pub const CPU_CLOCK_HZ: usize = 144000000;
/// SYSTICK时钟频率
#[cfg(feature = "cm32m4")]
pub const SYSTICK_CLOCK_HZ: usize = 144000000 / 4;

/// CPU时钟频率
#[cfg(feature = "stm32f4")]
pub const CPU_CLOCK_HZ: usize = 100_000_000; //stm32f401: 84_000_000;stm32f427: 180_000_000;
#[cfg(feature = "stm32f4")]
pub const SYSTICK_CLOCK_HZ: usize = 24_000_000;

#[cfg(feature = "stm32f1")]
pub const CPU_CLOCK_HZ: usize = 74000000;
#[cfg(feature = "stm32f1")]
pub const SYSTICK_CLOCK_HZ: usize = 8_000_000;

#[cfg(feature = "stm32h7")]
pub const CPU_CLOCK_HZ: usize = 280_000_000;
#[cfg(feature = "stm32h7")]
pub const SYSTICK_CLOCK_HZ: usize = 24_000_000;

#[cfg(feature = "rp2040")]
pub const CPU_CLOCK_HZ: usize = 125_000_000;
#[cfg(feature = "rp2040")]
pub const SYSTICK_CLOCK_HZ: usize = 1_000_000;

/// 每秒产生多少次中断，没一次中断间隔就是任务能获得的时间片
pub const TICK_CLOCK_HZ: usize = 1000;

/// 软件定时器任务栈大小（单位：字长），默认1k字节栈空间
pub const TIMER_STACK_SIZE_WORD: usize = 1024;
