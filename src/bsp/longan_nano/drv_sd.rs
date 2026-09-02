//! longan_nano 板载 TF 卡槽的 SPI 模式 SD 卡驱动
//!
//! 接线（与 Zephyr `boards/sipeed/longan_nano/longan_nano-common.dtsi` 核对，
//! 板载 TF 槽原生挂在 SPI1，不需要 remap）：SCK=PB13、MISO=PB14、MOSI=PB15、
//! CS=PB12（复用为 GPIO 片选，不用硬件 NSS）。插卡即用，无需外接线。
//!
//! 与 `sd_proto` 的分工：帧怎么摆、容量怎么算、地址怎么传是规范纯数学，
//! 在 `crate::sd_proto`（宿主回归）；**本文件只剩硬件行为**——时钟、片选、
//! 初始化时序序列、512 字节数据块的事务。协议层已测的部分，这里不得重写。
//!
//! 关键设计——**整个扇区事务持一整段 `RefCell` 借用，不关中断**，与
//! `drv_uart.rs` 的"每字节短借用在临界区内"形成对照：
//! - UART 的单字节操作是微秒级（57600 波特 ≈ 174 µs），关中断每字节一次
//!   无妨；且 UART 经注册表可被**多个调用方**触碰，必须自身保卫；
//! - SD 的一次读/写是协议原子事务（命令→应答→数据→忙等），任何一步被
//!   切走都会**毁掉协议流**；忙等最坏可到数百毫秒——关中断整段等于杀掉
//!   调度器（systick 中断全部丢失）。所以这里选**单使用者契约**：所有访问
//!   必须经同一把文件系统互斥锁串行化（`BlockDevice` 文档明令），事务期间
//!   即使被抢占，第二个使用者也无法进入（拿不到锁）；万一有代码违反契约
//!   绕过锁直接访问，`RefCell` 双重借用 panic 即探测器——违约立刻炸在
//!   现场，而不是让协议流错位后给出神秘坏数据。
//!
//! 安全论证（`unsafe impl Send/Sync`）与 `drv_uart.rs` 同构但更简单：
//! 本驱动**全程轮询、无 ISR**（SD 卡不用中断），硬件句柄只在"持文件系统
//! 锁的任务"里访问；`sectors`/`ccs` 在构造期写入后只读；单核抢占模型下
//! 唯一风险是第二个使用者插队——由 RefCell 借用契约探测（见上）。

use alloc::boxed::Box;
use core::cell::RefCell;

use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::OutputPin;
use gd32vf103xx_hal::gpio::gpiob::{PB12, PB13, PB14, PB15};
use gd32vf103xx_hal::gpio::{Active, Alternate, Floating, Input, Output, PushPull, State};
use gd32vf103xx_hal::pac::SPI1;
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::spi::{Spi, MODE_0};
use gd32vf103xx_hal::time::U32Ext;

use crate::device::{BlockDevice, Device, DeviceError, DeviceKind, SECTOR_SIZE};
use crate::sd_proto::{
    block_addr, cmd_frame, csd_capacity, CMD0, CMD16, CMD17, CMD24, CMD41, CMD55, CMD58, CMD8,
    CMD9, CRC7_CMD0, CRC7_CMD8,
};

/// SPI1 三线型（SCK/MISO/MOSI 接 PB13/PB14/PB15）
type SdSpi = Spi<SPI1, (PB13<Alternate<PushPull>>, PB14<Input<Floating>>, PB15<Alternate<PushPull>>)>;
/// 片选：PB12 复用为 GPIO 输出（初始高电平，低有效）
type SdCs = PB12<Output<PushPull>>;

