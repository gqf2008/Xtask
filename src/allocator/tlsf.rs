//! 迷你 TLSF(Two-Level Segregated Fit)——书稿第 27 章的手写引擎。
//!
//! 与 `linked_list_allocator`(第 11 章的 first-fit 空闲链表)的对照:
//!
//! - **first-fit**:所有空闲块串成一条链,分配从链头走,走到第一个够大的
//!   为止——分配时间是 O(空闲块数),碎片越多越慢,**最坏情况无界**。
//! - **TLSF**:空闲块按尺寸分桶(`mapping`:一级 log2 分段 + 二级线性细分),
//!   两张位图记录哪些桶非空;分配 = 两次位扫描找到最小可用桶,取桶链表头,
//!   **与空闲块总数无关**——这就是"O(1) 分配/释放"的全部含义。
//!
//! 诚实的边界(第 27 章反复讲):
//!
//! 1. "O(1)"指**不随堆状态退化**,不是每条指令单周期:位扫描用的
//!    `leading_zeros`/`trailing_zeros` 在没有 CLZ/CTZ 指令的目标(如 RV32
//!    基础指令集)上走 compiler-rt 软件实现,但那是位宽有界的常数。
//! 2. TLSF 治的是**时间**无界与**尺寸隔离**(同类尺寸的块聚在同尺寸桶里,
//!    长期抖动下碎片显著少于 first-fit);它治不好**物理碎片**——棋盘格
//!    布局下总空闲充裕但无连续大块,任何通用分配器都分不出来(本文件
//!    测试 `checkerboard_fragmentation_is_a_shared_limit` 把这一点钉死)。
//! 3. 迷你版的声明式局限:对齐上限 = **2×usize**(覆盖 RV32 的 u64/double
//!    与 x64 的 max_align_t;超对齐请求返回 None,诚实退化不静默出错);
//!    单区域(多区域 `add_pool` 留作第 27 章练习)。
//!
//! 参考:Masmano, Ripoll, Crespo & Real, "TLSF: a New Dynamic Memory
//! Allocator for Real-Time Systems"(ECRTS 2004);完整 Rust 实现见 rlsf
//! crate(yvt/rlsf);业界落点 Zephyr 的堆。
//!
//! 并发纪律与 `Heap` 相同:引擎不自带锁,调用方必须在临界区内使用
//! (`XTaskAllocer` 的 `sync::free` 包装;测试为单线程直驱)。

use core::alloc::Layout;
use core::ptr::NonNull;

/// 分配粒度 = 2×usize 对齐:块头两个词,负载首地址与它同余——
/// RV32 上为 8(Rust 数据布局 i64:64,u64/double 对齐是 8 而非 4,
/// usize 对齐根本覆盖不了它,本引擎是要当 GlobalAlloc 的);x64 上为
/// 16(对齐 max_align_t)。尺寸低位偷 1 个 USED 标志位(ALIGN ≥ 4 保证)
const ALIGN: usize = 2 * core::mem::size_of::<usize>();
const US: usize = core::mem::size_of::<usize>();
/// 块头:尺寸|USED + prev_phys(物理前驱块地址,0 = 无前驱/首块)
const HDR: usize = 2 * US;
/// 最小块 = 块头(2 词)+ 空闲链表指针(2 词);空闲块的负载区复用为桶链节点
pub(crate) const MIN_BLOCK: usize = 4 * US;
const LOG2_MIN: usize = MIN_BLOCK.trailing_zeros() as usize;
/// 一级分段数(log2 档位);堆尺寸远小于 MIN_BLOCK × 2^32,32 档封顶
const FL_COUNT: usize = 32;
/// 二级细分位数:每个一级档内 2^SLI 个线性桶
const SLI: usize = 3;
const SL_COUNT: usize = 1 << SLI; // 8

const USED: usize = 1;

/// 读/写一口词(引擎内部一切内存访问的原语;调用方保证区域有效)
#[inline]
unsafe fn r(addr: usize) -> usize {
    (addr as *const usize).read()
}
#[inline]
unsafe fn w(addr: usize, v: usize) {
    (addr as *mut usize).write(v);
}

