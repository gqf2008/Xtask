/// 移植环境配置参数

/// 定时控制器基地址
#[cfg(feature = "gd32vf103")]
pub const TIMER_CTRL_ADDR: usize = 0xd100_0000;

/// CPU时钟频率
pub const CPU_CLOCK_HZ: usize = 84000000;
/// SYSTICK时钟频率
pub const SYSTICK_CLOCK_HZ: usize = CPU_CLOCK_HZ;
/// 每秒产生多少次中断，没一次中断间隔就是任务能获得的时间片
pub const TICK_CLOCK_HZ: usize = 1000;

// /// CPU时钟频率
// pub const CPU_CLOCK_HZ: usize = 108000000;
// /// RTC时钟频率
// pub const RTC_CLOCK_HZ: usize = 108000000 / 4;
// /// 每秒产生多少次中断，没一次中断间隔就是任务能获得的时间片
// pub const TICK_CLOCK_HZ: usize = 1000;
/// 软件定时器任务栈大小（单位：字长），默认1k字节栈空间
pub const TIMER_STACK_SIZE_WORD: usize = 256;
