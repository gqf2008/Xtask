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
pub const CPU_CLOCK_HZ: usize = 84_000_000;
#[cfg(feature = "stm32f4")]
pub const SYSTICK_CLOCK_HZ: usize = 84_000_000;

#[cfg(feature = "stm32f1")]
pub const CPU_CLOCK_HZ: usize = 8_000_000;
#[cfg(feature = "stm32f1")]
pub const SYSTICK_CLOCK_HZ: usize = 8_000_000;

#[cfg(feature = "stm32h7")]
pub const CPU_CLOCK_HZ: usize = 280_000_000;
#[cfg(feature = "stm32h7")]
pub const SYSTICK_CLOCK_HZ: usize = 24_000_000;

#[cfg(feature = "rp2040")]
pub const CPU_CLOCK_HZ: usize = 125_000_000;
#[cfg(feature = "rp2040")]
pub const SYSTICK_CLOCK_HZ: usize = 125_000_000; // SysTick clock_source=Core;原 1M 是错的(时间快 125 倍)

/// ⚠️ 时钟契约:SYSTICK_CLOCK_HZ 必须 == SysTick 的实际输入时钟
/// (ARM 口 clock_source(Core) 即 HCLK;示例 freeze() 的实际配置决定它——
/// 常数与示例时钟不一致 = tick/delay 整体漂移,f4/f1 口都栽过:原 f4 写
/// 180M/24M(f427 遗留)而示例默认 HSI 16M;原 f1 写 74M(72M 笔误)而
/// 示例默认 HSI 8M。现在:f4=84M(示例显式 sysclk(84M));f1=8M(默认 HSI)。
/// 改示例时钟时必须同步改这里。

/// 每秒产生多少次中断，没一次中断间隔就是任务能获得的时间片
pub const TICK_CLOCK_HZ: usize = 1000;

/// 软件定时器任务栈大小（单位：字长），默认1k字节栈空间
pub const TIMER_STACK_SIZE_WORD: usize = 1024;
