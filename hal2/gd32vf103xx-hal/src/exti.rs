//! External interrupt controller (EXTI).

use crate::pac::EXTI;

/// An `ExtiLine` that can be `listen()`ed for interrupt
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ExtiLine(u8);

/// Internal sources(lines) for ExtiLine
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum InternalLine {
    Lvd = 16,
    RtcAlarm = 17,
    UsbWakeup = 18,
}

/// Enable/Disable event genration to wakeup unit for an EXTI line
#[derive(Copy, Clone)]
pub enum ExtiEvent{
    Enable,
    Disable,
}

impl ExtiLine {
    /// Generate an `Option<ExtiLine>` from a GPIO pin configured by Afio
    ///
    /// A line from gpio can be obtained through its `pin_number()` method
    /// ```no_run
    /// extiline = ExtiLine::from_gpio_line(some_pin.pin_number()).unwrap();
    /// exti.listen(extiline, TriggerEdge::Falling);
    /// ```
    pub fn from_gpio_line(line: u8) -> Option<Self> {
        match line {
            0..=15 => Some(ExtiLine(line)),
            _ => None,
        }
    }

    /// Generate an `ExtiLine` from internal source specified by `InternalLine`
    pub fn from_internal_line(line: InternalLine) -> Self {
        ExtiLine(line as u8)
    }
}

/// Edges that can trigger a configurable interrupt line.
pub enum TriggerEdge {
    /// Trigger on rising edges only.
    Rising,
    /// Trigger on falling edges only.
    Falling,
    /// Trigger on both rising and falling edges.
    Both,
}

/// Higher-lever wrapper around the `EXTI` peripheral.
pub struct Exti {
    raw: EXTI,
}

impl Exti {
    /// Creates a new `Exti` wrapper from the raw `EXTI` peripheral.
    pub fn new(raw: EXTI) -> Self {
        Self { raw }
    }

    /// Destroys this `Exti` instance, returning the raw `EXTI` peripheral.
    pub fn release(self) -> EXTI {
        self.raw
    }

    /// Listen on one of the Lines
    #[inline]
    pub fn listen(&mut self, line: ExtiLine, edge: TriggerEdge) {
        let bm: u32 = 1 << line.0;

        unsafe {
            match edge {
                TriggerEdge::Falling => {
                    self.raw.ften.modify(|r, w| w.bits(r.bits() | bm));
                    self.raw.rten.modify(|r, w| w.bits(r.bits() & !bm));
                },
                TriggerEdge::Rising => {
                    self.raw.ften.modify(|r, w| w.bits(r.bits() & !bm));
                    self.raw.rten.modify(|r, w| w.bits(r.bits() | bm));
                },
                TriggerEdge::Both => {
                    self.raw.ften.modify(|r, w| w.bits(r.bits() | bm));
                    self.raw.rten.modify(|r, w| w.bits(r.bits() | bm));
                }
            }

            self.raw.inten.modify(|r, w| w.bits(r.bits() | bm));
        }
    }

    /// Unlisten on the specified line
    #[inline]
    pub fn unlisten(&mut self, line: ExtiLine) {
        let bm: u32 = 1 << line.0;

        unsafe {
            self.raw.rten.modify(|r, w| w.bits(r.bits() & !bm));
            self.raw.ften.modify(|r, w| w.bits(r.bits() & !bm));
            self.raw.inten.modify(|r, w| w.bits(r.bits() & !bm));
        }
    }

    /// Enable/Disable event generation on line to wakeup unit
    #[inline]
    pub fn gen_event(&mut self, line: ExtiLine, enable: ExtiEvent) {
        let bm: u32 = 1 << line.0;

        if let ExtiEvent::Enable = enable {
            unsafe { (*EXTI::ptr()).even.modify(|r, w| w.bits(r.bits() | bm)) };
        } else {
            unsafe { (*EXTI::ptr()).even.modify(|r, w| w.bits(r.bits() & !bm)) };
        }
    }

    /// `true` if this line has a pending interrupt
    #[inline]
    pub fn is_pending(line: ExtiLine) -> bool {
        let bm: u32 = 1 << line.0;
        unsafe { (*EXTI::ptr()).pd.read().bits() & bm != 0 }
    }

    /// Clear the pending interrupt flag
    #[inline]
    pub fn clear(line: ExtiLine) {
        let bm: u32 = 1 << line.0;
        unsafe { (*EXTI::ptr()).pd.write(|w| w.bits(bm)) };
    }

    /// Request a pending interrupt for this line from software
    #[inline]
    pub fn pend(line: ExtiLine) {
        let bm: u32 = 1 << line.0;
        unsafe { (*EXTI::ptr()).swiev.modify(|r, w| w.bits(r.bits() | bm)) };
    }

    /// Deactivate a software pending request for this line
    #[inline]
    pub fn unpend(line: ExtiLine) {
        let bm: u32 = 1 << line.0;
        unsafe { (*EXTI::ptr()).swiev.modify(|r, w| w.bits(r.bits() & !bm)) };
    }
}
