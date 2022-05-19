//! General Purpose Input / Output

use core::marker::PhantomData;
use riscv::interrupt;
use crate::rcu::Rcu;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, rcu: &mut Rcu) -> Self::Parts;
}

#[repr(u8)]
pub enum Port {
    PAx = 0,
    PBx = 1,
    PCx = 2,
    PDx = 3,
    PEx = 4,
}

/// Marker trait for active states.
pub trait Active {}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}
impl<MODE> Active for Input<MODE> {}

/// Used by the debugger
///
/// To convert this pin into a normal one, call [Afio::disable_jtag()](crate::afio::Afio::disable_jtag)
pub struct Debugger;
/// Floating Input
pub struct Floating;
/// Pulled down Input
pub struct PullDown;
/// Pulled up Input
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}
impl<MODE> Active for Output<MODE> {}

/// Totem Pole aka Push-Pull
pub struct PushPull;
/// Open drain output
pub struct OpenDrain;

/// Analog mode
pub struct Analog;

/// Alternate function
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}
impl<MODE> Active for Alternate<MODE> {}

pub enum State {
    High,
    Low,
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum InputPortConfiguration {
    Analog = 0b0000,
    Floating = 0b0100,
    Pulled = 0b1000,
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum OutputPortConfiguration {
    GpioPushPull = 0b0000,
    GpioOpenDrain = 0b0100,
    AfioPushPull = 0b1000,
    AfioOpenDrain = 0b1100,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum PortMode {
    Input(InputPortConfiguration),
    Output10Mhz(OutputPortConfiguration),
    Output2Mhz(OutputPortConfiguration),
    Output50Mhz(OutputPortConfiguration),
}

impl PortMode {
    #[inline(always)]
    pub fn into_bits(self) -> u8 {
        match self {
            PortMode::Input(conf)       => (conf as u8) | 0b00,
            PortMode::Output10Mhz(conf) => (conf as u8) | 0b01,
            PortMode::Output2Mhz(conf)  => (conf as u8) | 0b10,
            PortMode::Output50Mhz(conf) => (conf as u8) | 0b11,
        }
    }
}

trait PeripheralAccess {
    fn peripheral() -> &'static crate::pac::gpioa::RegisterBlock;

    fn set_mode(index: u8, mode: PortMode) {
        assert!(index < 16);

        let bits = mode.into_bits();
        let offset = (index * 4) % 32;
        let mask = !(0b1111u32 << offset);
        let value = (bits as u32) << offset;
        let regs = Self::peripheral();

        interrupt::free(|_| {
            if index < 8 {
                regs.ctl0.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | value)
                });
            } else {
                regs.ctl1.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | value)
                });
            }
        });
    }

    #[inline(always)]
    fn set_bit(index: u8) {
        assert!(index < 16);

        let regs = Self::peripheral();

        // NOTE(unsafe) atomic write to a stateless register
        regs.bop.write(|w| unsafe { w.bits(1u32 << index) });
    }

    #[inline(always)]
    fn clear_bit(index: u8) {
        assert!(index < 16);

        let regs = Self::peripheral();

        // NOTE(unsafe) atomic write to a stateless register
        regs.bop.write(|w| unsafe { w.bits(1u32 << (16 + index)) });
    }

    #[inline(always)]
    fn is_high(index: u8) -> bool {
        assert!(index < 16);

        let regs = Self::peripheral();

        let mask = 1u32 << index;
        regs.istat.read().bits() & mask != 0
    }

    #[inline(always)]
    fn is_set_high(index: u8) -> bool {
        assert!(index < 16);

        let regs = Self::peripheral();

        let mask = 1u32 << index;
        regs.octl.read().bits() & mask != 0
    }
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $gpioy:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::convert::Infallible;
            use core::marker::PhantomData;
            use crate::hal::digital::v2::{OutputPin, InputPin, StatefulOutputPin, toggleable};
            use crate::pac::$GPIOX;
            use crate::rcu::{Rcu, Enable, Reset};
            use super::{
                PeripheralAccess,
                PortMode,
                InputPortConfiguration,
                OutputPortConfiguration,
                Alternate, Floating, GpioExt, Input,
                OpenDrain,
                Output,
                PullDown,
                PullUp,
                PushPull,
                Analog,
                State,
                Active,
                Debugger,
                Pxx,
                Port
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl PeripheralAccess for $GPIOX {
                #[inline(always)]
                fn peripheral() -> &'static crate::pac::gpioa::RegisterBlock {
                    unsafe { &*$GPIOX::ptr() }
                }
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self, rcu: &mut Rcu) -> Parts {
                    $GPIOX::enable(rcu);
                    $GPIOX::reset(rcu);

                    Parts {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Partially erased pin. Only used in the Pxx enum
            pub struct Generic<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> Generic<MODE> {
                pub fn downgrade(self) -> Pxx<MODE> {
                    Pxx::$PXx(self)
                }

                pub fn pin_number(&self) -> u8 {
                    self.i
                }
            }

            impl<MODE> OutputPin for Generic<Output<MODE>> {
                type Error = Infallible;
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    $GPIOX::set_bit(self.i);
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    $GPIOX::clear_bit(self.i);
                    Ok(())
                }
            }

            impl<MODE> InputPin for Generic<Input<MODE>> {
                type Error = Infallible;
                fn is_high(&self) -> Result<bool, Self::Error> {
                    Ok($GPIOX::is_high(self.i))
                }

                fn is_low(&self) -> Result<bool, Self::Error> {
                    Ok(!$GPIOX::is_high(self.i))
                }
            }


            impl<MODE> StatefulOutputPin for Generic<Output<MODE>> {
                fn is_set_high(&self) -> Result<bool, Self::Error> {
                    Ok($GPIOX::is_set_high(self.i))
                }

                fn is_set_low(&self) -> Result<bool, Self::Error> {
                    Ok(!$GPIOX::is_set_high(self.i))
                }
            }

            impl<MODE> toggleable::Default for Generic<Output<MODE>> {}

            impl InputPin for Generic<Output<OpenDrain>> {
                type Error = Infallible;
                fn is_high(&self) -> Result<bool, Self::Error> {
                    Ok($GPIOX::is_high(self.i))
                }

                fn is_low(&self) -> Result<bool, Self::Error> {
                    Ok(!$GPIOX::is_high(self.i))
                }
            }

            pub type $PXx<MODE> = Pxx<MODE>;



            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    pub const fn port(&self) -> Port {
                        Port::$PXx
                    }

                    pub const fn pin_number(&self) -> u8 {
                        $i
                    }
                }

                impl $PXi<Debugger> {
                    /// Put the pin in an active state. The caller
                    /// must enforce that the pin is really in this
                    /// state in the hardware.
                    #[allow(dead_code)]
                    pub(crate) unsafe fn activate(self) -> $PXi<Input<Floating>> {
                        // JTAG/Serial-Wired Debug pins are in input PU/PD mode after reset.
                        // Explicitly convert into floating inputs.
                        let mode = PortMode::Input(InputPortConfiguration::Floating);
                        $GPIOX::set_mode($i, mode);

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> $PXi<MODE> where MODE: Active {
                    /// Configures the pin to operate as an alternate function push-pull output
                    /// pin.
                    pub fn into_alternate_push_pull(self) -> $PXi<Alternate<PushPull>> {
                        let mode = PortMode::Output50Mhz(OutputPortConfiguration::AfioPushPull);
                        $GPIOX::set_mode($i, mode);

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an alternate function open-drain output
                    /// pin.
                    pub fn into_alternate_open_drain(self) -> $PXi<Alternate<OpenDrain>> {
                        let mode = PortMode::Output50Mhz(OutputPortConfiguration::AfioOpenDrain);
                        $GPIOX::set_mode($i, mode);

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(self) -> $PXi<Input<Floating>> {
                        let mode = PortMode::Input(InputPortConfiguration::Floating);
                        $GPIOX::set_mode($i, mode);

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(self) -> $PXi<Input<PullDown>> {
                        $GPIOX::clear_bit($i); // pull down

                        let mode = PortMode::Input(InputPortConfiguration::Pulled);
                        $GPIOX::set_mode($i, mode);

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(self) -> $PXi<Input<PullUp>> {
                        $GPIOX::set_bit($i); // pull up

                        let mode = PortMode::Input(InputPortConfiguration::Pulled);
                        $GPIOX::set_mode($i, mode);

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open-drain output pin.
                    /// Initial state will be low.
                    pub fn into_open_drain_output(self) -> $PXi<Output<OpenDrain>> {
                        self.into_open_drain_output_with_state(State::Low)
                    }

                    /// Configures the pin to operate as an open-drain output pin.
                    /// `initial_state` specifies whether the pin should be initially high or low.
                    pub fn into_open_drain_output_with_state(
                        self,
                        initial_state: State,
                    ) -> $PXi<Output<OpenDrain>> {
                        match initial_state {
                            State::High => $GPIOX::set_bit($i),
                            State::Low  => $GPIOX::clear_bit($i),
                        }

                        let mode = PortMode::Output50Mhz(OutputPortConfiguration::GpioOpenDrain);
                        $GPIOX::set_mode($i, mode);

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push-pull output pin.
                    /// Initial state will be low.
                    pub fn into_push_pull_output(
                        self
                    ) -> $PXi<Output<PushPull>> {
                        self.into_push_pull_output_with_state(State::Low)
                    }

                    /// Configures the pin to operate as an push-pull output pin.
                    /// `initial_state` specifies whether the pin should be initially high or low.
                    pub fn into_push_pull_output_with_state(
                        self,
                        initial_state: State,
                    ) -> $PXi<Output<PushPull>> {
                        match initial_state {
                            State::High => $GPIOX::set_bit($i),
                            State::Low  => $GPIOX::clear_bit($i),
                        }

                        let mode = PortMode::Output50Mhz(OutputPortConfiguration::GpioPushPull);
                        $GPIOX::set_mode($i, mode);

                        $PXi { _mode: PhantomData }
                    }


                    /// Configures the pin to operate as an analog input pin
                    pub fn into_analog(self) -> $PXi<Analog> {
                        let mode = PortMode::Input(InputPortConfiguration::Analog);
                        $GPIOX::set_mode($i, mode);

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> $PXi<MODE> where MODE: Active {
                    /// Erases the pin number from the type
                    fn into_generic(self) -> Generic<MODE> {
                        Generic {
                            i: $i,
                            _mode: self._mode,
                        }
                    }

                    /// Erases the pin number and port from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> Pxx<MODE> {
                        self.into_generic().downgrade()
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    type Error = Infallible;
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        $GPIOX::set_bit($i);
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        $GPIOX::clear_bit($i);
                        Ok(())
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        Ok($GPIOX::is_set_high($i))
                    }

                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        Ok(!$GPIOX::is_set_high($i))
                    }
                }

                impl<MODE> toggleable::Default for $PXi<Output<MODE>> {}

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    type Error = Infallible;
                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok($GPIOX::is_high($i))
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        Ok(!$GPIOX::is_high($i))
                    }
                }

                impl InputPin for $PXi<Output<OpenDrain>> {
                    type Error = Infallible;
                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok($GPIOX::is_high($i))
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        Ok(!$GPIOX::is_high($i))
                    }
                }
            )+
        }
    }
}

macro_rules! impl_pxx {
    ($(($port:ident :: $pin:ident)),*) => {
        use core::convert::Infallible;
        use embedded_hal::digital::v2::{InputPin, StatefulOutputPin, OutputPin};

        pub enum Pxx<MODE> {
            $(
                $pin($port::Generic<MODE>)
            ),*
        }

        impl<MODE> Pxx<MODE> {
            pub fn pin_number(&self) -> u8 {
                match self {
                    $(Pxx::$pin(pin) => pin.pin_number()),*
                }
            }
        }

        impl<MODE> OutputPin for Pxx<Output<MODE>> {
            type Error = Infallible;
            fn set_high(&mut self) -> Result<(), Infallible> {
                match self {
                    $(Pxx::$pin(pin) => pin.set_high()),*
                }
            }

            fn set_low(&mut self) -> Result<(), Infallible> {
                match self {
                    $(Pxx::$pin(pin) => pin.set_low()),*
                }
            }
        }

        impl<MODE> StatefulOutputPin for Pxx<Output<MODE>> {
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                match self {
                    $(Pxx::$pin(pin) => pin.is_set_high()),*
                }
            }

            fn is_set_low(&self) -> Result<bool, Self::Error> {
                match self {
                    $(Pxx::$pin(pin) => pin.is_set_low()),*
                }
            }
        }

        impl<MODE> InputPin for Pxx<Input<MODE>> {
            type Error = Infallible;
            fn is_high(&self) -> Result<bool, Infallible> {
                match self {
                    $(Pxx::$pin(pin) => pin.is_high()),*
                }
            }

            fn is_low(&self) -> Result<bool, Infallible> {
                match self {
                    $(Pxx::$pin(pin) => pin.is_low()),*
                }
            }
        }
    }
}

