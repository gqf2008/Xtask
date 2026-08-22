#![cfg_attr(not(test), no_std)]
#![feature(strict_provenance)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#![feature(ptr_internals)]
#![feature(const_btree_new)]
#![feature(binary_heap_retain)]
#![feature(ptr_const_cast)]

extern crate alloc;

mod allocator;
pub mod arch;
pub mod bsp;
pub mod bus;
pub mod drv;
pub mod drv_static;
pub mod chip;
#[cfg(feature = "fs")]
pub mod fs;
pub mod fsm;
pub mod logger;
#[cfg(feature = "net")]
pub mod net;
#[cfg(feature = "usb")]
pub mod usb;
pub mod port;
pub mod prelude;
pub mod sd_proto;
pub mod sync;
pub mod task;
pub mod time;
#[cfg(feature = "timer")]
pub mod timer;

#[cfg(all(not(test), not(target_arch = "arm")))]
use panic_halt as _;
#[cfg(all(not(test), target_arch = "arm"))]
use panic_probe as _;
pub use prelude::*;

pub fn init_logger() {
    logger::init().ok();
}

pub fn init_heap(start_addr: usize, size: usize) {
    allocator::init(start_addr, size);
}

#[deprecated]
pub fn init(start_addr: usize, size: usize) {
    logger::init().ok();
    init_heap(start_addr, size);
}

// 内存不足执行此处代码(调试用)
// 测试构建链到 std，不能重复定义 alloc_error_handler（会与 std 冲突）
#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    panic!("memory out");
}
