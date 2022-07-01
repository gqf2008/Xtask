use crate::pac::ECLIC;
use riscv::interrupt::Nr;

const EFFECTIVE_LEVEL_PRIORITY_BITS: u8 = 4;

#[repr(u8)]
#[derive(Debug)]
pub enum LevelPriorityBits {
    L0P4 = 0,
    L1P3 = 1,
    L2P2 = 2,
    L3P1 = 3,
    L4P0 = 4,
}

#[repr(u8)]
#[derive(Debug)]
pub enum TriggerType {
    Level = 0,
    RisingEdge = 1,
    FallingEdge = 3,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Level {
    L0 = 0,
    L1 = 1,
    L2 = 2,
    L3 = 3,
    L4 = 4,
    L5 = 5,
    L6 = 6,
    L7 = 7,
    L8 = 8,
    L9 = 9,
    L10 = 10,
    L11 = 11,
    L12 = 12,
    L13 = 13,
    L14 = 14,
    L15 = 15,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Priority {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
    P8 = 8,
    P9 = 9,
    P10 = 10,
    P11 = 11,
    P12 = 12,
    P13 = 13,
    P14 = 14,
    P15 = 15,
}

pub trait EclicExt {
    /// Reset all ECLIC registers to 0
    fn reset();

    /// Set interrupts threshold level
    fn set_threshold_level(level: Level);

    /// Get interrupts threshold level
    fn get_threshold_level() -> Level;

    fn set_level_priority_bits(lp: LevelPriorityBits);

    fn get_level_priority_bits() -> Option<LevelPriorityBits>;

    /// Get number of bits designated for interrupt level
    fn get_level_bits() -> u8;

    /// Get number of bits designated for interrupt priority
    fn get_priority_bits() -> u8;

    /// Setup `interrupt`
    fn setup<I: Nr + Copy>(interrupt: I, tt: TriggerType, level: Level, priority: Priority);

    /// Enables `interrupt`
    unsafe fn unmask<I: Nr>(interrupt: I);

    /// Disables `interrupt`
    fn mask<I: Nr>(interrupt: I);

    /// Checks if `interrupt` is enabled
    fn is_enabled<I: Nr>(interrupt: I) -> bool;

    /// Forces `interrupt` into pending state
    fn pend<I: Nr>(interrupt: I);

    /// Clears `interrupt`'s pending state
    fn unpend<I: Nr>(interrupt: I);

    /// Checks if `interrupt` is pending
    fn is_pending<I: Nr>(interrupt: I) -> bool;

    /// Set `interrupt` trigger type
    fn set_trigger_type<I: Nr>(interrupt: I, tt: TriggerType);

    /// Get `interrupt` trigger type
    fn get_trigger_type<I: Nr>(interrupt: I) -> Option<TriggerType>;

    // Set `interrupt` level
    fn set_level<I: Nr>(interrupt: I, level: Level);

    // Get `interrupt` level
    fn get_level<I: Nr>(interrupt: I) -> Level;

    // Set `interrupt` priority
    fn set_priority<I: Nr>(interrupt: I, priority: Priority);

    // Get `interrupt` interrupt
    fn get_priority<I: Nr>(interrupt: I) -> Priority;
}

impl EclicExt for ECLIC {
    fn reset() {
        let eclic = unsafe { &*Self::ptr() };

        eclic.cliccfg.write(|w| unsafe { w.bits(0) });
        eclic.mth.write(|w| unsafe { w.bits(0) });

        for nr in 0..eclic.clicinfo.read().num_interrupt().bits() as usize {
            eclic.clicints[nr].clicintip.write(|w| unsafe { w.bits(0) });
            eclic.clicints[nr].clicintie.write(|w| unsafe { w.bits(0) });
            eclic.clicints[nr]
                .clicintattr
                .write(|w| unsafe { w.bits(0) });
            eclic.clicints[nr]
                .clicintctl
                .write(|w| unsafe { w.bits(0) });
        }
    }

    #[inline]
    fn set_threshold_level(level: Level) {
        unsafe { (*Self::ptr()).mth.write(|w| w.mth().bits(level as u8)) }
    }

    #[inline]
    fn get_threshold_level() -> Level {
        unsafe { core::mem::transmute((*Self::ptr()).mth.read().mth().bits() & 0xF) }
    }

    #[inline]
    fn set_level_priority_bits(lp: LevelPriorityBits) {
        unsafe { (*Self::ptr()).cliccfg.write(|w| w.nlbits().bits(lp as u8)) }
    }

    #[inline]
    fn get_level_priority_bits() -> Option<LevelPriorityBits> {
        match unsafe { (*Self::ptr()).cliccfg.read().nlbits().bits() } {
            0 => Some(LevelPriorityBits::L0P4),
            1 => Some(LevelPriorityBits::L1P3),
            2 => Some(LevelPriorityBits::L2P2),
            3 => Some(LevelPriorityBits::L3P1),
            4 => Some(LevelPriorityBits::L4P0),
            _ => None,
        }
    }

    #[inline]
    fn get_level_bits() -> u8 {
        let bits = unsafe { (*Self::ptr()).cliccfg.read().nlbits().bits() };

        core::cmp::min(bits, EFFECTIVE_LEVEL_PRIORITY_BITS)
    }

    #[inline]
    fn get_priority_bits() -> u8 {
        EFFECTIVE_LEVEL_PRIORITY_BITS - Self::get_level_bits()
    }

    fn setup<I: Nr + Copy>(interrupt: I, tt: TriggerType, level: Level, priority: Priority) {
        Self::mask(interrupt);
        Self::set_trigger_type(interrupt, tt);
        Self::set_level(interrupt, level);
        Self::set_priority(interrupt, priority);
        Self::unpend(interrupt);
    }

    #[inline]
    unsafe fn unmask<I: Nr>(interrupt: I) {
        let nr = usize::from(interrupt.nr());

        (*Self::ptr()).clicints[nr]
            .clicintie
            .write(|w| w.ie().set_bit())
    }

    #[inline]
    fn mask<I: Nr>(interrupt: I) {
        let nr = usize::from(interrupt.nr());

        unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintie
                .write(|w| w.ie().clear_bit())
        }
    }

    #[inline]
    fn is_enabled<I: Nr>(interrupt: I) -> bool {
        let nr = usize::from(interrupt.nr());

        unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintie
                .read()
                .ie()
                .bit_is_set()
        }
    }

