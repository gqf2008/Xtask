//! Reset and clock unit

use crate::pac::RCU;
use riscv::interrupt;
use crate::time::Hertz;
use core::cmp;


/// Extension trait that sets up the `RCU` peripheral
pub trait RcuExt {
    /// Configure the clocks of the `RCU` peripheral
    fn configure(self) -> UnconfiguredRcu;
}

impl RcuExt for RCU {
    fn configure(self) -> UnconfiguredRcu {
        UnconfiguredRcu::new(self)
    }
}

/// Configured RCU peripheral
pub struct Rcu {
    /// Frozen clock frequencies
    pub clocks: Clocks,
    pub(crate) regs: RCU,
}

pub struct UnconfiguredRcu {
    hxtal: Option<u32>,
    sysclk: Option<u32>,
    regs: RCU,
}

impl UnconfiguredRcu {
    fn new(rcu: RCU) -> Self {
        Self {
            hxtal: None,
            sysclk: None,
            regs: rcu,
        }
    }

    /// Uses an external oscillator instead of IRC8M (internal RC oscillator) as the high-speed
    /// clock source. Will result in a hang if an external oscillator is not connected or it fails
    /// to start.
    pub fn ext_hf_clock(mut self, freq: impl Into<Hertz>) -> Self {
        let freq = freq.into().0;
        assert!(4_000_000 <= freq && freq <= 32_000_000);

        self.hxtal = Some(freq);
        self
    }

    /// Sets the desired frequency for the SYSCLK clock
    pub fn sysclk(mut self, freq: impl Into<Hertz>) -> Self {
        let freq = freq.into().0;
        assert!(freq <= 108_000_000);

        self.sysclk = Some(freq);
        self
    }

    /// Freezes clock configuration, making it effective
    pub fn freeze(self) -> Rcu {
        const IRC8M: u32 = 8_000_000;

        let target_sysclk = self.sysclk.unwrap_or(IRC8M);

        let (scs_bits, use_pll) = match (self.hxtal, target_sysclk) {
            (Some(freq), sysclk) if freq == sysclk => (0b01, false),
            (None, sysclk) if IRC8M == sysclk => (0b00, false),
            _ => (0b10, true),
        };

        let pllsel_bit;
        let predv0_bits;
        let pllmf_bits;
        if use_pll {
            let pllmf;

            if let Some(hxtal_freq) = self.hxtal {
                // Use external clock + divider
                pllsel_bit = true;

                let calculate_pll = |source: u32, target: u32| -> Option<(u8, u8)> {
                    const PLL_IN_MIN: u32 = 600_000;
                    let div_max = cmp::min(16, source / PLL_IN_MIN);

                    for d in 1..=div_max {
                        let pllsource = source / d;
                        let pllm = target / pllsource;
                        if pllm < 2 || pllm == 15 || pllm > 32{
                            continue;
                        }
                        let actual_freq = pllsource * pllm;
                        if actual_freq == target {
                            return Some((d as u8, pllm as u8));
                        }
                    }
                    None
                };

                let (d, m) = calculate_pll(hxtal_freq, target_sysclk).expect("invalid sysclk value");
                predv0_bits = d - 1;
                pllmf = m;
            } else {
                // IRC8M/2 is used as an input clock
                pllsel_bit = false;

                let pllsource = IRC8M / 2;
                let m = target_sysclk / pllsource;
                let m = cmp::max(2, cmp::min(m, 32));
                assert_ne!(m, 15, "invalid sysclk value");
                let actual_sysclk = pllsource * m;
                assert_eq!(target_sysclk, actual_sysclk, "invalid sysclk value");

                predv0_bits = 0;
                pllmf = m as u8;
            }

            pllmf_bits = match pllmf {
                2..=14 => pllmf - 2,
                16..=32 => pllmf - 1,
                _ => unreachable!("invalid pll multiplier"),
            };
        } else {
            pllsel_bit = false;
            predv0_bits = 0;
            pllmf_bits = 0;
        }

        // Switch to the internal clock
        let rcu = unsafe { &*crate::pac::RCU::ptr() };
        rcu.ctl.modify(|_, w| w.irc8men().set_bit()); // Enable IRC8M oscillator
        while rcu.ctl.read().irc8mstb().bit_is_clear() {} // Wait for oscillator to stabilize
        rcu.cfg0.modify(|_, w| unsafe { w.scs().bits(0b00) }); // Switch to the internal oscillator
        rcu.ctl.modify(|_, w| w.pllen().clear_bit()); // Disable PLL

        // Set bus prescalers
        rcu.cfg0.modify(|_, w| unsafe { w.ahbpsc().bits(0b0000) }); // CK_SYS
        rcu.cfg0.modify(|_, w| unsafe { w.apb1psc().bits(0b100) }); // CK_AHB / 2
        rcu.cfg0.modify(|_, w| unsafe { w.apb2psc().bits(0b000) }); // CK_AHB
        let apb1_psc = 2;
        let apb2_psc = 1;

        if self.hxtal.is_some() {
            // Enable external oscillator
            rcu.ctl.modify(|_, w| w.hxtalen().set_bit());
            // Wait for oscillator to stabilize
            while rcu.ctl.read().hxtalstb().bit_is_clear() {}

            // Select HXTAL as prescaler input source clock
            rcu.cfg1.modify(|_, w| w.predv0sel().clear_bit());
            // Configure the prescaler
            rcu.cfg1.modify(|_, w| unsafe { w.predv0().bits(predv0_bits) });
        }

        if use_pll {
            // Configure PLL input selector
            rcu.cfg0.modify(|_, w| w.pllsel().bit(pllsel_bit));
            // Configure PLL multiplier
            rcu.cfg0.modify(|_, w| unsafe { w
                .pllmf_4().bit(pllmf_bits & 0x10 != 0)
                .pllmf_3_0().bits(pllmf_bits & 0xf)
            });
            // Enable PLL
            rcu.ctl.modify(|_, w| w.pllen().set_bit());
            // Wait for PLL to stabilize
            while rcu.ctl.read().pllstb().bit_is_clear() {}
        } else {
            // Disable PLL
            rcu.ctl.modify(|_, w| w.pllen().clear_bit());
        }

        // Switch to the configured clock source
        rcu.cfg0.modify(|_, w| unsafe { w.scs().bits(scs_bits) });

        let usbclk_valid;
        if use_pll {
            let pllclk = target_sysclk;
            let (valid, pr) = match pllclk {
                48_000_000 => (true, 0b01), // pllclk / 1
                72_000_000 => (true, 0b00), // pllclk / 1.5
                96_000_000 => (true, 0b11), // pllclk / 2
                _ => (false, 0),
            };
            usbclk_valid = valid;

            // Configure USB prescaler
            rcu.cfg0.modify(|_, w| unsafe { w.usbfspsc().bits(pr) });
        } else {
            usbclk_valid = false;
        }

        let clocks = Clocks {
            sysclk: Hertz(target_sysclk),
            apb1_psc,
            apb2_psc,
            usbclk_valid
        };

        Rcu {
            clocks,
            regs: self.regs
        }
    }
}

