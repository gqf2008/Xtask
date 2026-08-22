//! SLIP(RFC 1055)帧编解码——第 22 章"会算错"的半层,与 ch21 的 `sd_proto` 同款哲学:
//! 协议位级编码全部下沉为宿主可测的纯函数/纯状态机,真机上只剩"字节流"这一件事。
//!
//! RFC 1055(SLIP)帧格式(在串行线路上实际传输的字节序):
//!
//! ```text
//! END | 帧数据(转义后) | END            END = 0xC0
//! ```
//!
//! 转义规则(只有两个有意义的字节):
//! - 数据里的 `0xC0` → 发送为 `0xDB 0xDC`(ESC + ESC_END);
//! - 数据里的 `0xDB` → 发送为 `0xDB 0xDD`(ESC + ESC_ESC);
//! - 其余字节原样。两端各有且仅有一个 END 作定界符,连续 END 是空帧,应被忽略。
//!
//! 丢帧策略(与"重传"的分层边界,书稿教学点):SLIP 是**尽力而为**的链路层——
//! 非法转义、超长帧、半截帧一律**丢弃整帧**并重同步到下一个 END;不纠错、不重传。
//! 重传是 TCP 的事(第 22 章代码精读 2 展开)。

/// 帧定界符(END)。RFC 1055 起止各一个
pub const SLIP_END: u8 = 0xC0;
/// 转义引导符(ESC)。后跟一个字节表示"这是被转义的原字节"
pub const SLIP_ESC: u8 = 0xDB;
/// 转义后的 END(0xC0 在线路上的形态)
pub const SLIP_ESC_END: u8 = 0xDC;
/// 转义后的 ESC(0xDB 在线路上的形态)
pub const SLIP_ESC_ESC: u8 = 0xDD;

/// 默认点到点链路 MTU。RFC 1055 建议端点 MTU 不超过 1006,
/// BSD slattach 的传统默认是 296——低速串口下小 MTU 让单帧更快发完。
/// (书稿"改造型练习":改小观察 ping 报文切分)
pub const SLIP_MTU: usize = 296;
/// 最坏情况编码长度:所有字节都要转义(每字节变 2 字节)+ 两端 END
pub const SLIP_MAX_FRAME: usize = 2 * SLIP_MTU + 2;

/// 解码状态机的三态:
/// - `Data`: 正常收数据;
/// - `Escaped`: 刚收到 ESC,下一字节决定转义结果(或判非法);
/// - `Discard`: 本帧已判坏(非法转义/超长),丢掉直到下一个 END。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SlipState {
    Data,
    Escaped,
    Discard,
}

/// 字节流 → 帧。逐字节喂入,在帧尾 END 处产出完整的一帧。
pub struct SlipDecoder {
    state: SlipState,
    /// 已装入 out 的字节数;Discard 期间保持最后值,END 时清零
    len: usize,
}

impl SlipDecoder {
    pub const fn new() -> Self {
        SlipDecoder { state: SlipState::Data, len: 0 }
    }

    /// 逐字节喂入。
    ///
    /// - `out` 是帧装配缓冲,`out.len()` 即本解码器能接受的**最大帧长**;
    /// - 返回 `Some(len)` 表示 `out[..len]` 是一帧完整数据(帧尾 END 已消费);
    /// - 返回 `None` 表示帧未到 / 空帧被丢 / 本帧已判坏在丢弃中。
    ///
    /// 契约:返回 `Some` 后、下一次 `feed` 前,调用方必须把 `out[..len]`
    /// 取走(我们的设备侧会 `rx_len = len` 后停止喂入,见 `device.rs`)。
    pub fn feed(&mut self, byte: u8, out: &mut [u8]) -> Option<usize> {
        match self.state {
            SlipState::Discard => {
                // 坏帧:吞掉一切字节直到帧尾 END,然后回到正常态等待下一帧
                if byte == SLIP_END {
                    self.state = SlipState::Data;
                    self.len = 0;
                }
                None
            }
            SlipState::Escaped => {
                match byte {
                    SLIP_ESC_END => {
                        self.push(SLIP_END, out);
                        self.state = SlipState::Data;
                        None
                    }
                    SLIP_ESC_ESC => {
                        self.push(SLIP_ESC, out);
                        self.state = SlipState::Data;
                        None
                    }
                    // 非法转义(ESC 后不是 DC/DD,包括 END):整帧作废
                    _ => {
                        self.state = SlipState::Discard;
                        None
                    }
                }
            }
            SlipState::Data => match byte {
                SLIP_END => {
                    if self.len == 0 {
                        // 连续 END = 空帧,RFC 1055 要求接收方忽略
                        None
                    } else {
                        let n = self.len;
                        self.len = 0; // 下一帧从零开始
                        Some(n)
                    }
                }
                SLIP_ESC => {
                    self.state = SlipState::Escaped;
                    None
                }
                b => {
                    self.push(b, out);
                    None
                }
            },
        }
    }

