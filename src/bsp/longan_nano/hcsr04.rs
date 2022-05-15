//! 超声波测距传感器

use crate::kalman::{LinearKalmanFilter, ScalarKalmanFilter};
use alloc::vec;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use nalgebra::{dmatrix, dvector};

#[derive(Debug, Copy, Clone)]
pub struct Distance(f32);

impl Distance {
    pub fn cm(&self) -> f32 {
        self.0 / 10.0
    }
    pub fn mm(&self) -> f32 {
        self.0
    }
}

pub struct HcSr04<Triger, Echo, Delay> {
    trig: Triger,
    echo: Echo,
    delay: Delay,
    timer: MonoTimer,
    filter: ScalarKalmanFilter,
    kfilter: LinearKalmanFilter,
}

impl<Trig, Echo, Delay> HcSr04<Trig, Echo, Delay>
where
    Trig: OutputPin,
    Echo: InputPin,
    Delay: DelayUs<u32>,
{
    pub fn new(mut trig: Trig, echo: Echo, delay: Delay, core_frequency: u64) -> Self {
        trig.set_low().ok();
        let q = dmatrix![
            1.0, 0.1;
            0.1, 1.0];
        // Measurement noise matrix
        let r = dmatrix![
        1.0, 0.2, 0.1;
        0.2, 0.8, 0.5;
        0.1, 0.5, 1.2];
        // Observation matrix
        let h = dmatrix![
        1.0, 0.7;
        0.5, 0.7;
        0.8, 0.1];
        // State transition matrix
        let f = dmatrix![
        0.6, 0.2;
        0.1, 0.3];
        // Initial guess for state mean at time 1
        let x0 = dvector![1.0, 1.0];
        // Initial guess for state covariance at time 1
        let p0 = dmatrix![
        1.0, 0.0;
        0.0, 1.0];
        let kfilter = LinearKalmanFilter::new(q, r, h, f, x0, p0);
        HcSr04 {
            trig,
            echo,
            delay,
            timer: MonoTimer {
                frequency: core_frequency,
            },
            filter: ScalarKalmanFilter::new(1.0, 1.0, 1.0, 1.0),
            kfilter,
        }
    }

    pub fn measure(&mut self) -> Distance {
        // self.delay.delay_us(100000u32);
        let mut d = 0.0;
        for _ in 0..5 {
            let tmp = self.measure1();
            d = self.filter.filter(tmp);
            self.delay.delay_us(10000u32);
        }
        Distance(d)
    }

    fn measure1(&mut self) -> f32 {
        //发送信号
        self.trig.set_high().ok();
        self.delay.delay_us(20u32);
        self.trig.set_low().ok();
        //let start_wait = self.timer.now();
        //等高电平
        while let Ok(true) = self.echo.is_low() {
            // if start_wait.elapsed() > self.timer.frequency() * 5 {
            //     return Err(Error::Timeout);
            // }
        }
        //等低电平（高电平持续的时间就是信号往返的时间）
        let start_instant = self.timer.now();
        //crate::sprintln!("start {}", start_instant.now);
        while let Ok(true) = self.echo.is_high() {
            // if start_instant.elapsed() > self.timer.frequency().0 {
            //     return Err(Error::Timeout);
            // }
        }
        let ticks = start_instant.elapsed() as f32;
        // crate::sprintln!("elapsed {}", ticks);
        ticks / self.timer.frequency() as f32 * 170.0 * 1000.0
    }
}

#[derive(Clone, Copy)]
pub struct MonoTimer {
    frequency: u64,
}

impl MonoTimer {
    /// Creates a new `Monotonic` timer
    pub fn new(frequency: u64) -> Self {
        MonoTimer { frequency }
    }

    /// Returns the frequency at which the monotonic timer is operating at
    pub fn frequency(self) -> u64 {
        self.frequency
    }

    /// Returns an `Instant` corresponding to "now"
    pub fn now(self) -> Instant {
        Instant {
            now: riscv::register::mcycle::read64(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Instant {
    now: u64,
}

impl Instant {
    pub fn elapsed(self) -> u64 {
        riscv::register::mcycle::read64().wrapping_sub(self.now)
    }
}