impl_pxx! {
    (gpioa::PAx),
    (gpiob::PBx),
    (gpioc::PCx),
    (gpiod::PDx),
    (gpioe::PEx)
}

gpio!(GPIOA, gpioa, gpioa, PAx, [
    PA0: (pa0, 0, Input<Floating>),
    PA1: (pa1, 1, Input<Floating>),
    PA2: (pa2, 2, Input<Floating>),
    PA3: (pa3, 3, Input<Floating>),
    PA4: (pa4, 4, Input<Floating>),
    PA5: (pa5, 5, Input<Floating>),
    PA6: (pa6, 6, Input<Floating>),
    PA7: (pa7, 7, Input<Floating>),
    PA8: (pa8, 8, Input<Floating>),
    PA9: (pa9, 9, Input<Floating>),
    PA10: (pa10, 10, Input<Floating>),
    PA11: (pa11, 11, Input<Floating>),
    PA12: (pa12, 12, Input<Floating>),
    PA13: (pa13, 13, Debugger),
    PA14: (pa14, 14, Debugger),
    PA15: (pa15, 15, Debugger),
]);

gpio!(GPIOB, gpiob, gpioa, PBx, [
    PB0: (pb0, 0, Input<Floating>),
    PB1: (pb1, 1, Input<Floating>),
    PB2: (pb2, 2, Input<Floating>),
    PB3: (pb3, 3, Debugger),
    PB4: (pb4, 4, Debugger),
    PB5: (pb5, 5, Input<Floating>),
    PB6: (pb6, 6, Input<Floating>),
    PB7: (pb7, 7, Input<Floating>),
    PB8: (pb8, 8, Input<Floating>),
    PB9: (pb9, 9, Input<Floating>),
    PB10: (pb10, 10, Input<Floating>),
    PB11: (pb11, 11, Input<Floating>),
    PB12: (pb12, 12, Input<Floating>),
    PB13: (pb13, 13, Input<Floating>),
    PB14: (pb14, 14, Input<Floating>),
    PB15: (pb15, 15, Input<Floating>),
]);

