use log::{LevelFilter, Metadata, Record, SetLoggerError};

#[cfg(any(feature = "gd32vf103", feature = "stm32f1", feature = "stm32f4",))]
use crate::sprintln;
// time 只被 StdoutLogger（芯片 feature 门控）和 RTTLogger（rtt_log 且非测试）使用，
// host 测试构建两个都不编译，这里也要同步门控，否则产生 unused import 警告
#[cfg(any(
    feature = "gd32vf103",
    feature = "stm32f1",
    feature = "stm32f4",
    all(feature = "rtt_log", not(test))
))]
use crate::time;

// logger 选择：rtt 优先，其次 stdout，都没有（或 host 测试）时用空 logger。
// 注意 host 上 rtt_target 不可用（按 arm/riscv target 门控），测试构建强制空 logger。
#[cfg(all(feature = "rtt_log", not(test)))]
static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Debug);

#[cfg(all(not(feature = "rtt_log"), feature = "stdout_log", not(test)))]
static LOGGER: StdoutLogger = StdoutLogger::new(LevelFilter::Debug);

#[cfg(any(test, not(any(feature = "rtt_log", feature = "stdout_log"))))]
static LOGGER: NoopLogger = NoopLogger;

pub fn init() -> Result<(), SetLoggerError> {
    #[cfg(all(feature = "rtt_log", not(test)))]
    rtt_target::rtt_init_print!();

    #[cfg(not(atomic_cas))]
    unsafe {
        log::set_logger_racy(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
    }
    #[cfg(atomic_cas)]
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

/// 空 logger，丢弃所有日志。用于 host 测试，或未启用任何日志后端的配置。
#[cfg(any(test, not(any(feature = "rtt_log", feature = "stdout_log"))))]
pub struct NoopLogger;

#[cfg(any(test, not(any(feature = "rtt_log", feature = "stdout_log"))))]
impl log::Log for NoopLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        false
    }
    fn log(&self, _record: &Record) {}
    fn flush(&self) {}
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
                //tick 只读一次：tick() 要进临界区，读两次既多一次开关中断，
                //打印的 tick 与换算出的分钟数还可能差一个 tick 对不上
                let ticks = time::tick();
                let ticks_min = ticks / (crate::chip::TICK_CLOCK_HZ as u64 * 60);
                sprintln!(
                    "{}/{}min used({}KiB) free({}KiB) {:?}: {:?} {} - {}",
                    ticks,
                    ticks_min,
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

#[cfg(all(feature = "rtt_log", not(test)))]
pub struct RTTLogger {
    level_filter: LevelFilter,
}

#[cfg(all(feature = "rtt_log", not(test)))]
impl RTTLogger {
    pub const fn new(level_filter: LevelFilter) -> RTTLogger {
        RTTLogger { level_filter }
    }
}

#[cfg(all(feature = "rtt_log", not(test)))]
impl log::Log for RTTLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.level_filter.ge(&metadata.level())
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            //与 StdoutLogger 同理：tick 只读一次，少进一次临界区且两个字段自洽
            let ticks = time::tick();
            let ticks_min = ticks / (crate::chip::TICK_CLOCK_HZ as u64 * 60);
            rtt_target::rprintln!(
                "{}/{}min used({}KiB) free({}KiB) {:?}: {:?} {} - {}",
                ticks,
                ticks_min,
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

    fn flush(&self) {}
}
