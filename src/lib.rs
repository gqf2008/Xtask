#![no_std]
#![feature(strict_provenance)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(const_mut_refs)]
#![feature(const_intrinsic_copy)]
#![feature(alloc_error_handler)]
#![feature(ptr_internals)]
#![feature(const_btree_new)]

extern crate alloc;

mod allocator;
pub mod arch;
pub mod bsp;
pub mod bus;
pub mod chip;
pub mod io;
pub mod port;
pub mod prelude;
pub mod sync;
pub mod task;
pub mod time;
pub mod timer;

pub use prelude::*;

// 内存不足执行此处代码(调试用)
#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    panic!("memory out");
}
