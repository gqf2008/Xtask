//! 寄存器访问助手 `RegMap`：I2C/SPI 器件的 consumer 侧助手（v3 §5.15.2）。
//!
//! 定位：`RegMap` 不是新能力——它只是 [`BusDevice`] 的薄封装，把"拼 tx/rx、
//! 定地址宽度、翻字节序、读改写、轮询"这些每颗器件的样板代码收拢到 core 层。
//! 不进 `as_*`、不进注册表、不新增能力 trait；取用者持 `BusDevice` 的常驻
//! 句柄，外面套一个 [`BusArbiter`] 即可。
//!
//! 帧格式（与 `src/device.rs` 的 `BusDevice` 契约一致）：
//! - 寄存器地址走 tx 前缀：`addr_bits = 8` → 1 字节；`addr_bits = 16` → 2 字节，
//!   字节序由 `big_endian` 决定；
//! - 数据固定 4 字节（`read` 返回 / `write` 入参都是 `u32`），字节序同 `big_endian`；
//! - 读 = `transfer(&地址前缀, &mut [0u8; 4])`：写地址 + 读数据两段时序由**总线
//!   驱动内部拼接**（`BusDevice` 文档原文），RegMap 不拆成两次 transfer；
//! - 写 = `transfer(&[地址前缀.., 数据..], &mut [])`。
//!
//! 使用边界（务必读）：
//! - **数据宽度固定 4 字节**：这是固定 API 的形状（`u32` 载荷），不是可配项。
//!   只有 8 位数据寄存器的器件（多数小型 I2C 传感器）不要直接使用本助手——
//!   多读/多写会 NAK 或越界；等 API 扩展数据宽度后再接入。
//! - **`addr` 只作诊断**：`BusDevice` 在构造期已绑定从机地址（I2C 建模，见
//!   `device.rs` 的 `BusDevice` 文档），tx 里再编一次地址字节会与驱动内的地址
//!   阶段重复；[`RegMap::dev_addr`] 只供日志/断言。
//! - **C2 纪律**：四个方法都不得在 `sync::free` 内调用（R8，入口 debug 断言）。
//!   `read`/`write`/`update_bits` 不阻塞；`poll_bits` 会 `sleep_ms`，必须在调度器
//!   启动后的任务上下文调用，且绝不能放进 [`BusArbiter::with_transaction`] 的
//!   闭包——release 构建没有断言，在关中断的临界区里睡眠会直接挂死。
//! - 零分配：全部缓冲是栈上定长数组；生产代码不含 unsafe。

use core::time::Duration;

use crate::device::bus::BusArbiter;
use crate::device::DeviceError;

/// 寄存器地址前缀的最大字节数（`addr_bits = 16`）
const ADDR_MAX: usize = 2;
/// 数据载荷固定 4 字节（`read`/`write` 的载荷类型是 `u32`）
const DATA_LEN: usize = 4;

/// I2C/SPI 寄存器访问助手：地址宽度、字节序随实例定制，驱动不再手拼 tx/rx。
pub struct RegMap<'a> {
    /// 共享总线仲裁器：事务边界由它给出，RegMap 自己不持锁
    arbiter: &'a BusArbiter<'a>,
    /// 器件（从机）地址——仅诊断；`BusDevice` 构造期已绑定地址，不参与帧拼装
    addr: u16,
    /// 寄存器地址宽度：8 或 16 位
    addr_bits: u8,
    /// 地址与数据的字节序（`true` = 大端）
    big_endian: bool,
}

impl<'a> RegMap<'a> {
    /// 构造寄存器访问助手。
    ///
    /// 构造期**不做校验**（签名无 `Result`）：非法的 `addr_bits` / 超宽 `reg`
    /// 在首次访问时回 `InvalidInput`，调用方无需为配置错误准备第二条错误路径。
    pub fn new(arbiter: &'a BusArbiter<'a>, addr: u16, addr_bits: u8, big_endian: bool) -> Self {
        Self {
            arbiter,
            addr,
            addr_bits,
            big_endian,
        }
    }

    /// 器件（从机）地址（C1，仅诊断）。
    pub fn dev_addr(&self) -> u16 {
        self.addr
    }