    /// 任意状态回 Data 起点:丢出正在装配的半帧,从头再来
    pub fn reset(&mut self) {
        self.state = SlipState::Data;
        self.len = 0;
    }

    #[inline]
    fn push(&mut self, b: u8, out: &mut [u8]) {
        if self.len < out.len() {
            out[self.len] = b;
            self.len += 1;
        } else {
            // 超长帧:装不下,判坏,丢弃直到下一 END
            self.state = SlipState::Discard;
        }
    }
}

/// 帧 → 字节流。纯函数、无状态(结构体仅为 API 对称)。
pub struct SlipEncoder;

impl SlipEncoder {
    pub const fn new() -> Self {
        SlipEncoder
    }

    /// 编码一帧:`END + 转义后的帧数据 + END`,返回写入 out 的字节数。
    ///
    /// 契约:`out.len() >= 2 * frame.len() + 2`(最坏全转义),否则 panic——
    /// 调用方(设备侧)用 `SLIP_MAX_FRAME` 大小的缓冲,越界是编程错误。
    pub fn encode(&self, frame: &[u8], out: &mut [u8]) -> usize {
        assert!(
            out.len() >= 2 * frame.len() + 2,
            "SLIP 编码缓冲不足: frame={} out={}",
            frame.len(),
            out.len()
        );
        let mut i = 0;
        out[i] = SLIP_END;
        i += 1;
        for &b in frame {
            match b {
                SLIP_END => {
                    out[i] = SLIP_ESC;
                    out[i + 1] = SLIP_ESC_END;
                    i += 2;
                }
                SLIP_ESC => {
                    out[i] = SLIP_ESC;
                    out[i + 1] = SLIP_ESC_ESC;
                    i += 2;
                }
                _ => {
                    out[i] = b;
                    i += 1;
                }
            }
        }
        out[i] = SLIP_END;
        i + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 解一帧的测试助手:喂入全部字节,返回最后产出的帧
    fn decode_all(bytes: &[u8], out: &mut [u8]) -> Option<usize> {
        let mut d = SlipDecoder::new();
        let mut res = None;
        for &b in bytes {
            if let Some(n) = d.feed(b, out) {
                res = Some(n);
            }
        }
        res
    }

    #[test]
    fn encoder_golden_minimal() {
        // 成品帧常量钉死:最简单的帧 AB → C0 41 42 C0
        let mut out = [0u8; 16];
        let n = SlipEncoder::new().encode(b"AB", &mut out);
        assert_eq!(&out[..n], &[SLIP_END, b'A', b'B', SLIP_END]);
    }

    #[test]
    fn encoder_escapes_control() {
        // 两个控制字节各自转义成两字节: C0 DB → C0 DB DC DB DD C0
        let mut out = [0u8; 16];
        let n = SlipEncoder::new().encode(&[SLIP_END, SLIP_ESC], &mut out);
        assert_eq!(
            &out[..n],
            &[SLIP_END, SLIP_ESC, SLIP_ESC_END, SLIP_ESC, SLIP_ESC_ESC, SLIP_END]
        );
    }

    #[test]
    fn encode_roundtrip_all_bytes() {
        // 0..=255 全字节序列 roundtrip——漏转义 0xC0/0xDB 中任何一个即红
        let frame: Vec<u8> = (0u16..=255).map(|v| v as u8).collect();
        let mut enc = [0u8; 2 * 256 + 2];
        let n = SlipEncoder::new().encode(&frame, &mut enc);
        let mut out = [0u8; 256];
        let m = decode_all(&enc[..n], &mut out).expect("应解出完整帧");
        assert_eq!(m, 256);
        assert_eq!(&out[..m], &frame[..]);
    }

    #[test]
    fn encoder_worst_case_two_x() {
        // 296 字节全 0xC0 → 恰好 594 = 2*SLIP_MTU + 2(最坏界数学钉死)
        let frame = [SLIP_END; SLIP_MTU];
        let mut out = [0u8; SLIP_MAX_FRAME];
        let n = SlipEncoder::new().encode(&frame, &mut out);
        assert_eq!(n, 2 * SLIP_MTU + 2);
    }

    #[test]
    fn decoder_frame_byte_at_a_time() {
        // 成品帧逐字节喂入:仅最后一个 END 产出 Some,之前恒 None
        let mut enc = [0u8; 16];
        let n = SlipEncoder::new().encode(b"hello", &mut enc);
        let mut d = SlipDecoder::new();
        let mut out = [0u8; 16];
        for (i, &b) in enc[..n].iter().enumerate() {
            let r = d.feed(b, &mut out);
            if i + 1 == n {
                assert_eq!(r, Some(5));
            } else {
                assert_eq!(r, None);
            }
        }
    }

    #[test]
    fn decoder_empty_frames_ignored() {
        // 连续 END:空帧恒被忽略,不产出
        let mut out = [0u8; 16];
        assert_eq!(decode_all(&[SLIP_END, SLIP_END], &mut out), None);
        // 两端 END 之间有内容才算一帧
        let mut d = SlipDecoder::new();
        assert!(d.feed(SLIP_END, &mut out).is_none());
        assert!(d.feed(b'X', &mut out).is_none());
        assert!(d.feed(SLIP_END, &mut out).is_some());
    }

    #[test]
    fn decoder_invalid_escape_discards_whole_frame() {
        // ESC 后接非法后缀(0x55):本帧作废,直到下一个 END,之后的帧不受影响
        let mut out = [0u8; 16];
        let mut d = SlipDecoder::new();
        d.feed(SLIP_END, &mut out); // 帧起始
        d.feed(b'A', &mut out);
        d.feed(SLIP_ESC, &mut out);
        d.feed(0x55, &mut out); // 非法转义 → 丢帧
        d.feed(b'B', &mut out); // 丢弃中
        assert!(d.feed(SLIP_END, &mut out).is_none()); // 坏帧结束,不产出
        // 下一帧正常
        d.feed(SLIP_END, &mut out);
        d.feed(b'C', &mut out);
        assert_eq!(d.feed(SLIP_END, &mut out), Some(1));
        assert_eq!(out[0], b'C');
    }

    #[test]
    fn decoder_overflow_discards() {
        // 帧长超过缓冲容量:判坏丢弃,至下一完整帧恢复
        let mut out = [0u8; 4];
        let mut d = SlipDecoder::new();
        assert!(d.feed(SLIP_END, &mut out).is_none());
        for i in 0..6u8 {
            let r = d.feed(b'A' + i, &mut out);
            assert!(r.is_none(), "超长帧不应产出");
        }
        assert!(d.feed(SLIP_END, &mut out).is_none()); // 坏帧终止
        d.feed(SLIP_END, &mut out);
        d.feed(b'Z', &mut out);
        assert_eq!(d.feed(SLIP_END, &mut out), Some(1));
    }

    #[test]
    fn decoder_escape_then_end_discards() {
        // ESC 后直接 END(帧尾落在转义态):按非法转义处理,丢整帧
        let mut out = [0u8; 16];
        let mut d = SlipDecoder::new();
        d.feed(SLIP_END, &mut out);
        d.feed(b'A', &mut out);
        d.feed(SLIP_ESC, &mut out);
        assert!(d.feed(SLIP_END, &mut out).is_none(), "非法转义帧不应产出");
        // 下一帧正常恢复
        d.feed(SLIP_END, &mut out);
        d.feed(b'B', &mut out);
        assert_eq!(d.feed(SLIP_END, &mut out), Some(1));
    }

    #[test]
    fn decoder_reset_from_discard() {
        // 任意状态 reset 后从零开始
        let mut out = [0u8; 16];
        let mut d = SlipDecoder::new();
        d.feed(SLIP_END, &mut out);
        d.feed(b'A', &mut out);
        d.feed(SLIP_ESC, &mut out);
        d.feed(0x55, &mut out); // 进入 Discard
        d.reset();
        // 现在能正常解一帧
        d.feed(SLIP_END, &mut out);
        d.feed(b'Z', &mut out);
        assert_eq!(d.feed(SLIP_END, &mut out), Some(1));
        assert_eq!(out[0], b'Z');
    }

    #[test]
    fn decoder_half_frame_pending() {
        // 半截帧(缺尾部 END)不算帧:喂一半不产出,补上后产出
        let mut enc = [0u8; 16];
        let n = SlipEncoder::new().encode(b"hello", &mut enc);
        let mut out = [0u8; 16];
        // 去掉最后 2 字节(数据尾 + END),让帧停在半截
        assert_eq!(decode_all(&enc[..n - 2], &mut out), None);
    }

    #[test]
    fn decoder_accepts_bare_text_chars() {
        // 普通可见字符原样过: C0 41 42 43 C0 → "ABC"
        let mut out = [0u8; 16];
        let m = decode_all(&[SLIP_END, b'A', b'B', b'C', SLIP_END], &mut out).unwrap();
        assert_eq!(&out[..m], b"ABC");
    }
}
