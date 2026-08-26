//! 内存分配器
//! TODO 对不连续的RAM设备块优化
//!
//! 全局分配器 = XTaskAllocer(本内核临界区保护)。原默认 XTaskSpinAlloc
//! (linked_list_allocator 的自旋锁版)在 thumbv6m(rp2040/M0+)上编不过——
//! M0+ 无原子 CAS,spinning_top 依赖 CAS;临界区版在单核上语义等价且
//! 与内核的 free() 纪律同源。XTaskSpinAlloc 类型保留(有 CAS 的目标可用)。
//!
//! 后端引擎二选一(书稿第 11 章 vs 第 28 章):
//! - 默认 `Heap`(linked_list_allocator):first-fit 空闲链表,O(n) 分配;
//! - `tlsf` feature:换用本仓库手写的迷你 TLSF(`allocator::tlsf::MiniTlsf`),
//!   O(1) 分桶分配——同一条临界区包装,接口不变,对内核透明。

use bare_metal::Mutex;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};

/// 迷你 TLSF 引擎(书稿第 28 章):O(1) 分桶分配器,与 first-fit 对照教学
pub mod tlsf;

#[cfg(not(feature = "tlsf"))]
use linked_list_allocator::Heap as Engine;
#[cfg(feature = "tlsf")]
use tlsf::MiniTlsf as Engine;

/// 第 28 章对照引擎:first-fit 空闲链表的薄包装(与 [`tlsf::MiniTlsf`]
/// 同一接口形状,执行级实验 A/B 两引擎直驱;全局分配器的后端选择不受影响)
pub struct FirstFit {
    heap: linked_list_allocator::Heap,
}

impl FirstFit {
    pub const fn empty() -> Self {
        Self {
            heap: linked_list_allocator::Heap::empty(),
        }
    }
    /// 安全契约同 MiniTlsf:区域独占,调用方提供并发保护
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.heap.init(start as *mut u8, size);
    }
    pub unsafe fn alloc(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        self.heap.allocate_first_fit(layout).ok()
    }
    pub unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        self.heap.deallocate(ptr, layout);
    }
    pub fn used(&self) -> usize {
        self.heap.used()
    }
    pub fn free(&self) -> usize {
        self.heap.free()
    }
}

// 测试构建链到 std，由 std 提供全局分配器，不能再注册 global_allocator；
// 用 cfg_attr 保持单一声明：非测试构建注册为全局分配器，测试构建仅保留符号供 used()/free() 调用
#[cfg_attr(not(test), global_allocator)]
static ALLOCATOR: XTaskAllocer = XTaskAllocer::empty();

pub fn init(start_addr: usize, size: usize) {
    unsafe {
        ALLOCATOR.init(start_addr, size);
    }
}

pub fn used() -> usize {
    ALLOCATOR.used()
}

pub fn free() -> usize {
    ALLOCATOR.free()
}

pub struct XTaskAllocer {
    heap: Mutex<RefCell<Engine>>,
}

impl XTaskAllocer {
    pub const fn empty() -> Self {
        Self {
            heap: Mutex::new(RefCell::new(Engine::empty())),
        }
    }

    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        crate::sync::free(|cs| {
            let mut e = self.heap.borrow(*cs).borrow_mut();
            #[cfg(not(feature = "tlsf"))]
            e.init(start_addr as *mut u8, size);
            #[cfg(feature = "tlsf")]
            e.init(start_addr, size);
        });
    }

    pub fn used(&self) -> usize {
        crate::sync::free(|cs| self.heap.borrow(*cs).borrow().used())
    }

    pub fn free(&self) -> usize {
        crate::sync::free(|cs| self.heap.borrow(*cs).borrow().free())
    }
}

unsafe impl GlobalAlloc for XTaskAllocer {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        crate::sync::free(|cs| {
            let mut e = self.heap.borrow(*cs).borrow_mut();
            #[cfg(not(feature = "tlsf"))]
            let r = e.allocate_first_fit(layout).ok();
            #[cfg(feature = "tlsf")]
            let r = e.alloc(layout);
            r.map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        crate::sync::free(|cs| {
            let mut e = self.heap.borrow(*cs).borrow_mut();
            let nn = NonNull::new_unchecked(ptr);
            #[cfg(not(feature = "tlsf"))]
            e.deallocate(nn, layout);
            #[cfg(feature = "tlsf")]
            e.dealloc(nn, layout);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::XTaskAllocer;
    use core::alloc::{GlobalAlloc, Layout};

    /// 回归：空闲链表的分配/释放/相邻合并（coalesce）。
    /// 在一块静态后备缓冲区上初始化临界区版分配器：
    /// 分配两块再全部释放后 `used()` 应归零；释放相邻块合并后，
    /// 应能再分配一块接近初始大小的整块（验证 first-fit + 合并）。
    /// 阳性对照：若 dealloc 不回收，`used()==0` 立即变红；
    /// 若相邻块不合并，最后的大块分配会返回空指针。
    #[test]
    fn alloc_free_and_coalesce() {
        // 4KiB 静态后备缓冲区（host 上就是普通内存），分配器在其上记账
        static mut BACKING: [u8; 4096] = [0; 4096];
        // addr_of_mut! 取裸指针，避免产生 static mut 的可变引用
        let base = unsafe { core::ptr::addr_of_mut!(BACKING) as *mut u8 as usize };
        // 堆起始地址对齐到 8 字节
        let start = (base + 7) & !7;
        let heap = XTaskAllocer::empty();
        unsafe { heap.init(start, 2048) };
        assert!(heap.free() > 0, "初始化后应有大块空闲");

        let lay = Layout::from_size_align(64, 8).unwrap();
        let p1 = unsafe { heap.alloc(lay) };
        let p2 = unsafe { heap.alloc(lay) };
        assert!(!p1.is_null() && !p2.is_null(), "两次分配都应成功");
        assert!(heap.used() >= 128, "分配后水位应上涨");

        unsafe {
            heap.dealloc(p1, lay);
            heap.dealloc(p2, lay);
        }
        assert_eq!(heap.used(), 0, "全部释放后 used 应归零（验证 dealloc 回收）");

        // 相邻块合并：释放后应能再分配一块接近初始大小的整块
        let big = Layout::from_size_align(1024, 8).unwrap();
        let pb = unsafe { heap.alloc(big) };
        assert!(!pb.is_null(), "释放合并后应能再分配大块（验证 coalesce）");
        unsafe { heap.dealloc(pb, big) };
    }
}