#[derive(Copy, Clone)]
pub struct Clocks {
    sysclk: Hertz,
    apb1_psc: u8,
    apb2_psc: u8,
    usbclk_valid: bool,
}

impl Clocks {
    /// Returns the system (core) frequency
    pub const fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns the frequency of the AHB
    pub const fn hclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns the frequency of the APB1
    pub const fn pclk1(&self) -> Hertz {
        Hertz(self.sysclk.0 / self.apb1_psc as u32)
    }

    /// Returns the frequency of the APB2
    pub const fn pclk2(&self) -> Hertz {
        Hertz(self.sysclk.0 / self.apb2_psc as u32)
    }

    /// Returns the frequency of the SysTick timer
    pub const fn systick(&self) -> Hertz {
        Hertz(self.sysclk.0 / 4)
    }

    /// Returns the frequency of the TIMER0 base clock
    pub fn timer0(&self) -> Hertz {
        let pclk2 = self.pclk2();
        if self.apb2_psc == 1 {
            pclk2
        } else {
            Hertz(pclk2.0 * 2)
        }
    }

    /// Returns the frequency of the TIMER1..6 base clock
    pub fn timerx(&self) -> Hertz {
        let pclk1 = self.pclk1();
        if self.apb1_psc == 1 {
            pclk1
        } else {
            Hertz(pclk1.0 * 2)
        }
    }

    /// Returns whether the USBCLK clock frequency is valid for the USB peripheral
    pub const fn usbclk_valid(&self) -> bool {
        self.usbclk_valid
    }
}

macro_rules! base_freq {
    ($($PER:ident => $func:ident,)+) => {
        $(
            impl BaseFrequency for crate::pac::$PER {
                #[inline(always)]
                fn base_frequency(rcu: &Rcu) -> Hertz {
                    rcu.clocks.$func()
                }
            }
        )+
    }
}

