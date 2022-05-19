/*!
  Registers that are not reset as long as Vbat or Vdd has power.

  The registers retain their values during wakes from standby mode or system resets. They also
  retain their value when Vdd is switched off as long as V_BAT is powered.
  The backup domain also contains tamper protection and writes to it must be enabled in order
  to use the real time clock (RTC).
  Write access to the backup domain is enabled using the `BKP::configure(rcu, &mut pmu)`
  function.
*/

use crate::pac::{BKP, PMU};
use crate::rcu::{Rcu, Enable};

/// Extension trait that sets up the `BKP` peripheral
pub trait BkpExt {
    /// Configure the `BKP` peripheral
    fn configure(self, rcu: &mut Rcu, pmu: &mut PMU) -> BackupDomain;
}

impl BkpExt for BKP {
    fn configure(self, rcu: &mut Rcu, pmu: &mut PMU) -> BackupDomain {
        // Enable the backup interface
        BKP::enable(rcu);
        PMU::enable(rcu);

        // Enable access to the backup registers
        pmu.ctl.modify(|_r, w| w.bkpwen().set_bit());
        BackupDomain {
            _regs: self
        }
    }
}

/**
  The existence of this struct indicates that writing to the the backup
  domain has been enabled. It is acquired by calling `configure` on `BKP`
*/
pub struct BackupDomain {
    pub(crate) _regs: BKP,
}
