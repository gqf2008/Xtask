//! 内存分配器
//! TODO 对不连续的RAM设备块优化

use crate::port::{Portable, Porting};
use bare_metal::Mutex;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};
use linked_list_allocator::Heap;
use linked_list_allocator::LockedHeap;

// 测试构建链到 std，由 std 提供全局分配器，不能再注册 global_allocator；
// 用 cfg_attr 保持单一声明：非测试构建注册为全局分配器，测试构建仅保留符号供 used()/free() 调用
#[cfg_attr(not(test), global_allocator)]
static ALLOCATOR: XTaskSpinAlloc = XTaskSpinAlloc::empty();

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
    heap: Mutex<RefCell<Heap>>,
}

impl XTaskAllocer {
    pub const fn empty() -> Self {
        Self {
            heap: Mutex::new(RefCell::new(Heap::empty())),
        }
    }

    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        Porting::free(|cs| {
            self.heap
                .borrow(*cs)
                .borrow_mut()
                .init(start_addr as *mut u8, size)
        });
    }

    pub fn used(&self) -> usize {
        Porting::free(|cs| self.heap.borrow(*cs).borrow().used())
    }

    pub fn free(&self) -> usize {
        Porting::free(|cs| self.heap.borrow(*cs).borrow().free())
    }
}

unsafe impl GlobalAlloc for XTaskAllocer {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Porting::free(|cs| {
            self.heap
                .borrow(*cs)
                .borrow_mut()
                .allocate_first_fit(layout)
                .ok()
                .map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        Porting::free(|cs| {
            self.heap
                .borrow(*cs)
                .borrow_mut()
                .deallocate(NonNull::new_unchecked(ptr), layout)
        });
    }
}

pub struct XTaskSpinAlloc {
    heap: LockedHeap,
}

impl XTaskSpinAlloc {
    pub const fn empty() -> Self {
        Self {
            heap: LockedHeap::empty(),
        }
    }

    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        self.heap.lock().init(start_addr as *mut u8, size);
    }

    pub fn used(&self) -> usize {
        self.heap.lock().used()
    }

    pub fn free(&self) -> usize {
        self.heap.lock().free()
    }
}

unsafe impl GlobalAlloc for XTaskSpinAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.heap.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.heap.dealloc(ptr, layout)
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
