//! E104-BT5032A 的 AT 命令层——命令编码/应答行解码/会话,全部纯实现。
//!
//! 手册事实(来源:Ebyte 官方手册 §7):
//! - **AT 命令不需要 `\r\n` 终止符**;应答以 `\r\n` 结尾(HEX 返回除外);
//! - 应答形态:`+OK` / `+OK=[para]`(查询回显) / `+ERR=[NUM]` /
//!   事件行 `STA:wakeup`(复位后)等 / `+MAC:xx` 这类 `+KEY:val`;
//! - **`+ERR` 是"模块有应答"**——参数错/状态不允许,不是传输失败
//!   (分层语义,`send` 把它作为 `Ok(RespLine::ErrCode(..))` 返回);
//! - 已连接时进配置模式要拉低 MOD 引脚(书稿踩坑)。
//!
//! 设计与 ch22 `slip` 同款纪律:位级/词法的"会算错"全部在这里,
//! 宿主 golden 常量钉死;真机上只剩 UART 时序。

use crate::device::StreamDevice;
use core::fmt::{self};

/// 最长命令帧字节:"AT+UUIDSVR128="(14) + 32 个 hex + 15 个空格 = 61,取整 64
pub const CMD_MAX: usize = 64;
/// 最长应答行字节("+MAC:0123456789AB" = 17,留余量)
pub const LINE_MAX: usize = 40;
/// `send` 的字节预算——无时钟纯层的"超时"替代品(读满 256 字节仍无完整行即放弃)
pub const RX_BUDGET: usize = 256;

/// 角色模式(AT+ROLE;复位后生效)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    /// 从机(默认)——手机连它
    Slave = 0,
    /// 主机——它连别人
    Master = 1,
    /// Observer——只听不连
    Observer = 2,
}

/// 广播模式(AT+ADV;iBeacon 模式下无法建立连接)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdvMode {
    /// 关广播
    Off = 0,
    /// 普通广播(默认)
    Normal = 1,
    /// iBeacon(不可连接)
    IBeacon = 2,
}

/// 一条 AT 命令(编码到无终止符 ASCII 帧;变体覆盖本章用到的手册命令)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cmd {
    /// `AT` 握手测试
    Test,
    /// `AT+BAUD=N`(N=0..14;8=57600;立即生效并保存)
    Baud(u8),
    /// `AT+ROLE=0|1|2`
    Role(Role),
    /// `AT+UUIDSVR=4352`(16 位服务 UUID,ASCII 十进制;**复位后生效**)
    ServiceUuid(u16),
    /// `AT+UUIDSVR128=11 22 ..`(128 位服务 UUID,大写 hex 空格分隔;
    /// 分隔格式未逐字核实——以实物手册为准)
    ServiceUuid128([u8; 16]),
    /// `AT+UUIDCHARA1=N`(从机→主机通道特征;复位后生效)
    Char1Uuid(u16),
    /// `AT+UUIDCHARA2=N`(主机→从机通道特征;复位后生效)
    Char2Uuid(u16),
    /// `AT+ADV=0|1|2`
    Adv(AdvMode),
    /// `AT+ADVINTV=N`(×0.625ms;160=100ms)
    AdvInterval(u16),
    /// `AT+RESET` 软复位(打印 `\r\nSTA:wakeup`)
    Reset,
    /// `AT+RESTORE` 恢复出厂(期间禁止复位/断电)
    Restore,
    /// `AT+MAC?` 查询 MAC(查询形态未核实——以实物为准)
    Mac,
    /// `AT+VER?` 查询版本
    Ver,
    /// `AT+DISCON` 断开(从机形态,断开全部)
    Disconnect,
}

