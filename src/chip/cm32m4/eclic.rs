//! 芯来科技core采用的中断控制器
//!

use core::ops::Deref;

pub trait EclicExt {
    fn set_level(n: u8, level: u8);
    fn set_priority(n: u8, prio: u8);
    fn set_trigger_type(n: u8, typ: u8);
    fn pend(n: u8);
    fn unpend(n: u8);
    fn mask(n: u8);
    fn unmask(n: u8);
}

pub struct CLICCFG {
    reg: vcell::VolatileCell<u8>,
}

impl CLICCFG {
    #[inline]
    pub fn set_nlbits(&self, nlbits: u32) {
        self.reg.set(self.reg.get() & (!(0xf << 1)));
        self.reg
            .set(self.reg.get() | (((nlbits << 1) & ((0xf as u32) << 1)) as u8));
    }

    pub fn get_nlbits(&self) -> u32 {
        ((self.reg.get() & (0xf << 1)) >> 1) as u32
    }
}

pub struct CLICINFO {
    reg: vcell::VolatileCell<u32>,
}

impl CLICINFO {
    pub fn get_info_ctl_bits(&self) -> u32 {
        ((self.reg.get() & (0xF << 21)) >> 0) as u32
    }
}

pub struct CLICCTRL {
    intip: vcell::VolatileCell<u8>,
    intie: vcell::VolatileCell<u8>,
    intattr: vcell::VolatileCell<u8>,
    intctrl: vcell::VolatileCell<u8>,
}

impl CLICCTRL {
    #[inline]
    fn enable(&self) {
        self.intie.set(self.intie.get() | (0x1 << 0));
    }
    #[inline]
    pub fn disable(&self) {
        self.intie.set(self.intie.get() & !(0x1 << 0));
    }
    #[inline]
    pub fn pending(&self) {
        self.intip.set(self.intip.get() | (0x1 << 0));
    }
    #[inline]
    pub fn unpending(&self) {
        self.intip.set(self.intip.get() & !(0x1 << 0));
    }
    // *                 - 00  level trigger, \ref ECLIC_LEVEL_TRIGGER
    // *                 - 01  positive edge trigger, \ref ECLIC_POSTIVE_EDGE_TRIGGER
    // *                 - 02  level trigger, \ref ECLIC_LEVEL_TRIGGER
    // *                 - 03  negative edge trigger, \ref ECLIC_NEGTIVE_EDGE_TRIGGER
    #[inline]
    pub fn set_trigger(&self, trig: u8) {
        self.intattr.set(self.intattr.get() & !(0x3 << 1));
        self.intattr.set(self.intattr.get() | (trig << 1));
    }

    #[inline]
    pub fn set_shv(&self, shv: u32) {
        self.intattr.set(self.intattr.get() & !(0x1 << 0));
        self.intattr.set(self.intattr.get() | ((shv << 0) as u8));
    }
    #[inline]
    pub fn set_ctrl(&self, ctrl: u8) {
        self.intctrl.set(ctrl);
    }
    #[inline]
    pub fn get_ctrl(&self) -> u8 {
        self.intctrl.get()
    }
}

pub struct RegisterBlock {
    cfg: CLICCFG,
    _reserved0: [u8; 3],

    info: CLICINFO,

    _reserved1: [u8; 3],
    _mth: vcell::VolatileCell<u8>,

    _reserved2: [u32; 0x3FD],
    ctl: [CLICCTRL; 8],
}

pub struct ECLIC;
impl ECLIC {
    pub fn mask(n: u8) {
        unsafe { (*Self::ptr()).ctl[n as usize].disable() }
    }
    pub fn unmask(n: u8) {
        unsafe { (*Self::ptr()).ctl[n as usize].enable() }
    }
    pub fn pending(n: u8) {
        unsafe { (*Self::ptr()).ctl[n as usize].pending() }
    }
    pub fn unpending(n: u8) {
        unsafe { (*Self::ptr()).ctl[n as usize].unpending() }
    }
    pub fn set_trigger(n: u8, trig: u8) {
        unsafe { (*Self::ptr()).ctl[n as usize].set_trigger(trig) }
    }

    pub fn set_level(n: u8, mut lvl_abs: u8) {
        let mut nlbits = unsafe { (*Self::ptr()).cfg.get_nlbits() } as u8;

        let intctlbits = unsafe { (*Self::ptr()).info.get_info_ctl_bits() } as u8;

        if nlbits == 0 {
            return;
        }

        if nlbits > intctlbits {
            nlbits = intctlbits;
        }
        let maxlvl = (1 << nlbits) - 1;
        if lvl_abs > maxlvl {
            lvl_abs = maxlvl;
        }
        let lvl = lvl_abs << (8 - nlbits);
        let mut cur_ctrl = unsafe { (*Self::ptr()).ctl[n as usize].get_ctrl() };
        cur_ctrl = cur_ctrl << nlbits;
        cur_ctrl = cur_ctrl >> nlbits;
        unsafe { (*Self::ptr()).ctl[n as usize].set_ctrl(cur_ctrl | lvl) };
    }

    pub fn set_priority(n: u8, mut pri: u8) {
        let nlbits = unsafe { (*Self::ptr()).cfg.get_nlbits() } as u8;
        let intctlbits = unsafe { (*Self::ptr()).info.get_info_ctl_bits() } as u8;

        if nlbits < intctlbits {
            let maxpri = (1 << (intctlbits - nlbits)) - 1;
            if pri > maxpri {
                pri = maxpri;
            }
            pri = pri << (8 - intctlbits);
            let mask = u8::MAX >> intctlbits;
            pri = pri | mask;
            let mut cur_ctrl = unsafe { (*Self::ptr()).ctl[n as usize].get_ctrl() };
            cur_ctrl = cur_ctrl >> (8 - nlbits);
            cur_ctrl = cur_ctrl << (8 - nlbits);
            unsafe { (*Self::ptr()).ctl[n as usize].set_ctrl(cur_ctrl | pri) };
        }
    }
}
unsafe impl Send for ECLIC {}
impl ECLIC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const RegisterBlock = super::ECLIC_CTRL_ADDR as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const RegisterBlock {
        Self::PTR
    }
}
impl Deref for ECLIC {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ECLIC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECLIC").finish()
    }
}
