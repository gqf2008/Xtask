//! `critical-section` 1.x 的 RISC-V 裸机实现——本仓 RISC-V 侧**唯一一份**。
//!
//! # 谁需要它
//!
//! 凡是 riscv32 目标都要提供,门控见 `src/arch/riscv/mod.rs`。原因:`atomic-polyfill`
//! 的 `build.rs` 表是 **(≤32 位/指针档, u64/i64 档) 二元组**:
//!
//! ```text
//! ("riscv32imac-*", (Native, Polyfill))   // 有 A:仅 ≤32 位/指针走原生
//! ("riscv32*",      (Polyfill, Polyfill)) // 无 A:两档都走 polyfill
//! ```
//!
//! 即 `AtomicU64`/`AtomicI64` 在任何 riscv32 档位上都经 `critical-section`;
//! ≤32 位原子在有 A 的口上不需要它,在无 A 的口上需要。缺这份实现的症状是
//! **编译过、链接报** `undefined symbol: _critical_section_1_0_acquire/release`
//! (2026-09-19 在 `riscv32imc-unknown-none-elf` 全量构建、以及
//! `riscv32imac-unknown-none-elf` 的 `AtomicU64` 探针上各实测复现一次)。
//! esp32c3 另有理由:其 PAC 自身引用 `critical-section`(ARM 侧不走这里,
//! 由 `cortex-m` 的 `critical-section-single-core` feature 提供)。
//!
//! # 语义
//!
//! 存 `mstatus.MIE` → 关中断 → 临界区 → 按旧值恢复,与 `Porting::free` 同款;
//! 嵌套天然正确(内层 acquire 存到的是"已关",释放不会提前开中断)。
//! **单核假设**:多核口必须换成自旋锁版本,只关本核中断挡不住别核。

/// 进临界区:返回"进入前中断是否开着"
#[no_mangle]
unsafe extern "C" fn _critical_section_1_0_acquire() -> bool {
    let was_enabled = riscv::register::mstatus::read().mie();
    riscv::interrupt::disable();
    was_enabled
}

/// 出临界区:按进入前的状态恢复
#[no_mangle]
unsafe extern "C" fn _critical_section_1_0_release(was_enabled: bool) {
    if was_enabled {
        riscv::interrupt::enable();
    }
}