impl Cmd {
    /// 编码为无终止符 ASCII 帧,返回写入 `out` 的字节数。
    /// 断言 `out.len() >= CMD_MAX`——调用方用定长栈缓冲,越界是编程错误
    /// (`SlipEncoder` 同款契约);**输出绝不含 `\r`/`\n`**(阳性对照扫全变体)。
    pub fn encode(&self, out: &mut [u8]) -> usize {
        assert!(out.len() >= CMD_MAX, "AT 命令缓冲不足");
        let mut w = Buf { b: out, i: 0 };
        match self {
            Cmd::Test => w.str("AT"),
            Cmd::Baud(n) => {
                w.str("AT+BAUD="); // FAULT-INJECT
                w.u8_dec(*n);
            }
            Cmd::Role(r) => {
                w.str("AT+ROLE=");
                w.u8_dec(*r as u8);
            }
            Cmd::ServiceUuid(v) => {
                w.str("AT+UUIDSVR=");
                w.u16_dec(*v);
            }
            Cmd::ServiceUuid128(bytes) => {
                w.str("AT+UUIDSVR128=");
                w.hex_spaced(bytes);
            }
            Cmd::Char1Uuid(v) => {
                w.str("AT+UUIDCHARA1=");
                w.u16_dec(*v);
            }
            Cmd::Char2Uuid(v) => {
                w.str("AT+UUIDCHARA2=");
                w.u16_dec(*v);
            }
            Cmd::Adv(a) => {
                w.str("AT+ADV=");
                w.u8_dec(*a as u8);
            }
            Cmd::AdvInterval(v) => {
                w.str("AT+ADVINTV=");
                w.u16_dec(*v);
            }
            Cmd::Reset => w.str("AT+RESET"),
            Cmd::Restore => w.str("AT+RESTORE"),
            Cmd::Mac => w.str("AT+MAC?"),
            Cmd::Ver => w.str("AT+VER?"),
            Cmd::Disconnect => w.str("AT+DISCON"),
        }
        w.i
    }
}

/// 无分配的写入助手(栈缓冲内拼帧)
struct Buf<'a> {
    b: &'a mut [u8],
    i: usize,
}

impl<'a> Buf<'a> {
    fn str(&mut self, s: &str) {
        for &c in s.as_bytes() {
            self.b[self.i] = c;
            self.i += 1;
        }
    }
    /// 手写十进制(u16 最长 5 位)——协议层零分配纪律
    fn u16_dec(&mut self, v: u16) {
        let mut tmp = [0u8; 5];
        let mut n = 0;
        let mut x = v;
        loop {
            tmp[n] = b'0' + (x % 10) as u8;
            n += 1;
            x /= 10;
            if x == 0 {
                break;
            }
        }
        while n > 0 {
            n -= 1;
            self.b[self.i] = tmp[n];
            self.i += 1;
        }
    }
    fn u8_dec(&mut self, v: u8) {
        self.u16_dec(v as u16);
    }
    /// 大写 hex 两两位 + 空格分隔(尾字节后无空格)——128 位 UUID 形态
    fn hex_spaced(&mut self, bytes: &[u8]) {
        for (k, &b) in bytes.iter().enumerate() {
            if k > 0 {
                self.b[self.i] = b' ';
                self.i += 1;
            }
            self.b[self.i] = hex_char(b >> 4);
            self.b[self.i + 1] = hex_char(b & 0xF);
            self.i += 2;
        }
    }
}

#[inline]
fn hex_char(v: u8) -> u8 {
    match v {
        0..=9 => b'0' + v,
        _ => b'A' + (v - 10),
    }
}

/// 无堆定长行文本(`RespLine` 的字符串载荷载体)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LineText {
    bytes: [u8; LINE_MAX],
    len: u8,
}

impl LineText {
    const fn new() -> Self {
        LineText { bytes: [0; LINE_MAX], len: 0 }
    }
    fn from_slice(s: &[u8]) -> Self {
        let mut t = LineText::new();
        for &b in s.iter().take(LINE_MAX) {
            t.bytes[t.len as usize] = b;
            t.len += 1;
        }
        t
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }
}

impl fmt::Debug for LineText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", core::str::from_utf8(self.as_bytes()).unwrap_or("<非UTF8>"))
    }
}