/// R1 应答等待上限（字节数）。初始化阶段时钟 400kHz：64 字节 ≈ 1.3 ms。
const RESP_LIMIT: usize = 0x40;
/// 数据令牌（0xFE）等待上限。读块时卡内寻址最坏可达数十 ms：
/// 0x1000 字节在 13.5 MHz ≈ 2.4 ms、在 400 kHz ≈ 82 ms。
const TOKEN_LIMIT: usize = 0x1000;
/// ACMD41 初始化握手重试上限（每次约 12 字节 ≈ 0.24 ms @400kHz，上限约 0.6 s）
const ACMD41_RETRY: usize = 0x1000;
/// 写块后忙等待上限（字节数）：写完成卡会拉低 MISO 直到内部写完（规范
/// 允许最坏约 250 ms）。13.5 MHz 下 1 Mi 字节 ≈ 0.6 s，带裕量封顶；
/// 超时返回 `DeviceError::Timeout`——绝不无限等，系统其他任务还要运行。
const BUSY_LIMIT: usize = 0x10_0000;

/// SPI 模式 SD 卡设备。
///
/// 状态只有三个：SPI 句柄（RefCell 提供 `&self` 下的可变借用）、CS 引脚、
/// 容量与寻址方式（构造期写入、之后只读）。
pub struct SdCard {
    /// SPI 总线句柄：整个扇区事务期间持借用（见模块注释）
    spi: RefCell<SdSpi>,
    /// CS 引脚：同上，事务期低电平
    cs: RefCell<SdCs>,
    /// 容量（扇区数），CSD 解析所得；构造期写入后只读
    sectors: u64,
    /// 寻址方式：true = SDHC/SDXC（扇区号寻址），false = SDSC（字节寻址）
    ccs: bool,
}

// SAFETY: 与 drv_uart.rs 同构的单核论证，且本驱动更简单——
// 1) 无 ISR：SD 全程轮询，硬件句柄不会被中断上下文访问；
// 2) 所有方法调用方必须是持文件系统互斥锁的任务（单使用者契约），
//    同一时刻至多一个任务访问 spi/cs；被抢占时第二个使用者拿不到锁，
//    违反契约则 RefCell 双重借用 panic（探测器，见模块注释）；
// 3) sectors/ccs 在 new() 里构造后只读，无写竞争。
unsafe impl Send for SdCard {}
// SAFETY: 同上；共享引用下的可变访问由单使用者契约串行化。
unsafe impl Sync for SdCard {}

impl SdCard {
    /// 初始化板载 TF 卡并进入就绪态：
    /// 400 kHz 慢速握手（预热→CMD0→CMD8→ACMD41→CMD58→CMD16→CMD9 读 CSD）
    /// 之后把 SPI 提速到 20 MHz（HAL 整数分频实际得到 13.5 MHz，见下），
    /// 返回 `&'static SdCard`（`Box::leak`——设备与系统同寿命，与
    /// `drv_uart.rs` 的 `Uart0::new` 同款，挂驱动层要 'static trait 对象）。
    ///
    /// 失败返回 `DeviceError`：无卡/卡坏/老卡（不支持 CMD8 的 2010 年前 v1.0 卡
    /// 明确拒绝，不做降级——教学内核宁早失败也不静默降级）。
    pub fn new<X, Y, Z, W>(
        spi1: SPI1,
        pb12: PB12<X>,
        pb13: PB13<Y>,
        pb14: PB14<Z>,
        pb15: PB15<W>,
        rcu: &mut Rcu,
    ) -> Result<&'static SdCard, DeviceError>
    where
        X: Active,
        Y: Active,
        Z: Active,
        W: Active,
    {
        // CS 初始高（未选中），随后转成复用引脚
        let mut cs = pb12.into_push_pull_output_with_state(State::High);
        let pins = (
            pb13.into_alternate_push_pull(),
            pb14.into_floating_input(),
            pb15.into_alternate_push_pull(),
        );
        // 规范约束：上电握手必须低速（400 kHz），之后才能提速
        let mut spi = Spi::spi1(spi1, pins, MODE_0, 400.khz(), rcu);
        let (sectors, ccs) = init_sd(&mut spi, &mut cs)?;
        // 提速 20 MHz：HAL 走整数分频（SPI1 基频 54 MHz → 实际 13.5 MHz）。
        // 地板除法陷阱：**此处不能请求 25 MHz**——得 54/2=27 MHz，超出 SD
        // 规范 25 MHz 上限；请求 20 则得 54/4=13.5 MHz，安全落在 "20±" 内。
        spi.change_clock_freq(20.mhz());
        Ok(Box::leak(Box::new(SdCard {
            spi: RefCell::new(spi),
            cs: RefCell::new(cs),
            sectors,
            ccs,
        })))
    }

