//! 时间类函数

use crate::chip::TICK_CLOCK_HZ;
use crate::port::{Portable, Porting};
#[cfg(feature = "timer")]
use crate::timer;
use cast::u64;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
/// 启动到现在总的systick数
static mut TICKS: u64 = 0;

/// 每TICK多少微秒
const TICK_PREIOD_US: usize = 1_000_000 / TICK_CLOCK_HZ;

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
    ms * 1000 / TICK_PREIOD_US
}

/// 返回tick时长，单位毫秒
#[inline]
pub fn tick_ms() -> u64 {
    tick_us() / 1000
}

/// 返回tick时长，单位微秒
#[inline]
pub fn tick_us() -> u64 {
    tick() * TICK_PREIOD_US as u64
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