gpio!(GPIOC, gpioc, gpioa, PCx, [
    PC0: (pc0, 0, Input<Floating>),
    PC1: (pc1, 1, Input<Floating>),
    PC2: (pc2, 2, Input<Floating>),
    PC3: (pc3, 3, Input<Floating>),
    PC4: (pc4, 4, Input<Floating>),
    PC5: (pc5, 5, Input<Floating>),
    PC6: (pc6, 6, Input<Floating>),
    PC7: (pc7, 7, Input<Floating>),
    PC8: (pc8, 8, Input<Floating>),
    PC9: (pc9, 9, Input<Floating>),
    PC10: (pc10, 10, Input<Floating>),
    PC11: (pc11, 11, Input<Floating>),
    PC12: (pc12, 12, Input<Floating>),
    PC13: (pc13, 13, Input<Floating>),
    PC14: (pc14, 14, Input<Floating>),
    PC15: (pc15, 15, Input<Floating>),
]);

gpio!(GPIOD, gpiod, gpioa, PDx, [
    PD0: (pd0, 0, Input<Floating>),
    PD1: (pd1, 1, Input<Floating>),
    PD2: (pd2, 2, Input<Floating>),
    PD3: (pd3, 3, Input<Floating>),
    PD4: (pd4, 4, Input<Floating>),
    PD5: (pd5, 5, Input<Floating>),
    PD6: (pd6, 6, Input<Floating>),
    PD7: (pd7, 7, Input<Floating>),
    PD8: (pd8, 8, Input<Floating>),
    PD9: (pd9, 9, Input<Floating>),
    PD10: (pd10, 10, Input<Floating>),
    PD11: (pd11, 11, Input<Floating>),
    PD12: (pd12, 12, Input<Floating>),
    PD13: (pd13, 13, Input<Floating>),
    PD14: (pd14, 14, Input<Floating>),
    PD15: (pd15, 15, Input<Floating>),
]);

gpio!(GPIOE, gpioe, gpioa, PEx, [
    PE0: (pe0, 0, Input<Floating>),
    PE1: (pe1, 1, Input<Floating>),
    PE2: (pe2, 2, Input<Floating>),
    PE3: (pe3, 3, Input<Floating>),
    PE4: (pe4, 4, Input<Floating>),
    PE5: (pe5, 5, Input<Floating>),
    PE6: (pe6, 6, Input<Floating>),
    PE7: (pe7, 7, Input<Floating>),
    PE8: (pe8, 8, Input<Floating>),
    PE9: (pe9, 9, Input<Floating>),
    PE10: (pe10, 10, Input<Floating>),
    PE11: (pe11, 11, Input<Floating>),
    PE12: (pe12, 12, Input<Floating>),
    PE13: (pe13, 13, Input<Floating>),
    PE14: (pe14, 14, Input<Floating>),
    PE15: (pe15, 15, Input<Floating>),
]);