    /// CS 时序包装：拉低 → 执行 → 拉高（无论成败）。
    /// 借用由本函数整体持有——事务是协议原子单元（见模块注释）。
    fn with_cs<T>(
        &self,
        f: impl FnOnce(&mut SdSpi, &mut SdCs) -> Result<T, DeviceError>,
    ) -> Result<T, DeviceError> {
        let mut spi = self.spi.borrow_mut();
        let mut cs = self.cs.borrow_mut();
        cs.set_low().map_err(|_| DeviceError::Io)?;
        let r = f(&mut spi, &mut cs);
        let _ = cs.set_high();
        r
    }

    /// 读一个扇区：CMD17 → 等数据令牌 0xFE → 512 字节数据 + 2 字节 CRC（不校验）。
    fn read_block(&self, no: u64, buf: &mut [u8]) -> Result<(), DeviceError> {
        self.with_cs(|spi, _cs| {
            let r1 = cmd(spi, CMD17, block_addr(self.ccs, no), 0)?;
            if r1 != 0 {
                return Err(DeviceError::Io); // R1 非 0：卡拒绝（越界/忙等异常）
            }
            wait_token(spi)?;
            // 数据相：主机只负责送时钟（MOSI 内容任意），接收交替输出
            buf.fill(0x00);
            spi.transfer(buf).map_err(|_| DeviceError::Io)?;
            let mut crc = [0xFFu8; 2];
            spi.transfer(&mut crc).map_err(|_| DeviceError::Io)?;
            Ok(())
        })
    }

    /// 写一个扇区：CMD24 → 数据起始令牌 0xFE → 512 字节数据 + 2 字节
    /// CRC 占位（SPI 模式不校验 CRC，但字节数必须给足）→ 应答令牌校验
    /// （'010' 模式 = 接受）→ 忙等待直到 MISO 释放（卡内部写完）。
    fn write_block(&self, no: u64, data: &[u8]) -> Result<(), DeviceError> {
        self.with_cs(|spi, _cs| {
            let r1 = cmd(spi, CMD24, block_addr(self.ccs, no), 0)?;
            if r1 != 0 {
                return Err(DeviceError::Io);
            }
            spi.write(&[0xFE]).map_err(|_| DeviceError::Io)?;
            spi.write(data).map_err(|_| DeviceError::Io)?;
            spi.write(&[0xFF, 0xFF]).map_err(|_| DeviceError::Io)?;
            let mut resp = [0xFFu8];
            spi.transfer(&mut resp).map_err(|_| DeviceError::Io)?;
            // 应答令牌：接受 = 0x05（位 3:1 = '010'）；0x0B = CRC 错、0x0C = 写错
            if (resp[0] & 0x0E) != 0x04 {
                return Err(DeviceError::Io);
            }
            // 忙等：写后卡拉低 MISO 直到内部完成，0xFF = 释放
            for _ in 0..BUSY_LIMIT {
                let mut b = [0xFFu8];
                spi.transfer(&mut b).map_err(|_| DeviceError::Io)?;
                if b[0] == 0xFF {
                    return Ok(());
                }
            }
            Err(DeviceError::Timeout)
        })
    }
}