/// 一条已分类的应答行
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RespLine {
    /// `+OK`
    Ok,
    /// `+OK=8`(查询回显类)
    OkValue(u32),
    /// `+ERR=123`(**模块有应答**——参数/状态错误,不是传输失败)
    ErrCode(u8),
    /// `STA:wakeup` 这类事件行(载荷即字样;事件词汇以实物为准)
    Sta(LineText),
    /// `+MAC:xx` / `+VER:x`(`:` 与 `=` 都认——分隔符未核实,两者兼容)
    KeyValue { key: LineText, val: LineText },
    /// 未能归类的非空行
    Other(LineText),
}

/// 字节流 → 行 → `RespLine`。仿 ch22 `slip` 的 feed 模式:
/// 逐字节喂入,行尾 `\n` 处切行(先剥尾部 `\r`)→ `parse_line` 分类返回。
/// 溢出行(超 `LINE_MAX` 无 `\n`)丢弃并在下一个 `\n` 重同步——丢行不丢同步。
#[derive(Clone, Copy)]
pub struct RespDecoder {
    buf: [u8; LINE_MAX],
    len: usize,
    overflow: bool,
}

impl RespDecoder {
    pub const fn new() -> Self {
        RespDecoder { buf: [0; LINE_MAX], len: 0, overflow: false }
    }

    /// 喂一个字节;完整行产出分类结果(空行跳过)。
    pub fn feed(&mut self, b: u8) -> Option<RespLine> {
        if b == b'\n' {
            // 剥尾部 \r(CRLF 可选)
            let mut end = self.len;
            if end > 0 && self.buf[end - 1] == b'\r' {
                end -= 1;
            }
            let had_overflow = self.overflow;
            self.overflow = false;
            self.len = 0;
            if had_overflow || end == 0 {
                return None; // 溢出行丢弃 / 空行跳过
            }
            return Some(parse_line(&self.buf[..end]));
        }
        if self.len >= LINE_MAX {
            // 攒不下:标记溢出,继续吞字节直到 \n 重同步
            self.overflow = true;
            return None;
        }
        self.buf[self.len] = b;
        self.len += 1;
        None
    }

    /// 取走当前半行并清零。**透明模式的出口**:手机经数据通道写来的裸载荷
    /// 没有 `\r\n`,稳态泵每轮把滞留半行当**数据**取走(行/数据二义性的
    /// 诚实出口——载荷恰含 `\n` 会被误切行,AT 模组的固有模糊性,书稿展开)。
    pub fn take_partial(&mut self) -> &[u8] {
        let r = &self.buf[..self.len];
        // 返回内部切片前先记长度;调用方用完调 reset 见下——这里直接清零
        // 不行(要返回切片),改为返回后由 Session 在下一轮前 reset。
        // 简化:拷贝语义不可行(无堆)——采用"读后即弃":返回切片引用的同时
        // 长度保持,由调用方随后调 reset()。此处仅返回视图。
        r
    }

    /// 丢弃当前半行(`take_partial` 的配套)
    pub fn reset(&mut self) {
        self.len = 0;
        self.overflow = false;
    }
}

/// 行 → `RespLine` 纯函数(独立可测)。
pub fn parse_line(line: &[u8]) -> RespLine {
    if line == b"+OK" {
        return RespLine::Ok;
    }
    if let Some(rest) = strip(line, b"+OK=") {
        return RespLine::OkValue(dec_u32(rest));
    }
    if let Some(rest) = strip(line, b"+ERR=") {
        return RespLine::ErrCode(dec_u32(rest) as u8);
    }
    if let Some(rest) = strip(line, b"STA:") {
        return RespLine::Sta(LineText::from_slice(rest));
    }
    if line.first() == Some(&b'+') {
        // +KEY:val 或 +KEY=val(分隔符兼容)
        for sep in [b':', b'='] {
            if let Some(p) = line.iter().position(|&b| b == sep) {
                return RespLine::KeyValue {
                    key: LineText::from_slice(&line[1..p]),
                    val: LineText::from_slice(&line[p + 1..]),
                };
            }
        }
    }
    RespLine::Other(LineText::from_slice(line))
}

