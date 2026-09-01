//! 时间类函数

use crate::chip::TICK_CLOCK_HZ;
use crate::port::{Portable, Porting};
#[cfg(feature = "timer")]
use crate::timer;
use cast::u64;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use vcell::VolatileCell;

/// 启动到现在总的systick数
static mut TICKS: VolatileCell<u64> = VolatileCell::new(0);

/// 每TICK多少微秒
const TICK_PERIOD_US: usize = 1_000_000 / TICK_CLOCK_HZ;

/// 节拍推进(tick ISR 侧,主核独占调用):TICKS 直接 +delta 并驱动
/// 软定时器堆。逐拍路径 delta=1(恒定节拍),tickless 一次性到点
/// delta=el(实测跳账,`jump_ticks`)——两者是同一操作的批量/逐拍两档,
/// 等价性即"绝对时刻账本对逐拍/跳账等价"(第 28 章)。
/// 读写同锁:SMP 下读侧(tick())在临界区内读,写侧也必须持同一把锁——
/// 否则 RV32 上 u64 两次 32 位读写可撕裂,别核读者会读到高低位错代的值
#[inline]
pub(crate) unsafe fn advance_ticks(delta: u64) {
    debug_assert!(delta > 0);
    let tick = crate::sync::free(|_| {
        let tick = TICKS.get() + delta;
        TICKS.set(tick);
        tick
    });
    #[cfg(feature = "timer")]
    timer::do_tick(tick);
}

/// 节拍自增(恒定节拍每拍一次)
#[inline]
pub(crate) unsafe fn increase_tick() {
    advance_ticks(1)
}

/// 节拍快进(tickless 一次性定时到点,tick ISR 侧):TICKS 直接 +delta
/// 并驱动软定时器堆——与 increase_tick 的逐拍路径等价,只是批量。
/// tick() 是运行时时钟,内核睡了 delta 拍就补 delta 拍账,不损失任何账目;
/// 期间到期/新建的延时与软定时器一律按绝对拍比较,跳账后天然归位
#[inline]
pub(crate) unsafe fn jump_ticks(delta: u64) {
    advance_ticks(delta)
}

/// 返回任务Tick
/// 32 位目标上 u64 读非原子，进临界区防止与 ISR 里的 increase_tick 并发撕裂
#[inline]
pub fn tick() -> u64 {
    crate::sync::free(|_| unsafe { TICKS.get() })
}

/// 毫秒转tick
#[inline(always)]
pub fn ms2ticks(ms: usize) -> usize {
    ms * 1000 / TICK_PERIOD_US
}

/// 返回tick时长，单位毫秒
#[inline]
pub fn tick_ms() -> u64 {
    tick_us() / 1000
}

/// 返回tick时长，单位微秒
#[inline]
pub fn tick_us() -> u64 {
    tick() * TICK_PERIOD_US as u64
}

/// 返回rtc tick
#[inline]
pub fn systick() -> u64 {
    Porting::systick()
}

/// 瞬时对象
/// 用于需要精确计时/测量的场景
#[derive(Clone, Copy)]
pub struct Instant {
    now: u64,
}

impl Instant {
    pub fn now() -> Self {
        Self { now: systick() }
    }
    pub fn elapsed(self) -> u64 {
        systick().wrapping_sub(self.now)
    }
}

pub struct Delay;

impl Delay {
    pub const fn new() -> Self {
        Self
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        Porting::delay_us(u64(ms) * 1000)
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        Porting::delay_us(u64(ms) * 1000)
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        Porting::delay_us(u64(ms) * 1000)
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        Porting::delay_us(u64(us))
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        Porting::delay_us(u64(us))
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        Porting::delay_us(u64(us))
    }
}

#[cfg(test)]
mod tests {
    use super::{jump_ticks, tick, TICKS};

    /// 跳账:直接 +delta——与逐拍 +1 的逐拍路径等价(第 28 章 tickless
    /// 到点补账)。唯一触碰 TICKS 全局的测试:其余 host 测试不读 tick 值,
    /// 无共享状态;末行还原现场
    #[test]
    fn jump_ticks_advances_exactly() {
        unsafe {
            let t0 = TICKS.get();
            jump_ticks(3);
            assert_eq!(tick(), t0 + 3, "跳 3 拍应到 t0+3");
            jump_ticks(5);
            assert_eq!(tick(), t0 + 8, "再跳 5 拍应到 t0+8");
            // 大跳:读写都在锁内,32 位目标上 u64 不撕裂
            jump_ticks(1u64 << 33);
            assert_eq!(tick(), t0 + 8 + (1u64 << 33));
            // 还原现场,避免影响其他测试
            TICKS.set(t0);
        }
    }
}
