//! HAL for the GD32VF103xx family
//!
//! This is an implementation of the [`embedded-hal`] traits for the GD32VF103xx family

//#![deny(missing_docs)]
#![no_std]

pub use gd32vf103_pac as pac;

use embedded_hal as hal;

pub mod adc;
pub mod afio;
pub mod backup_domain;
pub mod delay;
pub mod dma;
pub mod eclic;
pub mod exmc;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod prelude;
pub mod pwm;
pub mod rcu;
pub mod rtc;
pub mod serial;
pub mod signature;
pub mod spi;
pub mod time;
pub mod timer;
pub mod watchdog;