fn strip<'a>(s: &'a [u8], prefix: &[u8]) -> Option<&'a [u8]> {
    if s.len() >= prefix.len() && &s[..prefix.len()] == prefix {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

fn dec_u32(s: &[u8]) -> u32 {
    let mut v = 0u32;
    for &b in s {
        if b.is_ascii_digit() {
            v = v * 10 + (b - b'0') as u32;
        }
    }
    v
}

/// 物理字节流小接口——"非阻塞读 + 阻塞写"契约(镜像 ch22 `net::device::PhyIo`,
/// 放在 ble 内避免跨 feature 依赖):
/// - `read_byte` 仅在 `rx_len() > 0` 时调用(等价非阻塞读);
/// - `write_all` 任务上下文调用,允许轮询阻塞;
/// - `Sync` 上界:方法全 `&self`,经 `&dyn BleIo` 共享。
pub trait BleIo: Sync {
    fn rx_len(&self) -> usize;
    fn read_byte(&self) -> u8;
    fn write_all(&self, buf: &[u8]);
}

/// 一切 `StreamDevice` 都是 `BleIo`(方法转发;trait 名不同只为把
/// "非阻塞读"契约钉进类型——ch22 同款手法;
/// `Sync` 上界由 `StreamDevice: Device: Sync` 隐含)
impl<T: StreamDevice + ?Sized> BleIo for T {
    #[inline]
    fn rx_len(&self) -> usize {
        self.available()
    }
    #[inline]
    fn read_byte(&self) -> u8 {
        let mut b = [0u8; 1];
        // 契约:仅 rx_len() > 0 时调用——流设备的非阻塞读此时必返回 1 字节
        let n = self.read(&mut b).expect("BleIo 契约:available>0 时 read 不应失败");
        debug_assert_eq!(n, 1, "BleIo 契约:available>0 时 read 必返回 1 字节");
        b[0]
    }
    #[inline]
    fn write_all(&self, buf: &[u8]) {
        StreamDevice::write_all(self, buf).expect("BleIo:流设备写不应失败")
    }
}

/// AT 会话错误(传输层;模块侧错误走 `Ok(RespLine::ErrCode(..))`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtError {
    /// 保留:会话被并发占用(当前"泵任务独占"模型下不可达)
    Busy,
    /// 字节预算(`RX_BUDGET`)耗尽仍无完整应答行——无时钟纯层的超时替代品
    Timeout,
}

/// AT 会话:一条 UART 上的命令/应答状态机。
///
/// ⚠️ **红线变迁(书稿踩坑 5):旧版 `read_byte` 是阻塞读**,空缓冲会经任务
/// 状态机挂起,在 `xtask::start()` 之前调用命中 `xworker.current()` 的空指针
/// 解引用——该踩坑面已随驱动层重构移入内核适配器 `device::read_blocking`
/// (它仍是"仅任务上下文",见其上调用契约)。本层的 `read_byte` 现为
/// **非阻塞读**(`rx_len>0` 才调),`send` 空缓冲只走 `yield_now` + 字节
/// 预算——机制上任何上下文安全;但 boot 上下文里中断未开、永远等不到
/// 字节,只会把预算耗成 `Timeout`。纪律不变:boot 上下文只用 `poll()`。
pub struct AtSession<'a> {
    io: &'a dyn BleIo,
    dec: RespDecoder,
    tx: [u8; CMD_MAX],
}

impl<'a> AtSession<'a> {
    pub const fn new(io: &'a dyn BleIo) -> Self {
        AtSession { io, dec: RespDecoder::new(), tx: [0; CMD_MAX] }
    }

    /// 编码并写出一条命令(**无 `\r\n` 终止符**)。不读应答——
    /// 供调用方自行组超时策略(示例的 `wait_line` 助手)。
    pub fn request(&mut self, cmd: &Cmd) {
        let n = cmd.encode(&mut self.tx);
        self.io.write_all(&self.tx[..n]);
    }