/// 块头字段:尺寸(去标志位)
#[inline]
unsafe fn size_of(b: usize) -> usize {
    r(b) & !USED
}
#[inline]
unsafe fn is_used(b: usize) -> bool {
    r(b) & USED != 0
}
/// 物理后继块地址 = 本块地址 + 本块尺寸(边界标记的核心:O(1) 找到邻居)
#[inline]
unsafe fn next_phys(b: usize) -> usize {
    b + size_of(b)
}
/// 空闲块的桶链节点(负载区前两词)
#[inline]
unsafe fn fnext(b: usize) -> usize {
    r(b + HDR)
}
#[inline]
unsafe fn set_fnext(b: usize, v: usize) {
    w(b + HDR, v);
}
#[inline]
unsafe fn fprev(b: usize) -> usize {
    r(b + HDR + US)
}
#[inline]
unsafe fn set_fprev(b: usize, v: usize) {
    w(b + HDR + US, v);
}

/// 尺寸 → (一级档, 二级桶):块**自己该住哪个桶**
///
/// fl = log2(size) 相对 MIN_BLOCK 的档位;sl = 尺寸在档内的线性细分。
/// 例(RV32,MIN_BLOCK=16):16→(0,0),18→(0,1),31→(0,7),32→(1,0),64→(2,0)。
pub(crate) fn mapping(size: usize) -> (usize, usize) {
    debug_assert!(size >= MIN_BLOCK, "只有不小于最小块的尺寸才有桶");
    let fl_log = (usize::BITS - 1 - size.leading_zeros()) as usize;
    // fl_log >= LOG2_MIN >= 4 > SLI,移位安全
    let fl = (fl_log - LOG2_MIN).min(FL_COUNT - 1);
    let sl = (size >> (fl_log - SLI)) & (SL_COUNT - 1);
    (fl, sl)
}

/// 尺寸 → (一级档, 二级桶):**分配搜索**用的映射
///
/// 与 [`mapping`] 的差:先把请求向上取整到本 sl 档的步长。桶是有粒度的
/// ——请求 17 落在桶 [16,18),但桶里可能躺着 16 的块,比请求小!向上取整
/// 后搜索,桶内**任何**块都 ≥ 请求,取链表头即合法——"不扫桶内"是 O(1)
/// 成立的前提(代价:至多一步长的内部碎片,有界)。取整跨桶时牺牲掉的
/// "请求自己桶里够大的尾巴",由 [`MiniTlsf::find_suitable`] 的回退扫描
/// 兜底(否则堆里有够大的块却报 OOM——good-fit 的边界,迷你版选择关上)。
fn mapping_search(size: usize) -> (usize, usize) {
    debug_assert!(size >= MIN_BLOCK);
    let fl_log = (usize::BITS - 1 - size.leading_zeros()) as usize;
    let step = 1usize << (fl_log - SLI);
    mapping(size.saturating_add(step - 1))
}

/// 迷你 TLSF 引擎:一块连续区域 + 两张位图 + 桶链表头数组。
/// 一切方法的安全契约:区域由 `init` 划定且独占;调用方提供并发保护。
pub struct MiniTlsf {
    /// 主块起始(0 = 未初始化)
    base: usize,
    /// 主块尺寸(不含末尾哨兵)
    total: usize,
    /// 全部空闲块尺寸之和(含各自块头;`used = total - free_bytes`)
    free_bytes: usize,
    /// 非空一级档位图
    fl_bitmap: u32,
    /// 每个一级档的非空二级桶位图
    sl_bitmap: [u8; FL_COUNT],
    /// 桶链表头(块地址,0 = 空桶);空闲块用负载区自存 prev/next
    heads: [usize; FL_COUNT * SL_COUNT],
}

impl MiniTlsf {
    pub const fn empty() -> Self {
        Self {
            base: 0,
            total: 0,
            free_bytes: 0,
            fl_bitmap: 0,
            sl_bitmap: [0; FL_COUNT],
            heads: [0; FL_COUNT * SL_COUNT],
        }
    }