/// 发一条命令并读 R1。帧组装在 `sd_proto::cmd_frame`（宿主已测）；
/// CRC 只对 CMD0/CMD8 有意义，其余命令传 0（SPI 模式不校验）。
fn cmd(spi: &mut SdSpi, code: u8, arg: u32, crc: u8) -> Result<u8, DeviceError> {
    let frame = cmd_frame(code, arg, crc);
    spi.write(&frame).map_err(|_| DeviceError::Io)?;
    read_r1(spi)
}

/// 读 R1 应答：跳过卡连续输出的 0xFF，取第一个非 0xFF 字节（即 R1）。
/// 超时返回 `Timeout`——CS 已拉低但卡不应答（无卡/卡坏）。
fn read_r1(spi: &mut SdSpi) -> Result<u8, DeviceError> {
    for _ in 0..RESP_LIMIT {
        let mut b = [0xFFu8; 1];
        spi.transfer(&mut b).map_err(|_| DeviceError::Io)?;
        if b[0] != 0xFF {
            return Ok(b[0]);
        }
    }
    Err(DeviceError::Timeout)
}

/// 等待数据起始令牌 0xFE。非 0xFF/0xFE 的字节 = 数据错误令牌（卡报读错误，
/// 低 4 位是错误原因，统一映射为 `Io` 并放弃本次事务）。
fn wait_token(spi: &mut SdSpi) -> Result<(), DeviceError> {
    for _ in 0..TOKEN_LIMIT {
        let mut b = [0xFFu8; 1];
        spi.transfer(&mut b).map_err(|_| DeviceError::Io)?;
        match b[0] {
            0xFF => continue,
            0xFE => return Ok(()),
            _ => return Err(DeviceError::Io), // 数据错误令牌
        }
    }
    Err(DeviceError::Timeout)
}