    /// 非阻塞排空一口:rx 有字节才读,出完整行即返回。
    /// boot 上下文安全(不碰空缓冲的 read_byte)。
    pub fn poll(&mut self) -> Option<RespLine> {
        while self.io.rx_len() > 0 {
            let b = self.io.read_byte();
            if let Some(line) = self.dec.feed(b) {
                return Some(line);
            }
        }
        None
    }

    /// 发送并收集第一条应答行(空缓冲走 `yield_now` + 字节预算,机制上
    /// 任何上下文安全;boot 下只会耗成 `Timeout`——见类型文档红线变迁)。
    /// 先 `poll` 清场(丢弃上一命令的迟到残行);`RX_BUDGET` 字节内
    /// 无完整行 → `Err(Timeout)`。模块回 `+ERR=N` 仍是 `Ok`——
    /// 传输成功与模块错误分层。
    pub fn send(&mut self, cmd: &Cmd) -> Result<RespLine, AtError> {
        // 清场:把残行排掉(它们属于上一条命令)
        while self.poll().is_some() {}
        self.request(cmd);
        let mut budget = RX_BUDGET;
        loop {
            while self.io.rx_len() > 0 {
                if budget == 0 {
                    return Err(AtError::Timeout);
                }
                budget -= 1;
                let b = self.io.read_byte();
                if let Some(line) = self.dec.feed(b) {
                    return Ok(line);
                }
            }
            // 空缓冲:让出 CPU 等 ISR 填充(任务上下文;宿主测试下预算
            // 在迭代中耗尽走 Timeout——不碰阻塞读,任何上下文安全)
            if budget == 0 {
                return Err(AtError::Timeout);
            }
            budget -= 1;
            crate::task::yield_now();
        }
    }

    /// 透明模式写:原样字节(连接后 UART→FFF1 notify,无任何包装)
    pub fn write_raw(&mut self, data: &[u8]) {
        self.io.write_all(data);
    }

    /// 半行取走(透传载荷出口;用完调 `reset_partial`)
    pub fn take_partial(&mut self) -> &[u8] {
        self.dec.take_partial()
    }

    /// `take_partial` 的配套(读后即弃)
    pub fn reset_partial(&mut self) {
        self.dec.reset();
    }
}