    /// 读一个 32 位寄存器（C2）。
    ///
    /// 一次 `transfer`：tx = 地址前缀，rx = 4 字节数据。参数非法 →
    /// `InvalidInput`；总线错误原样透传。
    pub fn read(&self, reg: u32) -> Result<u32, DeviceError> {
        crate::sync::assert_c2("RegMap::read");
        let n = self.addr_len(reg)?;
        let big_endian = self.big_endian;
        let mut tx = [0u8; ADDR_MAX];
        self.encode_addr(reg, &mut tx);
        self.arbiter.with_transaction(|bus| {
            let mut rx = [0u8; DATA_LEN];
            bus.transfer(&tx[..n], &mut rx)?;
            Ok(decode_u32(&rx, big_endian))
        })
    }

    /// 写一个 32 位寄存器（C2）。tx = 地址前缀 ++ 4 字节数据，rx 为空。
    pub fn write(&self, reg: u32, val: u32) -> Result<(), DeviceError> {
        crate::sync::assert_c2("RegMap::write");
        let n = self.addr_len(reg)?;
        let big_endian = self.big_endian;
        let mut tx = [0u8; ADDR_MAX + DATA_LEN];
        self.encode_addr(reg, &mut tx);
        encode_u32(val, big_endian, &mut tx[n..n + DATA_LEN]);
        self.arbiter
            .with_transaction(|bus| bus.transfer(&tx[..n + DATA_LEN], &mut []))
    }

    /// 读-改-写（C2）。
    ///
    /// **不变量**：读阶段与写阶段两次 `transfer` 必须在同一个
    /// [`BusArbiter::with_transaction`] 闭包内——否则同总线其他器件可插进
    /// 读与写之间，并发写会丢位（§5.15.2）。因此本方法不调用 `read`/`write`
    /// （那会二次进入事务），而是把两次 transfer 内联。
    ///
    /// 新值 `new = (old & !mask) | (val & mask)`；无条件写回（不因无变化跳过）。
    pub fn update_bits(&self, reg: u32, mask: u32, val: u32) -> Result<(), DeviceError> {
        crate::sync::assert_c2("RegMap::update_bits");
        let n = self.addr_len(reg)?;
        let big_endian = self.big_endian;
        let mut tx = [0u8; ADDR_MAX + DATA_LEN];
        self.encode_addr(reg, &mut tx);
        self.arbiter.with_transaction(|bus| {
            // 读阶段
            let mut rx = [0u8; DATA_LEN];
            bus.transfer(&tx[..n], &mut rx)?;
            let old = decode_u32(&rx, big_endian);
            let new = (old & !mask) | (val & mask);
            // 写阶段：同一个闭包 = 同一个事务
            encode_u32(new, big_endian, &mut tx[n..n + DATA_LEN]);
            bus.transfer(&tx[..n + DATA_LEN], &mut [])
        })
    }

    /// 轮询直到 `(v & mask) == (expect & mask)`；超时回 `Timeout`（C2，R7）。
    ///
    /// 超时参数显式传入（`Duration`）；循环每轮都查截止时刻，不存在无限等待。
    /// 命中即返回——即使首读已耗时超过 `timeout`，结果成立就成功；超时只约束
    /// "等待条件成立"。`tick_ms` 是 1 ms 粒度、`sleep_ms(1)` 最小 1 tick，
    /// 因此 `Duration` 小于 1 ms 时语义等价于"探一次立即判超时"。
    /// 每轮读各自是一次独立事务；跨轮不持有总线（轮询不需要跨轮原子性）。
    ///
    /// **红线**：必须由调度器启动后的任务调用；不得在 `sync::free` 或
    /// `with_transaction` 闭包内调用（release 下会在关中断状态睡眠）。
    pub fn poll_bits(
        &self,
        reg: u32,
        mask: u32,
        expect: u32,
        timeout: Duration,
    ) -> Result<(), DeviceError> {
        crate::sync::assert_c2("RegMap::poll_bits");
        let ms = timeout.as_millis().min(u64::MAX as u128) as u64;
        let start = crate::time::tick_ms();
        loop {
            let v = self.read(reg)?;
            if (v & mask) == (expect & mask) {
                return Ok(());
            }
            // 先查截止再睡：超时为 0 时"探一次"即返回 Timeout，不进入 sleep_ms
            if crate::time::tick_ms().wrapping_sub(start) >= ms {
                return Err(DeviceError::Timeout);
            }
            crate::task::sleep_ms(1);
        }
    }