    #[inline]
    fn pend<I: Nr>(interrupt: I) {
        let nr = usize::from(interrupt.nr());

        unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintip
                .write(|w| w.ip().set_bit())
        }
    }

    #[inline]
    fn unpend<I: Nr>(interrupt: I) {
        let nr = usize::from(interrupt.nr());

        unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintip
                .write(|w| w.ip().clear_bit())
        }
    }

    #[inline]
    fn is_pending<I: Nr>(interrupt: I) -> bool {
        let nr = usize::from(interrupt.nr());

        unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintip
                .read()
                .ip()
                .bit_is_set()
        }
    }

    #[inline]
    fn set_trigger_type<I: Nr>(interrupt: I, tt: TriggerType) {
        let nr = usize::from(interrupt.nr());

        unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintattr
                .write(|w| w.trig().bits(tt as u8).shv().clear_bit())
        }
    }

    #[inline]
    fn get_trigger_type<I: Nr>(interrupt: I) -> Option<TriggerType> {
        let nr = usize::from(interrupt.nr());

        match unsafe { (*Self::ptr()).clicints[nr].clicintattr.read().trig().bits() } {
            0 => Some(TriggerType::Level),
            1 => Some(TriggerType::RisingEdge),
            3 => Some(TriggerType::FallingEdge),
            _ => None,
        }
    }

    #[inline]
    fn set_level<I: Nr>(interrupt: I, level: Level) {
        let nr = usize::from(interrupt.nr());

        let mut intctl = unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintctl
                .read()
                .level_priority()
                .bits()
        };
        let level_bits = Self::get_level_bits();

        intctl <<= level_bits;
        intctl >>= level_bits;

        let level = core::cmp::min(level as u8, (1 << level_bits) - 1);
        let level = level << (8 - level_bits);

        unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintctl
                .write(|w| w.level_priority().bits(intctl | level))
        }
    }

    #[inline]
    fn get_level<I: Nr>(interrupt: I) -> Level {
        let nr = usize::from(interrupt.nr());

        let intctl = unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintctl
                .read()
                .level_priority()
                .bits()
        };

        let level = (u16::from(intctl) >> (8 - Self::get_level_bits())) as u8;

        // Enum contains all values from 0-15
        unsafe { core::mem::transmute(level & 0xF) }
    }

    #[inline]
    fn set_priority<I: Nr>(interrupt: I, priority: Priority) {
        let nr = usize::from(interrupt.nr());

        let mut intctl = unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintctl
                .read()
                .level_priority()
                .bits()
        };
        let level_bits = Self::get_level_bits();

        intctl >>= 8 - level_bits;
        intctl <<= 8 - level_bits;

        let priority = core::cmp::min(
            priority as u8,
            (1 << (EFFECTIVE_LEVEL_PRIORITY_BITS - level_bits)) - 1,
        );
        let priority = priority << (8 - EFFECTIVE_LEVEL_PRIORITY_BITS);

        unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintctl
                .write(|w| w.level_priority().bits(intctl | priority))
        }
    }

    #[inline]
    fn get_priority<I: Nr>(interrupt: I) -> Priority {
        let nr = usize::from(interrupt.nr());

        let intctl = unsafe {
            (*Self::ptr()).clicints[nr]
                .clicintctl
                .read()
                .level_priority()
                .bits()
        };

        let level_bits = Self::get_level_bits();
        let priority = (u16::from(intctl << level_bits)
            >> (level_bits + (8 - EFFECTIVE_LEVEL_PRIORITY_BITS))) as u8;

        // Enum contains all values from 0-15
        unsafe { core::mem::transmute(priority & 0xF) }
    }
}
