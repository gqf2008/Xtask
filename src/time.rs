//! 时间类函数

use crate::chip::TICK_CLOCK_HZ;
use crate::port::{Portable, Porting};
use crate::timer;
/// 启动到现在总的systick数
static mut TICKS: u64 = 0;

/// 每TICK多少毫秒
const TICK_PREIOD_MS: usize = 1000 / TICK_CLOCK_HZ;

/// 每毫秒多少TICK
const MS_PREIOD_TICK: f32 = TICK_CLOCK_HZ as f32 / 1000.0;

#[inline]
pub(crate) unsafe fn increase_tick() {
    let tick = core::ptr::read_volatile(&TICKS);
    core::ptr::write_volatile(&mut TICKS, tick + 1);
    #[cfg(feature = "timer")]
    timer::do_tick(tick + 1);
}

/// 返回任务Tick
#[inline]
pub fn tick() -> u64 {
    unsafe { core::ptr::read_volatile(&TICKS) }
}

/// 毫秒转tick
#[inline(always)]
pub fn ms2ticks(ms: usize) -> usize {
    if ms < TICK_PREIOD_MS {
        return 0;
    }
    (ms as f32 * MS_PREIOD_TICK) as usize
}

/// 返回tick时长，单位毫秒
#[inline]
pub fn tick_ms() -> u64 {
    tick() * TICK_PREIOD_MS as u64
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