/// 最小 GATT 配置序列(书稿 §6.8 实测路径)——**故意只有 7 条、不含 `Adv`**:
/// `AT+UUIDSVR/CHARA1/CHARA2/ADVINTV` 都是**复位后生效**,所以第 7 条
/// `Reset` 是分界;开广播(第 9 步 `AT+ADV=1`)必须由调用方在等完
/// `STA:wakeup` 之后另发——这个"数组故意不含 ADV"就是教学点
/// (书稿代码精读 5:为什么 Reset 在中间)。
pub fn configure_gatt(svr: u16, ch1: u16, ch2: u16, adv_intv: u16) -> [Cmd; 7] {
    [
        Cmd::Test,
        Cmd::Role(Role::Slave),
        Cmd::ServiceUuid(svr),
        Cmd::Char1Uuid(ch1),
        Cmd::Char2Uuid(ch2),
        Cmd::AdvInterval(adv_intv),
        Cmd::Reset,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    // ---- 编码 golden(逐字节) ----

    fn enc(cmd: &Cmd) -> Vec<u8> {
        let mut out = [0u8; CMD_MAX];
        let n = cmd.encode(&mut out);
        out[..n].to_vec()
    }

    #[test]
    fn encode_golden_table() {
        assert_eq!(enc(&Cmd::Test), b"AT");
        assert_eq!(enc(&Cmd::Baud(8)), b"AT+BAUD=8");
        assert_eq!(enc(&Cmd::Role(Role::Slave)), b"AT+ROLE=0");
        assert_eq!(enc(&Cmd::Role(Role::Master)), b"AT+ROLE=1");
        assert_eq!(enc(&Cmd::Role(Role::Observer)), b"AT+ROLE=2");
        assert_eq!(enc(&Cmd::ServiceUuid(4352)), b"AT+UUIDSVR=4352");
        assert_eq!(enc(&Cmd::Char1Uuid(4353)), b"AT+UUIDCHARA1=4353");
        assert_eq!(enc(&Cmd::Char2Uuid(4354)), b"AT+UUIDCHARA2=4354");
        assert_eq!(enc(&Cmd::Adv(AdvMode::Off)), b"AT+ADV=0");
        assert_eq!(enc(&Cmd::Adv(AdvMode::Normal)), b"AT+ADV=1");
        assert_eq!(enc(&Cmd::Adv(AdvMode::IBeacon)), b"AT+ADV=2");
        assert_eq!(enc(&Cmd::AdvInterval(160)), b"AT+ADVINTV=160");
        assert_eq!(enc(&Cmd::AdvInterval(16384)), b"AT+ADVINTV=16384");
        assert_eq!(enc(&Cmd::AdvInterval(32)), b"AT+ADVINTV=32");
        assert_eq!(enc(&Cmd::Reset), b"AT+RESET");
        assert_eq!(enc(&Cmd::Restore), b"AT+RESTORE");
        assert_eq!(enc(&Cmd::Disconnect), b"AT+DISCON");
        assert_eq!(enc(&Cmd::Mac), b"AT+MAC?");
        assert_eq!(enc(&Cmd::Ver), b"AT+VER?");
    }

    #[test]
    fn encode_uuid128_hex_spacing() {
        // 16 字节固定样例 → 32 个大写 hex + 15 个空格 = 61 字节
        let bytes = [
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0xAA, 0xBB, 0xCC, 0xDD,
            0xEE, 0xFF,
        ];
        let v = enc(&Cmd::ServiceUuid128(bytes));
        assert_eq!(
            String::from_utf8(v.clone()).unwrap(),
            "AT+UUIDSVR128=11 22 33 44 55 66 77 88 99 00 AA BB CC DD EE FF"
        );
        assert_eq!(v.len(), "AT+UUIDSVR128=".len() + 16 * 3 - 1); // 尾字节后无空格
    }

    #[test]
    fn encode_no_terminator_positive_control() {
        // 阳性对照:encode 若多吐 \r 或 \n 立即红("命令无终止符"契约)
        let all = [
            Cmd::Test,
            Cmd::Baud(8),
            Cmd::Role(Role::Slave),
            Cmd::ServiceUuid(0xFFFF),
            Cmd::ServiceUuid128([0u8; 16]),
            Cmd::Char1Uuid(0xFFF1),
            Cmd::Char2Uuid(0xFFF2),
            Cmd::Adv(AdvMode::Normal),
            Cmd::AdvInterval(1600),
            Cmd::Reset,
            Cmd::Restore,
            Cmd::Mac,
            Cmd::Ver,
            Cmd::Disconnect,
        ];
        for c in &all {
            for &b in &enc(c) {
                assert!(b != b'\r' && b != b'\n', "{c:?} 编码混入终止符");
            }
        }
    }

    // ---- 行解码语料 ----

    fn feed_all(bytes: &[u8]) -> Vec<RespLine> {
        let mut d = RespDecoder::new();
        let mut out = Vec::new();
        for &b in bytes {
            if let Some(l) = d.feed(b) {
                out.push(l);
            }
        }
        out
    }

    #[test]
    fn decode_ok_plain() {
        // 前缀 \r\n 与行尾 \r\n 都在,只在最后一个 \n 产出
        assert_eq!(feed_all(b"\r\n+OK\r\n"), vec![RespLine::Ok]);
    }

    #[test]
    fn decode_ok_value() {
        assert_eq!(feed_all(b"\r\n+OK=8\r\n"), vec![RespLine::OkValue(8)]);
    }

    #[test]
    fn decode_err() {
        assert_eq!(feed_all(b"\r\n+ERR=123\r\n"), vec![RespLine::ErrCode(123)]);
    }

    #[test]
    fn decode_sta_wakeup() {
        assert_eq!(
            feed_all(b"\r\nSTA:wakeup\r\n"),
            vec![RespLine::Sta(LineText::from_slice(b"wakeup"))]
        );
    }

    #[test]
    fn decode_mac_keyvalue() {
        assert_eq!(
            feed_all(b"\r\n+MAC:0C43D33F4B10\r\n"),
            vec![RespLine::KeyValue {
                key: LineText::from_slice(b"MAC"),
                val: LineText::from_slice(b"0C43D33F4B10"),
            }]
        );
    }

    #[test]
    fn decode_lines_in_order() {
        // STA:wakeup 行 + +OK 行按序产出
        assert_eq!(
            feed_all(b"\r\nSTA:wakeup\r\n\r\n+OK\r\n"),
            vec![
                RespLine::Sta(LineText::from_slice(b"wakeup")),
                RespLine::Ok,
            ]
        );
    }

    #[test]
    fn decode_lf_only_tolerated() {
        // 无 \r 的 LF 行也认(CRLF 的 \r 可选)
        assert_eq!(feed_all(b"+OK\n"), vec![RespLine::Ok]);
    }

    #[test]
    fn decode_half_line_pending() {
        // 半行不算行;take_partial 取回后 reset
        let mut d = RespDecoder::new();
        for &b in b"\r\n+OK" {
            assert!(d.feed(b).is_none());
        }
        assert_eq!(d.take_partial(), b"+OK");
        d.reset();
        assert_eq!(d.take_partial(), b"");
    }

    #[test]
    fn decode_overflow_resync() {
        // 超长行(>LINE_MAX 无 \n)丢弃,下一正常行恢复——丢行不丢同步
        let mut bytes = vec![b'X'; LINE_MAX + 8];
        bytes.push(b'\n');
        bytes.extend_from_slice(b"\r\n+OK\r\n");
        assert_eq!(feed_all(&bytes), vec![RespLine::Ok]);
    }

    #[test]
    fn decode_empty_lines_skipped() {
        assert_eq!(feed_all(b"\r\n\r\n\r\n+OK\r\n"), vec![RespLine::Ok]);
    }

    #[test]
    fn parse_line_accepts_eq_and_colon() {
        // 分隔符未核实——':' 与 '=' 都归 KeyValue
        assert_eq!(parse_line(b"+MAC=0C43"), parse_line(b"+MAC:0C43"));
    }

    // ---- AtSession(模拟 IO) ----

    /// 测试用字节流:tx 捕获 + **应答延迟到达**模拟——预载进 `pending`,
    /// 在 write_all(命令发出的瞬间)才转入 rx,对应真实模组"命令→应答"
    /// 的因果序(静态预载会被 send 的清场误吃)。与 ch22 MockLink 同款论证。
    struct MockIo {
        rx: RefCell<VecDeque<u8>>,
        pending: RefCell<VecDeque<u8>>,
        tx: RefCell<Vec<Vec<u8>>>,
    }
    // SAFETY: 仅测试用;单线程测试内借用严格串行
    unsafe impl Sync for MockIo {}

    impl MockIo {
        fn new() -> Self {
            MockIo {
                rx: RefCell::new(VecDeque::new()),
                pending: RefCell::new(VecDeque::new()),
                tx: RefCell::new(Vec::new()),
            }
        }
        /// 预载"命令的应答"(write_all 时才到达)
        fn preload(&self, bytes: &[u8]) {
            self.pending.borrow_mut().extend(bytes.iter().copied());
        }
        /// 直接塞进 rx(模拟"上一命令的迟到残行")
        fn stale(&self, bytes: &[u8]) {
            self.rx.borrow_mut().extend(bytes.iter().copied());
        }
        fn last_tx(&self) -> Vec<u8> {
            self.tx.borrow().last().cloned().unwrap_or_default()
        }
    }

    impl BleIo for MockIo {
        fn rx_len(&self) -> usize {
            self.rx.borrow().len()
        }
        fn read_byte(&self) -> u8 {
            self.rx.borrow_mut().pop_front().unwrap()
        }
        fn write_all(&self, buf: &[u8]) {
            self.tx.borrow_mut().push(buf.to_vec());
            // 命令发出 → 应答到达(因果序)
            let mut p = self.pending.borrow_mut();
            self.rx.borrow_mut().extend(p.drain(..));
        }
    }

    #[test]
    fn session_send_roundtrip_golden() {
        let io = MockIo::new();
        io.preload(b"\r\n+OK\r\n");
        let mut s = AtSession::new(&io);
        assert_eq!(s.send(&Cmd::Test), Ok(RespLine::Ok));
        assert_eq!(io.last_tx(), b"AT"); // 命令逐字节(无终止符)
    }

    #[test]
    fn session_send_module_err_is_ok() {
        // 分层语义:+ERR 是"模块有应答",不是传输失败
        let io = MockIo::new();
        io.preload(b"\r\n+ERR=1\r\n");
        let mut s = AtSession::new(&io);
        assert_eq!(s.send(&Cmd::Test), Ok(RespLine::ErrCode(1)));
    }

    #[test]
    fn session_send_flushes_stale_lines() {
        let io = MockIo::new();
        io.stale(b"\r\n+STALE1\r\n\r\n+STALE2\r\n"); // 迟到残行,清场丢弃
        io.preload(b"\r\n+OK\r\n"); // 本命令应答(命令发出后到达)
        let mut s = AtSession::new(&io);
        assert_eq!(s.send(&Cmd::Reset), Ok(RespLine::Ok));
        // 残行已被清场消费,应答已读走——后续 poll 为空
        assert!(s.poll().is_none());
    }

    #[test]
    fn session_poll_drains_all() {
        let io = MockIo::new();
        io.stale(b"\r\n+OK=8\r\n\r\n+MAC:AB\r\n"); // 直接在 rx(已到达)
        let mut s = AtSession::new(&io);
        assert!(matches!(s.poll(), Some(RespLine::OkValue(8))));
        assert!(matches!(s.poll(), Some(RespLine::KeyValue { .. })));
        assert!(s.poll().is_none());
    }

    #[test]
    fn session_write_raw_passthrough() {
        let io = MockIo::new();
        let mut s = AtSession::new(&io);
        s.write_raw(b"hello");
        assert_eq!(io.last_tx(), b"hello"); // 原样,无包装
    }

    #[test]
    fn session_send_budget_timeout_positive_control() {
        // 阳性对照:预填 >RX_BUDGET 的无换行垃圾 → 预算耗尽路径
        // (不依赖阻塞分支——纯层"超时"的替身)
        let io = MockIo::new();
        io.stale(&vec![b'X'; RX_BUDGET + 16]); // 无换行垃圾直接在 rx
        let mut s = AtSession::new(&io);
        // 注意:清场只丢弃完整行,无换行的垃圾字节留在 rx 里耗预算
        assert_eq!(s.send(&Cmd::Test), Err(AtError::Timeout));
    }

    #[test]
    fn configure_gatt_sequence_golden() {
        let seq = configure_gatt(0x1102, 0x1103, 0x1104, 160);
        assert_eq!(seq.len(), 7);
        assert_eq!(seq[0], Cmd::Test);
        assert_eq!(seq[1], Cmd::Role(Role::Slave));
        assert_eq!(seq[2], Cmd::ServiceUuid(0x1102));
        assert_eq!(seq[3], Cmd::Char1Uuid(0x1103));
        assert_eq!(seq[4], Cmd::Char2Uuid(0x1104));
        assert_eq!(seq[5], Cmd::AdvInterval(160));
        assert_eq!(seq[6], Cmd::Reset);
        // "数组故意不含 ADV"——Reset 分界教学点:
        // 任何元素都不是 Adv 变体
        for c in &seq {
            assert!(!matches!(c, Cmd::Adv(_)), "配置序列不应含 Adv(复位后另发)");
        }
    }

    #[test]
    fn u16_decimal_full_width() {
        // 十进制 itoa 上界:0xFFFF = 65535(五位)
        assert_eq!(enc(&Cmd::ServiceUuid(65535)), b"AT+UUIDSVR=65535");
    }
}