    /// 在 `[start, start+size)` 上建堆:对齐后铺一个主空闲块 + 末尾哨兵
    /// (哨兵 = 尺寸 0 的 USED 块头,保证任何块的物理后继都存在且永不空闲,
    /// 合并逻辑不用判"已到末尾")。
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        debug_assert_eq!(self.base, 0, "MiniTlsf 只允许在 empty 上 init 一次");
        let s = start.div_ceil(ALIGN) * ALIGN;
        let e = (start + size) / ALIGN * ALIGN;
        let total = e.checked_sub(s).expect("TLSF 区域对齐后为空");
        assert!(total >= MIN_BLOCK + HDR, "TLSF 区域至少放下一个主块加哨兵");
        // 主块:尺寸 = total - HDR(末尾 HDR 字节归哨兵)
        w(s, (total - HDR) | 0); // FREE
        w(s + US, 0); // 首块无物理前驱
        let sentinel = s + (total - HDR);
        w(sentinel, 0 | USED); // 尺寸 0,USED——物理链终点
        w(sentinel + US, s);
        self.base = s;
        self.total = total - HDR;
        self.free_bytes = total - HDR;
        self.insert_free(s);
    }

    /// 分配:返回负载指针;失败(不够大 / 对齐超限)返回 None。
    pub unsafe fn alloc(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        debug_assert_ne!(self.base, 0, "init 之前 alloc");
        if layout.align() > ALIGN {
            return None; // 迷你版声明式局限:超 usize 对齐不服务(诚实退化)
        }
        let raw = (layout.size().max(1) + HDR).div_ceil(ALIGN) * ALIGN;
        let need = raw.max(MIN_BLOCK);
        let b = self.find_suitable(need);
        if b == 0 {
            return None;
        }
        let bsz = size_of(b);
        let rest = bsz - need;
        if rest >= MIN_BLOCK {
            // 劈开:前段给请求(USED),余段作为新空闲块回插;
            // 余段的物理后继(哨兵或别块)prev_phys 回指余段
            w(b, need | USED);
            let rem = b + need;
            w(rem, rest); // FREE
            w(rem + US, b);
            w(next_phys(rem) + US, rem);
            self.free_bytes -= need;
            self.insert_free(rem);
        } else {
            // 剩余不足最小块:整块给出(内部碎片,有界)
            w(b, bsz | USED);
            self.free_bytes -= bsz;
        }
        NonNull::new((b + HDR) as *mut u8)
    }

    /// 释放:`ptr` 必须是本引擎 `alloc` 的返回值。先与物理后继合并,
    /// 再与物理前驱合并(边界标记,O(1)),合完插入尺寸对应的桶。
    pub unsafe fn dealloc(&mut self, ptr: NonNull<u8>, _layout: Layout) {
        let b = ptr.as_ptr() as usize - HDR;
        debug_assert!(is_used(b), "重复释放或野指针(dealloc 非本引擎块)");
        self.free_bytes += size_of(b);
        w(b, size_of(b)); // 清 USED
        // 前向:后继空闲则吃掉(哨兵恒 USED,天然止步)
        let n = next_phys(b);
        if !is_used(n) {
            self.unlink_free(n);
            w(b, size_of(b) + size_of(n));
        }
        w(next_phys(b) + US, b); // 新物理后继的 prev_phys 回指
        // 后向:前驱空闲则并入前驱
        let p = r(b + US);
        if p != 0 && !is_used(p) {
            self.unlink_free(p);
            w(p, size_of(p) + size_of(b));
            w(next_phys(p) + US, p);
            self.insert_free(p);
        } else {
            self.insert_free(b);
        }
    }

    /// 已用字节数(含已分配块的块头)
    pub fn used(&self) -> usize {
        self.total - self.free_bytes
    }
    /// 空闲字节数(全部空闲块尺寸之和,含块头)
    pub fn free(&self) -> usize {
        self.free_bytes
    }
    /// 堆容量(主块尺寸,不含哨兵)
    pub fn capacity(&self) -> usize {
        self.total
    }

    /// 位图搜索 + 边界回退:为 `need` 找一块空闲块,摘出返回(0 = 真没有)。
    ///
    /// 主路径两次位扫描,常数步——TLSF "O(1)" 的心脏。回退路径钉死一个
    /// 教科书级边界:搜索映射向上取整可能**跨过桶界**(参考实现同此,
    /// good-fit 而非 best-fit),此时请求自己所在的桶里可能躺着"够大的
    /// 尾巴"(尺寸 ∈ [need, 桶上界) 的块)被跳过——不处理就是"堆里有
    /// 够大的块却报 OOM"。数学上被跳过的桶**恰好只有请求自己的桶**
    /// (取整步长 < 一个 sl 步长,至多跨一桶),所以回退只扫它;桶内按
    /// 尺寸档隔离,块数远小于全堆空闲块数——冷路径,有界。
    unsafe fn find_suitable(&mut self, need: usize) -> usize {
        // 主路径:从向上取整后的桶位起,找"桶内任何块都 ≥ need"的非空桶
        let (fl, sl) = mapping_search(need);
        let sl_map = self.sl_bitmap[fl] & (0xFFu8 << sl);
        let (f, s) = if sl_map != 0 {
            (fl, sl_map.trailing_zeros() as usize)
        } else {
            let fl_map = self.fl_bitmap & (!0u32 << (fl + 1));
            if fl_map == 0 {
                // 主路径全空,转入回退(下面)
                (usize::MAX, 0)
            } else {
                let f = fl_map.trailing_zeros() as usize;
                (f, self.sl_bitmap[f].trailing_zeros() as usize)
            }
        };
        if f != usize::MAX {
            let b = self.heads[f * SL_COUNT + s];
            debug_assert_ne!(b, 0, "位图与桶链不一致(引擎内部不变式)");
            self.unlink_free(b);
            return b;
        }
        // 回退:扫请求自己的桶,找第一块 ≥ need 的("够大的尾巴")
        let (fl0, sl0) = mapping(need);
        let mut b = self.heads[fl0 * SL_COUNT + sl0];
        while b != 0 && size_of(b) < need {
            b = fnext(b);
        }
        if b != 0 {
            self.unlink_free(b);
        }
        b
    }

    /// 空闲块入桶:头插,置两张位图
    unsafe fn insert_free(&mut self, b: usize) {
        let (fl, sl) = mapping(size_of(b));
        let head = self.heads[fl * SL_COUNT + sl];
        set_fnext(b, head);
        set_fprev(b, 0);
        if head != 0 {
            set_fprev(head, b);
        }
        self.heads[fl * SL_COUNT + sl] = b;
        self.fl_bitmap |= 1 << fl;
        self.sl_bitmap[fl] |= 1 << sl;
    }

    /// 空闲块出桶:双向链摘除,空桶清位图(位图永远与桶链一致)
    unsafe fn unlink_free(&mut self, b: usize) {
        let (fl, sl) = mapping(size_of(b));
        let p = fprev(b);
        let n = fnext(b);
        if n != 0 {
            set_fprev(n, p);
        }
        if p != 0 {
            set_fnext(p, n);
        } else {
            self.heads[fl * SL_COUNT + sl] = n;
            if n == 0 {
                self.sl_bitmap[fl] &= !(1 << sl);
                if self.sl_bitmap[fl] == 0 {
                    self.fl_bitmap &= !(1 << fl);
                }
            }
        }
    }

    /// 测试专用:沿物理链从 base 走到哨兵,核对 prev_phys 链完整,
    /// 返回空闲块尺寸之和(用于与 `free_bytes` 对账——引擎的总不变式)
    #[cfg(test)]
    fn walk_free_sum(&self) -> usize {
        // SAFETY: 测试直驱,区域由本测试的 init 划定,无并发
        unsafe {
            let mut b = self.base;
            let mut prev = 0usize;
            let mut sum = 0usize;
            loop {
                assert_eq!(r(b + US), prev, "prev_phys 链断裂");
                let sz = size_of(b);
                if sz == 0 {
                    assert!(is_used(b), "哨兵必须是 USED 标记");
                    break;
                }
                if !is_used(b) {
                    sum += sz;
                }
                prev = b;
                b = b + sz;
            }
            assert_eq!(b, self.base + self.total, "物理链必须恰好走到哨兵");
            sum
        }
    }

    /// 测试专用:读负载指针对应的块尺寸(重叠检查用实际块尺寸而非请求值)
    #[cfg(test)]
    fn block_size_of_payload(&self, ptr: usize) -> usize {
        unsafe { size_of(ptr - HDR) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 在一块静态后备缓冲区上建引擎(对齐预处理与生产路径相同)
    macro_rules! engine_on {
        ($backing:ident, $e:ident) => {
            let base = unsafe { core::ptr::addr_of_mut!($backing) as *mut u8 as usize };
            let start = base.div_ceil(ALIGN) * ALIGN;
            let mut $e = MiniTlsf::empty();
            unsafe { $e.init(start, $backing.len() - (start - base)) };
        };
    }

    #[test]
    fn mapping_truth_table() {
        let m = MIN_BLOCK;
        // 档内线性细分:第一档 [m, 2m) 均分 8 桶
        assert_eq!(mapping(m), (0, 0));
        assert_eq!(mapping(2 * m - 1), (0, SL_COUNT - 1));
        // 档边界:2m 升一级,sl 归零
        assert_eq!(mapping(2 * m), (1, 0));
        assert_eq!(mapping(4 * m - 1), (1, SL_COUNT - 1));
        assert_eq!(mapping(4 * m), (2, 0));
        // 性质扫描:普通映射的桶下界 <= size < 桶上界
        let mut size = m;
        while size < 8192 {
            let (fl, sl) = mapping(size);
            let fl_log = fl + LOG2_MIN;
            let low = (1usize << fl_log) + sl * (1usize << (fl_log - SLI));
            let step = 1usize << (fl_log - SLI);
            assert!(low <= size && size < low + step, "mapping 桶界错:{size}");
            size += 7;
        }
    }

    #[test]
    fn mapping_search_rounds_up_into_bin() {
        let m = MIN_BLOCK;
        // 恰在桶下界的请求不跳桶
        assert_eq!(mapping_search(m), (0, 0));
        assert_eq!(mapping_search(2 * m), (1, 0));
        // 桶中间的请求向上进下一桶:桶内任何块都 >= 请求
        let mut req = m;
        while req < 8192 {
            let (fl, sl) = mapping_search(req);
            let fl_log = fl + LOG2_MIN;
            let low = (1usize << fl_log) + sl * (1usize << (fl_log - SLI));
            assert!(low >= req, "搜索映射必须保证桶内块都够大:{req} -> {low}");
            req += 5;
        }
    }

    #[test]
    fn alloc_split_and_coalesce_roundtrip() {
        static mut BACKING: [u8; 4096] = [0; 4096];
        engine_on!(BACKING, e);
        let cap = e.capacity();
        assert_eq!(e.free(), cap);
        // 分配两块:free_bytes 下降且两指针不同
        let lay = Layout::from_size_align(64, 8).unwrap();
        let p1 = unsafe { e.alloc(lay) }.unwrap();
        let p2 = unsafe { e.alloc(lay) }.unwrap();
        assert_ne!(p1, p2);
        assert!(e.used() >= 2 * (64 + HDR));
        // 全放:合并回一整块——再分配"接近容量"的大块必须成功
        unsafe {
            e.dealloc(p1, lay);
            e.dealloc(p2, lay);
        }
        assert_eq!(e.used(), 0, "全部释放后 used 应归零");
        assert_eq!(e.walk_free_sum(), e.free());
        let big = Layout::from_size_align(cap - HDR, 8).unwrap();
        let pb = unsafe { e.alloc(big) }.expect("合并后应能再分整块");
        unsafe { e.dealloc(pb, big) };
        assert_eq!(e.used(), 0);
    }

    #[test]
    fn split_remainder_stays_allocatable() {
        static mut BACKING: [u8; 4096] = [0; 4096];
        engine_on!(BACKING, e);
        let cap = e.capacity();
        // 小块劈分后,余量必须还能分出"容量 - 已用 - 块头"级别的大块
        let small = Layout::from_size_align(32, 8).unwrap();
        let ps = unsafe { e.alloc(small) }.unwrap();
        let rest = e.free();
        let big = Layout::from_size_align(rest - HDR, 8).unwrap();
        let pb = unsafe { e.alloc(big) }.expect("劈分余量应可继续分配");
        unsafe {
            e.dealloc(pb, big);
            e.dealloc(ps, small);
        }
        assert_eq!(e.free(), cap, "全放后应合并回初始容量");
    }

    /// 棋盘格碎片:16 × 64B 申请、放掉奇数号 → 8 个不相邻的 64B 级洞。
    /// 申请 256B:两个引擎都**失败**——物理碎片是一切通用分配器的共限,
    /// TLSF 不是魔法(第 27 章的诚实底线);全放合并后双双成功。
    #[test]
    fn checkerboard_fragmentation_is_a_shared_limit() {
        use linked_list_allocator::Heap;

        static mut BACKING_A: [u8; 8192] = [0; 8192];
        static mut BACKING_B: [u8; 8192] = [0; 8192];
        // 引擎 A:第 11 章的 first-fit 空闲链表(对照组)
        let base_a = unsafe { core::ptr::addr_of_mut!(BACKING_A) as *mut u8 as usize };
        let mut ff = Heap::empty();
        unsafe { ff.init(base_a as *mut u8, BACKING_A.len()) };
        // 引擎 B:本章的迷你 TLSF
        engine_on!(BACKING_B, tlsf);

        let lay64 = Layout::from_size_align(64, 8).unwrap();
        let lay256 = Layout::from_size_align(256, 8).unwrap();
        let mut ff_ptrs = [core::ptr::null_mut::<u8>(); 16];
        let mut tf_ptrs = [None; 16];
        for i in 0..16 {
            ff_ptrs[i] = ff.allocate_first_fit(lay64).unwrap().as_ptr();
            tf_ptrs[i] = unsafe { tlsf.alloc(lay64) };
        }
        // 填隙块:把棋盘区之外的余量占掉——否则连续余量会乐呵呵地把
        // 256 分出去,碎片根本没造成(本测试第一版就漏了它,假绿)
        let ff_fill = Layout::from_size_align(ff.free(), 8).unwrap();
        let tf_fill = Layout::from_size_align(tlsf.free() - HDR, 8).unwrap();
        let _ff_filler = ff.allocate_first_fit(ff_fill).unwrap();
        let _tf_filler = unsafe { tlsf.alloc(tf_fill) }.expect("余量填隙必须成功");
        // 放掉奇数号:8 个互不相邻的洞
        for i in (1..16).step_by(2) {
            unsafe { ff.deallocate(NonNull::new(ff_ptrs[i]).unwrap(), lay64) };
            unsafe { tlsf.dealloc(tf_ptrs[i].unwrap(), lay64) };
            ff_ptrs[i] = core::ptr::null_mut();
            tf_ptrs[i] = None;
        }
        // 总空闲远超 256,但没有连续 256——两个引擎都必须诚实失败
        assert!(ff.allocate_first_fit(lay256).is_err(), "first-fit 应败于物理碎片");
        assert!(unsafe { tlsf.alloc(lay256) }.is_none(), "TLSF 同样败于物理碎片(共限)");
        // 放掉偶数号:合并回大块,双双成功
        for i in (0..16).step_by(2) {
            unsafe { ff.deallocate(NonNull::new(ff_ptrs[i]).unwrap(), lay64) };
            unsafe { tlsf.dealloc(tf_ptrs[i].unwrap(), lay64) };
        }
        assert!(ff.allocate_first_fit(lay256).is_ok(), "合并后 first-fit 应能分出");
        assert!(unsafe { tlsf.alloc(lay256) }.is_some(), "合并后 TLSF 应能分出");
    }

    /// 差分模型测试:固定种子 LCG 随机申请/释放,影子账本逐操作核对——
    /// ① 物理链走账与 free_bytes 一致(walk_free_sum);
    /// ② used + free == capacity;
    /// ③ 活块两两不重叠(用实际块尺寸)。
    #[test]
    fn random_workload_invariants() {
        static mut BACKING: [u8; 16384] = [0; 16384];
        engine_on!(BACKING, e);
        let mut live: Vec<(usize, usize)> = Vec::new(); // (payload 地址, 请求尺寸)
        let mut rng: u64 = 0x9E3779B97F4A7C15;
        let mut next = move || {
            rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (rng >> 33) as usize
        };
        for _ in 0..6000 {
            if live.len() < 32 && next() % 100 < 60 {
                let sz = next() % 300 + 1;
                let lay = Layout::from_size_align(sz, 8).unwrap();
                if let Some(p) = unsafe { e.alloc(lay) } {
                    let pa = p.as_ptr() as usize;
                    let bsz = e.block_size_of_payload(pa);
                    for &(q, qs) in &live {
                        let qb = q;
                        let qe = q + e.block_size_of_payload(qb);
                        assert!(
                            pa + bsz <= qb || qe <= pa,
                            "活块重叠:{pa:#x}+{bsz} vs {qb:#x}+{qs}"
                        );
                    }
                    live.push((pa, sz));
                }
            } else if !live.is_empty() {
                let i = next() % live.len();
                let (pa, sz) = live.swap_remove(i);
                let lay = Layout::from_size_align(sz, 8).unwrap();
                unsafe { e.dealloc(NonNull::new(pa as *mut u8).unwrap(), lay) };
            }
            assert_eq!(e.walk_free_sum(), e.free(), "物理链与账本对不上");
            assert_eq!(e.used() + e.free(), e.capacity(), "used+free 必须等于容量");
        }
    }

    /// 边界跳桶回归:need 严格落在一级档上界内侧时,向上取整跨桶,
    /// 位图搜索从更高档开始必然落空;空闲块(躺在请求自己的桶里、
    /// 尺寸 ≥ need)必须被回退扫描捞到。
    /// 阳性对照:本测试先红(修复前 alloc 返回 None、整堆明明空着)
    /// 后绿——迷你版选择关上 good-fit 的这扇窗。
    #[test]
    fn boundary_bin_tail_is_not_stranded() {
        static mut BACKING: [u8; 8192] = [0; 8192];
        engine_on!(BACKING, e);
        let cap = e.capacity();
        // 情形一:整堆申请(need == cap,空闲块就是 cap 自身)
        let big = Layout::from_size_align(cap - HDR, 8).unwrap();
        let p = unsafe { e.alloc(big) }.expect("整堆申请必须成功(回退捞尾巴)");
        unsafe { e.dealloc(p, big) };
        assert_eq!(e.used(), 0);
        // 情形二:先切一刀,让余量块严格落在某档上界内侧;再申请
        // "同桶但大于桶下界"的尺寸——无回退时必败
        let a = Layout::from_size_align(4100, 8).unwrap();
        let pa = unsafe { e.alloc(a) }.unwrap();
        let rest = e.free(); // 余量块尺寸(含头),严格在某档上界内侧
        let b = Layout::from_size_align(rest - HDR, 8).unwrap();
        let pb = unsafe { e.alloc(b) }.expect("桶内尾巴必须被捞到");
        unsafe {
            e.dealloc(pb, b);
            e.dealloc(pa, a);
        }
        assert_eq!(e.used(), 0);
        assert_eq!(e.walk_free_sum(), e.free());
    }

    #[test]
    fn min_block_zst_and_align_limit() {
        static mut BACKING: [u8; 2048] = [0; 2048];
        engine_on!(BACKING, e);
        // 零尺寸与超小尺寸:按最小块服务,不崩不漏
        let z = Layout::from_size_align(0, 1).unwrap();
        let pz = unsafe { e.alloc(z) }.expect("零尺寸请求按最小块服务");
        unsafe { e.dealloc(pz, z) };
        assert_eq!(e.used(), 0);
        // 对齐上限:usize 对齐以内正常,超对齐诚实返回 None
        let ok = Layout::from_size_align(24, ALIGN).unwrap();
        assert!(unsafe { e.alloc(ok) }.is_some());
        let over = Layout::from_size_align(24, 2 * ALIGN).unwrap();
        assert!(unsafe { e.alloc(over) }.is_none(), "超对齐必须诚实退化");
    }
}
