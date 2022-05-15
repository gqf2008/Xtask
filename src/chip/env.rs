/// 移植环境配置参数

/// 定时控制器基地址
#[cfg(feature = "gd32vf103")]
pub const TIMER_CTRL_ADDR: usize = 0xd100_0000;

/// CPU时钟频率
pub const CPU_CLOCK_HZ: usize = 108000000;
/// RTC时钟频率
pub const RTC_CLOCK_HZ: usize = 108000000 / 4;
/// 每秒产生多少次中断，没一次中断间隔就是任务能获得的时间片
pub const TICK_CLOCK_HZ: usize = 1000;
/// 是否启用软件定时器
pub const TIMER_TASK_ENABLE: bool = false;
/// 是否启用调试任务
pub const DEBUG_TASK_ENABLE: bool = true;
