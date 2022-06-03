use log::{LevelFilter, Metadata, Record, SetLoggerError};

use rtt_target::*;
#[cfg(any(feature = "gd32vf103", feature = "stm32f1", feature = "stm32f4",))]
use crate::{sprintln, time};

static LOGGER: StdoutLogger = StdoutLogger::new(LevelFilter::Debug);

pub fn init() -> Result<(), SetLoggerError> {
    #[cfg(not(atomic_cas))]
    unsafe {
        log::set_logger_racy(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
    }
    #[cfg(atomic_cas)]
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

pub struct StdoutLogger {
    level_filter: LevelFilter,
}

impl StdoutLogger {
    pub const fn new(level_filter: LevelFilter) -> Self {
        Self { level_filter }
    }
}

impl log::Log for StdoutLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.level_filter.ge(&metadata.level())
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            #[cfg(any(feature = "gd32vf103", feature = "stm32f1", feature = "stm32f4",))]
            {
                let ticks_sec = crate::tick_ms() / 1000 / 60;
                sprintln!(
                    "{}/{}min used({}KiB) free({}KiB) {:?}: {:?} {} - {}",
                    time::tick(),
                    ticks_sec,
                    crate::used_memory() / 1024,
                    crate::free_memory() / 1024,
                    if let Some(file) = record.file() {
                        file
                    } else {
                        "-"
                    },
                    if let Some(line) = record.line() {
                        line
                    } else {
                        0
                    },
                    record.level(),
                    record.args()
                );
            }
        }
    }

    fn flush(&self) {}
}

pub struct RTTLogger {
    level_filter: LevelFilter,
}

impl RTTLogger {
    pub const fn new(level_filter: LevelFilter) -> RTTLogger {
        RTTLogger { level_filter }
    }
}

impl log::Log for RTTLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.level_filter.ge(&metadata.level())
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            rprintln!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
