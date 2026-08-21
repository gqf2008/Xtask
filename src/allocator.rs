//! 内存分配器
//! TODO 对不连续的RAM设备块优化

use crate::port::{Portable, Porting};
use bare_metal::Mutex;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};
use linked_list_allocator::Heap;
use linked_list_allocator::LockedHeap;

// 测试构建链到 std，由 std 提供全局分配器，不能再定义 global_allocator
#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: XTaskSpinAlloc = XTaskSpinAlloc::empty();

// 测试构建下的占位：global_allocator 由 std 提供，这里仅保留符号供 used()/free() 调用
#[cfg(test)]
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
