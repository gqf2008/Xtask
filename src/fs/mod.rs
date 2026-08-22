//! 文件系统（第 21 章）
//!
//! 分层：`drv::BdDevice`（块设备，读/写扇区）→ [`block::FatAdapter`]
//! （翻译成 fatfs 的字节流 + 游标）→ fatfs（FAT 语义）。本模块只做翻译
//! 与取舍，不分叉文件系统实现——FAT 的正确性由 fatfs 库承担，教学点在于
//! **接口形状**：文件系统要的是什么、块设备给了什么、中间差什么。
//!
//! fatfs 0.4 是 no_std + alloc 可用的纯 Rust 实现（自带 `Read`/`Write`/`Seek`
//! trait，`FsOptions::new()` 默认 `NullTimeProvider`——无 chrono 依赖），
//! 仓库 `fs` feature 已把 `["unicode","alloc","lfn"]` 接好。

pub mod block;

pub use fatfs::*;
pub use block::FatAdapter;