/// 上电初始化时序（400 kHz，完成后返回 `(容量, 是否 SDHC)`）：
/// 预热 80 时钟 → CMD0 → CMD8(R7 校验) → ACMD41 循环 → CMD58(OCR/CCS) →
/// CMD16(512B) → CMD9(CSD)。每一步的 R1/数据校验见各段注释。
///
/// 独立成自由函数：让 `SdCard::new` 只做"接线 + 收尾提速"，握手序列一眼
/// 看全是规范流程（真机验证页逐行对照）。
fn init_sd(spi: &mut SdSpi, cs: &mut SdCs) -> Result<(u64, bool), DeviceError> {
    // 预热：CS 高时发 80 个时钟（10 字节 0xFF）——卡要看到时钟沿才能解析
    // 命令；CS=1 期间 MISO 高阻，这批时钟只用于提升卡内部电源稳定。
    cs.set_high().map_err(|_| DeviceError::Io)?;
    let mut warm = [0xFFu8; 10];
    spi.write(&warm).map_err(|_| DeviceError::Io)?;

    cs.set_low().map_err(|_| DeviceError::Io)?;

    // CMD0：切入 SPI 模式。卡进入 idle 态，R1 唯一合法应答 = 0x01
    let r1 = cmd(spi, CMD0, 0, CRC7_CMD0)?;
    if r1 != 0x01 {
        return Err(DeviceError::Io);
    }

    // CMD8：电压/校验字握手（区分 SDSC v1 与 SDHC/SDXC v2 的充要命令）。
    // R7 = R1(0x01) + 4 字节：bit2(电压 0x01 = 2.7–3.6 V) + bit3(0xAA 回显)。
    let r1 = cmd(spi, CMD8, 0x0000_01AA, CRC7_CMD8)?;
    if r1 != 0x01 {
        return Err(DeviceError::Io); // 0x05 = 老卡拒绝 CMD8：不降级，明确失败
    }
    let mut r7 = [0xFFu8; 4];
    spi.transfer(&mut r7).map_err(|_| DeviceError::Io)?;
    if r7[2] != 0x01 || r7[3] != 0xAA {
        return Err(DeviceError::Io);
    }

    // ACMD41：上电初始化握手。ACMD 前缀 = CMD55（宣告下一条是应用类命令）；
    // 参数 HCS=1（bit30）声明主机支持 SDHC/SDXC。卡未就绪返回 R1=0x01，
    // 就绪返回 R1=0x00——循环直到就绪或超时。
    let mut ready = false;
    for _ in 0..ACMD41_RETRY {
        let _ = cmd(spi, CMD55, 0, 0)?;
        match cmd(spi, CMD41, 0x4000_0000, 0)? {
            0x00 => {
                ready = true;
                break;
            }
            0x01 => continue, // 还在初始化
            _ => return Err(DeviceError::Io),
        }
    }
    if !ready {
        return Err(DeviceError::Timeout);
    }

    // CMD58：读 OCR。R3 = R1(0x00) + 4 字节 OCR；CCS 位 = OCR bit30 =
    // 数据首字节的 bit6——SDHC/SDXC 标记，决定后续寻址方式
    let r1 = cmd(spi, CMD58, 0, 0)?;
    if r1 != 0 {
        return Err(DeviceError::Io);
    }
    let mut ocr = [0xFFu8; 4];
    spi.transfer(&mut ocr).map_err(|_| DeviceError::Io)?;
    let ccs = (ocr[0] & 0x40) != 0;

    // CMD16：块长定为 512 B（绝对块长；之后所有单块命令按 512 传输）
    let r1 = cmd(spi, CMD16, SECTOR_SIZE as u32, 0)?;
    if r1 != 0 {
        return Err(DeviceError::Io);
    }

    // CMD9：读 CSD（卡专用数据）。16 字节 + 2 字节 CRC（CRC 只在 CMD0/CMD8
    // 校验，其余命令不校验）。容量解析在 sd_proto（宿主已测）
    let r1 = cmd(spi, CMD9, 0, 0)?;
    if r1 != 0 {
        return Err(DeviceError::Io);
    }
    wait_token(spi)?;
    let mut csd = [0xFFu8; 16];
    spi.transfer(&mut csd).map_err(|_| DeviceError::Io)?;
    let mut crc = [0xFFu8; 2];
    spi.transfer(&mut crc).map_err(|_| DeviceError::Io)?;

    cs.set_high().map_err(|_| DeviceError::Io)?;
    Ok((csd_capacity(&csd), ccs))
}

impl Device for SdCard {
    fn kind(&self) -> DeviceKind {
        DeviceKind::Block
    }

    fn as_block(&self) -> Option<&dyn BlockDevice> {
        Some(self)
    }
}

impl BlockDevice for SdCard {
    /// 逻辑扇区大小：SD 协议定死 512 B（CMD16 已按此配置）
    fn sector_size(&self) -> u64 {
        SECTOR_SIZE
    }

    /// 容量（CSD 解析所得；0 = 未就绪——本构造不支持，失败即 Err 返回）
    fn sector_count(&self) -> u64 {
        self.sectors
    }

    /// 读一个扇区。越界返回 `InvalidInput`（坏地址绝不能摸到卡上变成
    /// 垃圾扇区号）；长度必须整 512 B（契约见 `device::BlockDevice`）。
    fn read_sector(&self, no: u64, buf: &mut [u8]) -> Result<(), DeviceError> {
        assert_eq!(buf.len(), SECTOR_SIZE as usize, "块设备契约：读必须整 512 B");
        if no >= self.sectors {
            return Err(DeviceError::InvalidInput);
        }
        self.read_block(no, buf)
    }

    /// 写一个扇区（同上，越界拒绝 + 长度断言）
    fn write_sector(&self, no: u64, data: &[u8]) -> Result<(), DeviceError> {
        assert_eq!(data.len(), SECTOR_SIZE as usize, "块设备契约：写必须整 512 B");
        if no >= self.sectors {
            return Err(DeviceError::InvalidInput);
        }
        self.write_block(no, data)
    }
}