    // ---- 私有编解码 ----

    /// 校验地址宽度与 `reg` 范围，返回地址前缀字节数（1 或 2）。
    fn addr_len(&self, reg: u32) -> Result<usize, DeviceError> {
        let n = match self.addr_bits {
            8 => 1,
            16 => 2,
            _ => return Err(DeviceError::InvalidInput),
        };
        if reg > (1u32 << self.addr_bits) - 1 {
            return Err(DeviceError::InvalidInput);
        }
        Ok(n)
    }

    /// 地址前缀编码（调用前必须已过 [`Self::addr_len`] 校验）。
    fn encode_addr(&self, reg: u32, out: &mut [u8]) {
        if self.addr_bits == 8 {
            out[0] = reg as u8;
        } else if self.big_endian {
            out[0] = (reg >> 8) as u8;
            out[1] = reg as u8;
        } else {
            out[0] = reg as u8;
            out[1] = (reg >> 8) as u8;
        }
    }
}

/// 数据载荷编码（固定 4 字节）。
#[inline]
fn encode_u32(val: u32, big_endian: bool, out: &mut [u8]) {
    let bytes = if big_endian {
        val.to_be_bytes()
    } else {
        val.to_le_bytes()
    };
    out[..DATA_LEN].copy_from_slice(&bytes);
}