base_freq! {
    ADC0 => pclk2,
    ADC1 => pclk2,
    I2C0 => pclk1,
    I2C1 => pclk1,
    SPI0 => pclk2,
    SPI1 => pclk1,
    SPI2 => pclk1,
    TIMER0 => timer0,
    TIMER1 => timerx,
    TIMER2 => timerx,
    TIMER3 => timerx,
    TIMER4 => timerx,
    TIMER5 => timerx,
    TIMER6 => timerx,
    UART3 => pclk1,
    UART4 => pclk1,
    USART0 => pclk2,
    USART1 => pclk1,
    USART2 => pclk1,
}

pub(crate) mod closed_traits {
    use super::Rcu;
    use crate::time::Hertz;

    /// Enable/disable peripheral
    pub trait Enable {
        fn enable(rcu: &mut Rcu);
        fn disable(rcu: &mut Rcu);
    }

    /// Reset peripheral
    pub trait Reset {
        fn reset(rcu: &mut Rcu);
    }

    pub trait BaseFrequency {
        fn base_frequency(rcu: &Rcu) -> Hertz;
    }
}
pub(crate) use closed_traits::*;

macro_rules! bus_enable {
    ($PER:ident => ($apben:ident, $peren:ident)) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(rcu: &mut Rcu) {
                interrupt::free(|_| {
                    rcu.regs.$apben.modify(|_, w| w.$peren().set_bit());
                });
            }

            #[inline(always)]
            fn disable(rcu: &mut Rcu) {
                interrupt::free(|_| {
                    rcu.regs.$apben.modify(|_, w| w.$peren().clear_bit());
                });
            }
        }
    }
}

macro_rules! bus {
    ($($PER:ident => ($apben:ident, $apbrst:ident, $peren:ident, $perrst:ident),)+) => {
        $(
            bus_enable!($PER => ($apben, $peren));

            impl Reset for crate::pac::$PER {
                #[inline(always)]
                fn reset(rcu: &mut Rcu) {
                    interrupt::free(|_| {
                        rcu.regs.$apbrst.modify(|_, w| w.$perrst().set_bit());
                        rcu.regs.$apbrst.modify(|_, w| w.$perrst().clear_bit());
                    });
                }
            }
        )+
    }
}

bus! {
    ADC0 => (apb2en, apb2rst, adc0en, adc0rst),
    ADC1 => (apb2en, apb2rst, adc1en, adc1rst),
    AFIO => (apb2en, apb2rst, afen, afrst),
    BKP => (apb1en, apb1rst, bkpien, bkpirst),
    CAN0 => (apb1en, apb1rst, can0en, can0rst),
    CAN1 => (apb1en, apb1rst, can1en, can1rst),
    DAC => (apb1en, apb1rst, dacen, dacrst),
    GPIOA => (apb2en, apb2rst, paen, parst),
    GPIOB => (apb2en, apb2rst, pben, pbrst),
    GPIOC => (apb2en, apb2rst, pcen, pcrst),
    GPIOD => (apb2en, apb2rst, pden, pdrst),
    GPIOE => (apb2en, apb2rst, peen, perst),
    I2C0 => (apb1en, apb1rst, i2c0en, i2c0rst),
    I2C1 => (apb1en, apb1rst, i2c1en, i2c1rst),
    PMU => (apb1en, apb1rst, pmuen, pmurst),
    SPI0 => (apb2en, apb2rst, spi0en, spi0rst),
    SPI1 => (apb1en, apb1rst, spi1en, spi1rst),
    SPI2 => (apb1en, apb1rst, spi2en, spi2rst),
    TIMER0 => (apb2en, apb2rst, timer0en, timer0rst),
    TIMER1 => (apb1en, apb1rst, timer1en, timer1rst),
    TIMER2 => (apb1en, apb1rst, timer2en, timer2rst),
    TIMER3 => (apb1en, apb1rst, timer3en, timer3rst),
    TIMER4 => (apb1en, apb1rst, timer4en, timer4rst),
    TIMER5 => (apb1en, apb1rst, timer5en, timer5rst),
    TIMER6 => (apb1en, apb1rst, timer6en, timer6rst),
    UART3 => (apb1en, apb1rst, uart3en, uart3rst),
    UART4 => (apb1en, apb1rst, uart4en, uart4rst),
    USART0 => (apb2en, apb2rst, usart0en, usart0rst),
    USART1 => (apb1en, apb1rst, usart1en, usart1rst),
    USART2 => (apb1en, apb1rst, usart2en, usart2rst),
    USBFS_GLOBAL => (ahben, ahbrst, usbfsen, usbfsrst),
    WWDGT => (apb1en, apb1rst, wwdgten, wwdgtrst),
}
bus_enable!(CRC => (ahben, crcen));
bus_enable!(DMA0 => (ahben, dma0en));
bus_enable!(DMA1 => (ahben, dma1en));
bus_enable!(EXMC => (ahben, exmcen));