/// 数据载荷解码（固定 4 字节）。
#[inline]
fn decode_u32(buf: &[u8; DATA_LEN], big_endian: bool) -> u32 {
    if big_endian {
        u32::from_be_bytes(*buf)
    } else {
        u32::from_le_bytes(*buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::BusDevice;
    use alloc::boxed::Box;
    use alloc::collections::VecDeque;
    use alloc::vec::Vec;
    use core::cell::{Cell, RefCell};

    /// mock 总线：记录每次 transfer 的 (tx 副本, rx 声明长度)，按脚本回读，
    /// 可注入底层错误。
    struct MockBus {
        /// 帧级日志：每次 transfer 一项（含失败的那次）
        log: RefCell<Vec<(Vec<u8>, usize)>>,
        /// 预置读回数据：每次 transfer 弹一个（不回填则 rx 保持 0）
        reads: RefCell<VecDeque<[u8; DATA_LEN]>>,
        /// transfer 总次数（失败也计数——用于"读失败后不得进入写阶段"）
        transfers: Cell<usize>,
        /// 注入错误：有值则每次 transfer 都失败
        fail: Cell<Option<DeviceError>>,
    }

    // SAFETY: 仅宿主测试内使用；实例经 Box::leak 存活，且只在创建它的测试线程
    // 访问，RefCell/Cell 的访问严格串行（与 src/device.rs tests 的论证同款）。
    unsafe impl Sync for MockBus {}

    impl MockBus {
        fn new() -> Self {
            Self {
                log: RefCell::new(Vec::new()),
                reads: RefCell::new(VecDeque::new()),
                transfers: Cell::new(0),
                fail: Cell::new(None),
            }
        }

        /// 预置一次读回数据（按字节原样写入 rx）
        fn read_back(&self, bytes: [u8; DATA_LEN]) {
            self.reads.borrow_mut().push_back(bytes);
        }

        /// 注入底层错误
        fn fail_with(&self, err: DeviceError) {
            self.fail.set(Some(err));
        }

        /// transfer 总次数
        fn transfer_count(&self) -> usize {
            self.transfers.get()
        }

        /// 第 `i` 次 transfer 的 tx 内容
        fn tx(&self, i: usize) -> Vec<u8> {
            self.log.borrow()[i].0.clone()
        }

        /// 第 `i` 次 transfer 的 rx 声明长度
        fn rx_len(&self, i: usize) -> usize {
            self.log.borrow()[i].1
        }
    }

    impl crate::device::DeviceBase for MockBus {
        fn kind(&self) -> crate::device::DeviceKind {
            crate::device::DeviceKind::Bus
        }
    }

    impl crate::device::Device for MockBus {
        fn as_bus(&self) -> Option<&dyn BusDevice> {
            Some(self)
        }
    }

    impl BusDevice for MockBus {
        fn transfer(&self, tx: &[u8], rx: &mut [u8]) -> Result<(), DeviceError> {
            self.transfers.set(self.transfers.get() + 1);
            self.log.borrow_mut().push((tx.to_vec(), rx.len()));
            if let Some(err) = self.fail.get() {
                return Err(err);
            }
            if let Some(data) = self.reads.borrow_mut().pop_front() {
                // 防御性截断：本助手的 rx 恒为 4 字节
                let n = rx.len().min(DATA_LEN);
                rx[..n].copy_from_slice(&data[..n]);
            }
            Ok(())
        }
    }

    /// 测试夹具：leak 出 bus 与 arbiter（每个用例 owner 名唯一，互不干扰），
    /// 并按其配置构造 `RegMap`。
    ///
    /// 注意：`BusArbiter::new` 只应保存本实例 owner、不写全局 owner 表
    /// （写表只发生在 `claim_owner`），否则 19 条用例的 owner 会触顶小容量表。
    fn fixture(
        owner: &'static str,
        addr_bits: u8,
        big_endian: bool,
    ) -> (&'static MockBus, RegMap<'static>) {
        let bus: &'static MockBus = Box::leak(Box::new(MockBus::new()));
        let lock: &'static crate::sync::mutex::Mutex<()> =
            Box::leak(Box::new(crate::sync::mutex::Mutex::new(())));
        let arb: &'static BusArbiter<'static> =
            Box::leak(Box::new(BusArbiter::new(bus, owner, lock)));
        (bus, RegMap::new(arb, 0x50, addr_bits, big_endian))
    }

    /// 1. 8 位小端读：tx 只有地址前缀，rx 4 字节按小端解码。
    #[test]
    fn read_8bit_le_frames_address_only() {
        let (bus, map) = fixture("regmap-read8", 8, false);
        bus.read_back([0xEF, 0xBE, 0xAD, 0xDE]);
        assert_eq!(map.read(0x12), Ok(0xDEAD_BEEF));
        assert_eq!(bus.transfer_count(), 1);
        assert_eq!(bus.tx(0), [0x12]);
        assert_eq!(bus.rx_len(0), DATA_LEN);
    }

    /// 2. 8 位地址 + 大端：大端标志只影响数据，1 字节地址不得被改写。
    #[test]
    fn read_8bit_be_keeps_address_bytes() {
        let (bus, map) = fixture("regmap-read8be", 8, true);
        bus.read_back([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(map.read(0x12), Ok(0x1122_3344));
        assert_eq!(bus.tx(0), [0x12], "大端不得影响 1 字节地址");
    }

    /// 3. 8 位小端写：tx = 地址前缀 ++ 4 字节小端数据，rx 为空。
    #[test]
    fn write_8bit_le_frames_addr_then_data() {
        let (bus, map) = fixture("regmap-write8", 8, false);
        map.write(0x12, 0xDEAD_BEEF).unwrap();
        assert_eq!(bus.transfer_count(), 1);
        assert_eq!(bus.tx(0), [0x12, 0xEF, 0xBE, 0xAD, 0xDE]);
        assert_eq!(bus.rx_len(0), 0);
    }

    /// 4. 16 位大端读：地址前缀按大端编码，数据按大端解码。
    #[test]
    fn read_16bit_be_decodes_be() {
        let (bus, map) = fixture("regmap-read16be", 16, true);
        bus.read_back([0x01, 0x02, 0x03, 0x04]);
        assert_eq!(map.read(0x1234), Ok(0x0102_0304));
        assert_eq!(bus.tx(0), [0x12, 0x34]);
    }

    /// 5. 16 位小端读：地址前缀与数据都按小端。
    #[test]
    fn read_16bit_le_decodes_le() {
        let (bus, map) = fixture("regmap-read16le", 16, false);
        bus.read_back([0x44, 0x33, 0x22, 0x11]);
        assert_eq!(map.read(0x1234), Ok(0x1122_3344));
        assert_eq!(bus.tx(0), [0x34, 0x12]);
        assert_eq!(bus.rx_len(0), DATA_LEN);
    }

    /// 6. 16 位小端写：地址前缀与数据都按小端拼接。
    #[test]
    fn write_16bit_le_frames_addr_and_data() {
        let (bus, map) = fixture("regmap-write16", 16, false);
        map.write(0x1234, 0xA1B2_C3D4).unwrap();
        assert_eq!(bus.tx(0), [0x34, 0x12, 0xD4, 0xC3, 0xB2, 0xA1]);
    }

    /// 7. 读-改-写：恰两次 transfer 且顺序固定（读在前、写在后）。
    ///
    /// 结构验证：固定 API 无法观测事务入口数；本用例只证明两次 transfer 与
    /// 顺序，单事务由源码结构保证（`update_bits` 内恰一处 `with_transaction`，
    /// 闭包内不得调用 `self.read`/`self.write`）。
    #[test]
    fn update_bits_is_read_modify_write_in_one_transaction() {
        let (bus, map) = fixture("regmap-update", 8, false);
        bus.read_back([0xFF, 0x00, 0xFF, 0x00]); // old = 0x00FF_00FF (LE)
        map.update_bits(0x20, 0x0000_FF00, 0x0000_AB00).unwrap();
        assert_eq!(bus.transfer_count(), 2, "读一次 + 写一次");
        assert_eq!(bus.tx(0), [0x20]);
        // new = (old & !mask) | (val & mask) = 0x00FF_ABFF（小端）
        assert_eq!(bus.tx(1), [0x20, 0xFF, 0xAB, 0xFF, 0x00]);
        assert_eq!(bus.rx_len(0), DATA_LEN);
        assert_eq!(bus.rx_len(1), 0);
    }

    /// 8. 掩码外的位必须原样保留（old 全 1、mask 中间字节、val = 0）。
    #[test]
    fn update_bits_keeps_bits_outside_mask() {
        let (bus, map) = fixture("regmap-mask", 8, false);
        bus.read_back([0xFF, 0xFF, 0xFF, 0xFF]);
        map.update_bits(0x20, 0x00FF_0000, 0).unwrap();
        // new = 0xFF00_FFFF（小端）：只有 mask 覆盖的字节被清零
        assert_eq!(bus.tx(1), [0x20, 0xFF, 0xFF, 0x00, 0xFF]);
    }

    /// 9. 读阶段失败后不得进入写阶段：transfer 计数必须停在 1。
    #[test]
    fn update_bits_does_not_write_when_read_fails() {
        let (bus, map) = fixture("regmap-update-fail", 8, false);
        bus.fail_with(DeviceError::Io);
        assert_eq!(map.update_bits(0x20, 0xFF, 0x01), Err(DeviceError::Io));
        assert_eq!(bus.transfer_count(), 1, "读失败后不得写回");
    }

    /// 10. 首读即命中：Ok，且只发生一次 transfer。
    #[test]
    fn poll_bits_returns_ok_on_first_match() {
        let (bus, map) = fixture("regmap-poll-hit", 8, false);
        bus.read_back([0b1010, 0, 0, 0]);
        assert_eq!(
            map.poll_bits(0x01, 0b0010, 0b0010, Duration::from_millis(10)),
            Ok(())
        );
        assert_eq!(bus.transfer_count(), 1);
        assert_eq!(bus.tx(0), [0x01]);
    }

    /// 11. 零超时：探一次即判超时，不进入 sleep_ms（宿主上 sleep 必 panic）。
    #[test]
    fn poll_bits_zero_timeout_probes_once() {
        let (bus, map) = fixture("regmap-poll-zero", 8, false);
        assert_eq!(
            map.poll_bits(0x01, 1, 1, Duration::ZERO),
            Err(DeviceError::Timeout)
        );
        assert_eq!(bus.transfer_count(), 1);
    }

    /// 12. 亚毫秒超时（as_millis() == 0）与零超时同语义：只探一次。
    #[test]
    fn poll_bits_sub_millisecond_timeout_probes_once() {
        let (bus, map) = fixture("regmap-poll-subms", 8, false);
        assert_eq!(
            map.poll_bits(0x01, 1, 1, Duration::from_micros(999)),
            Err(DeviceError::Timeout)
        );
        assert_eq!(bus.transfer_count(), 1);
    }

    /// 13. 非法 addr_bits：四个方法都回 InvalidInput，且不碰总线。
    #[test]
    fn invalid_addr_bits_is_invalid_input_without_bus_access() {
        let (bus, map) = fixture("regmap-bits", 12, false);
        assert_eq!(map.read(0), Err(DeviceError::InvalidInput));
        assert_eq!(map.write(0, 0), Err(DeviceError::InvalidInput));
        assert_eq!(map.update_bits(0, 0xFF, 0), Err(DeviceError::InvalidInput));
        assert_eq!(
            map.poll_bits(0, 1, 1, Duration::ZERO),
            Err(DeviceError::InvalidInput)
        );
        assert_eq!(bus.transfer_count(), 0, "参数校验必须先于总线访问");
    }

    /// 14. 寄存器宽度超出 addr_bits：越界回 InvalidInput，边界值放行。
    #[test]
    fn register_wider_than_addr_bits_is_invalid_input() {
        let (bus8, map8) = fixture("regmap-range8", 8, false);
        assert_eq!(map8.read(0x100), Err(DeviceError::InvalidInput));
        assert_eq!(map8.write(0x100, 0), Err(DeviceError::InvalidInput));
        assert_eq!(map8.read(0xFF), Ok(0), "边界 0xFF 应放行");
        assert_eq!(bus8.transfer_count(), 1, "只有放行那次碰总线");

        let (bus16, map16) = fixture("regmap-range16", 16, false);
        assert_eq!(map16.read(0x1_0000), Err(DeviceError::InvalidInput));
        assert_eq!(map16.read(0xFFFF), Ok(0), "边界 0xFFFF 应放行");
        assert_eq!(bus16.transfer_count(), 1, "只有放行那次碰总线");
    }

    /// 15. poll_bits 的 reg 越界：InvalidInput 且不碰总线（超时为 0 也不读）。
    #[test]
    fn poll_bits_invalid_reg_is_invalid_input_without_bus_access() {
        let (bus, map) = fixture("regmap-poll-range", 8, false);
        assert_eq!(
            map.poll_bits(0x100, 1, 1, Duration::ZERO),
            Err(DeviceError::InvalidInput)
        );
        assert_eq!(bus.transfer_count(), 0);
    }

    /// 16. 底层错误原样透传（不做本地映射）。
    #[test]
    fn transfer_error_is_propagated_unchanged() {
        // 三个方法各自独立注入，避免相互影响
        {
            let (bus, map) = fixture("regmap-err-read", 8, false);
            bus.fail_with(DeviceError::Io);
            assert_eq!(map.read(0x01), Err(DeviceError::Io));
        }
        {
            let (bus, map) = fixture("regmap-err-write", 8, false);
            bus.fail_with(DeviceError::Io);
            assert_eq!(map.write(0x01, 0), Err(DeviceError::Io));
        }
        {
            let (bus, map) = fixture("regmap-err-update", 8, false);
            bus.fail_with(DeviceError::Io);
            assert_eq!(map.update_bits(0x01, 1, 1), Err(DeviceError::Io));
        }
    }

    /// 17. addr 只作诊断：构造参数可观测。
    #[test]
    fn dev_addr_is_reported() {
        let (_bus, map) = fixture("regmap-addr", 8, false);
        assert_eq!(map.dev_addr(), 0x50);
    }

    /// 18. 栈上 arbiter 的协变闸门：**不使用 Box::leak 包装 arbiter**。
    ///
    /// 本行钉住 `BusArbiter<'static>` → `BusArbiter<'local>` 的协变；若
    /// bus.rs 引入生命周期参数化的 `Cell`/`&mut` 不变字段，本用例编译失败。
    #[test]
    fn stack_arbiter_borrow_is_accepted() {
        let bus: &'static MockBus = Box::leak(Box::new(MockBus::new()));
        // 显式钉死 `arb: BusArbiter<'static>`：不写类型时区域推断可在 `new()`
        // 处把 `'a` 缩短到栈借用，即使 `BusArbiter` 不变也能编译，本用例就失去
        // 闸门作用；写死后才真正要求 `BusArbiter<'static> <: BusArbiter<'local>`。
        let lock: &'static crate::sync::mutex::Mutex<()> =
            Box::leak(Box::new(crate::sync::mutex::Mutex::new(())));
        let arb: BusArbiter<'static> = BusArbiter::new(bus, "regmap-covariance", lock);
        let map = RegMap::new(&arb, 0x50, 8, false);
        assert_eq!(map.dev_addr(), 0x50);
    }

    /// 19. 编译期断言：`RegMap<'a>` 的跨任务可用性等价于 `BusArbiter<'a>: Sync`。
    /// 若 bus.rs 有意保持 !Sync，应删除本用例并在文档写明"仅任务内使用"。
    #[test]
    fn regmap_is_sync_when_arbiter_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<RegMap<'static>>();
    }
}
