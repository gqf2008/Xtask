#![no_std]
#![no_main]

extern crate alloc;

// QEMU SMP 双核执行验证——`-smp 2` 下从核真正参与调度的执行级证据。
// 应用显式 `xtask::smp::enable()` 后,hart1 才从 _mp_hook 停泊中被放行
// (qemu_kernel_tests 不开启,保持单核语义逐字不变——次序断言仍成立)。
//
// 验证项(考官全程阻塞等令牌;每个阶段只有**最后完成者**推令牌——
// 中途唤醒考官会把在跑的被测任务顶回就绪队列造成合法迁移,掩盖放置断言):
//   1. core_count     —— 开启后核数 = 2(从核已在 _mp_hook 登记)
//   2. parallel_exec  —— 两个自旋任务分别落在不同核(位图断言);
//                       hart1 能拿到任务本身就是跨核 IPI 的直接证据
//   3. wake_latency   —— 一核 post 唤醒阻塞任务,延迟有界
//                       (waker 的 sleep 到期由主核 tick 经 IPI 投回其所在核)
//   4. lock_stress    —— 8 任务 × 1000 次 Mutex 递增,总数必须精确
//   5. heap_stress    —— 4 任务并发分配/释放,分配器跨核自旋不坏
//   6. tick_alive     —— 双核都忙时主核节拍照常推进
//
// 运行:qemu-system-riscv32 -M virt -smp 2 -nographic -bios none -kernel \
//       target/riscv32imac-unknown-none-elf/release/examples/qemu_smp

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

use xtask::arch::riscv::rt;
use xtask::chip::qemu_riscv::stdout::{qemu_exit_fail, qemu_exit_pass, write_str};
use xtask::port::{Portable, Porting};
use xtask::prelude::*;
use xtask::sprintln;
use xtask::sync::mutex::Mutex;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    write_str("\r\n!!! PANIC at ");
    if let Some(l) = info.location() {
        let full = format!("{}:{}:{}", l.file(), l.line(), l.column());
        for b in full.bytes() {
            xtask::chip::qemu_riscv::stdout::putc(b);
        }
    }
    write_str("\r\n");
    qemu_exit_fail();
}

static FAILED: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn fail(msg: String) {
    FAILED.lock().push(msg);
}

fn check(cond: bool, name: &str, detail: String) {
    if !cond {
        fail(format!("{name}: {detail}"));
    }
}

/// 完成令牌通道:考官阻塞 pop 唯一令牌(由该阶段最后完成者推)
static DONE: Mutex<Option<Queue<u32>>> = Mutex::new(None);
/// 阶段完成计数(考官在每个阶段开始时清零)
static PHASE_DONE: AtomicUsize = AtomicUsize::new(0);

/// 被测任务完成打卡:只有**最后**一个完成者推令牌唤醒考官——
/// 中途不唤醒,考官就不会在阶段内抢占/迁移任何被测任务
fn finish(total: usize) {
    if PHASE_DONE.fetch_add(1, Ordering::AcqRel) + 1 == total {
        DONE.lock().as_ref().unwrap().push_back(1);
    }
}

fn begin_phase() {
    PHASE_DONE.store(0, Ordering::Release);
}

/// 阻塞等本阶段完成
fn wait_phase() {
    let q = DONE.lock().as_ref().unwrap().clone();
    loop {
        if q.pop_front().is_some() {
            break;
        }
    }
}

/// 自旋 ~30 tick,记录见过的核;结束打卡
fn spin_task(seen: &'static AtomicUsize, total: usize) {
    let t0 = xtask::tick();
    while xtask::tick() - t0 < 30 {
        seen.fetch_or(1 << Porting::hart_id(), Ordering::Relaxed);
    }
    finish(total);
}

/// 自旋 ~20 tick 记录核位(自旋不阻塞,不迁移)
fn spin_seen(seen: &AtomicUsize) {
    seen.store(0, Ordering::Release);
    let t0 = xtask::tick();
    while xtask::tick() - t0 < 20 {
        seen.fetch_or(1 << Porting::hart_id(), Ordering::Relaxed);
    }
}

#[rt::entry]
fn main() -> ! {
    extern "C" {
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init_heap(start_addr, 1 * 1024 * 1024);
    xtask::init_logger();
    write_str("qemu_smp: suite starting\r\n");

    assert!(Porting::hart_id() == 0, "只有 hart0 应进入 main");
    // 开启 SMP:start() 时放行 hart1 参与调度(必须在 start 之前)
    xtask::smp::enable();

    TaskBuilder::new()
        .name("examiner")
        .priority(1)
        .stack_size(4096)
        .spawn(|| {
            *DONE.lock() = Some(Queue::with_capacity(16));

            // ---- 1. 核数(同一 ELF 支持 -smp 2..8;门禁跑 -smp 2)----
            let n = Porting::core_count();
            sprintln!("harts online: {n}");
            check(n >= 2, "core_count", format!("core_count={n}(应 ≥2)"));

            // ---- 2. 双核并行:两个自旋任务应落在不同核 ----
            static SEEN_A: AtomicUsize = AtomicUsize::new(0);
            static SEEN_B: AtomicUsize = AtomicUsize::new(0);
            SEEN_A.store(0, Ordering::Release);
            SEEN_B.store(0, Ordering::Release);
            begin_phase();
            TaskBuilder::new()
                .name("smp.a")
                .priority(8)
                .spawn(|| spin_task(&SEEN_A, 2));
            TaskBuilder::new()
                .name("smp.b")
                .priority(8)
                .spawn(|| spin_task(&SEEN_B, 2));
            wait_phase();
            let (sa, sb) = (
                SEEN_A.load(Ordering::Acquire),
                SEEN_B.load(Ordering::Acquire),
            );
            check(
                sa.count_ones() == 1 && sb.count_ones() == 1 && sa != sb,
                "parallel_exec",
                format!("A 见过核 {sa:#b} B 见过核 {sb:#b}(应各一核且不同)"),
            );
            sprintln!(
                "test parallel_exec ... done (A on hart {}, B on hart {})",
                sa.trailing_zeros(),
                sb.trailing_zeros()
            );

            // ---- 3. 唤醒:一核 post,阻塞任务在有界延迟内醒来 ----
            static WAKE_TICK: AtomicUsize = AtomicUsize::new(0);
            static POST_TICK: AtomicUsize = AtomicUsize::new(0);
            let sem = Semaphore::new();
            let s_wait = sem.clone();
            let s_post = sem.clone();
            begin_phase();
            TaskBuilder::new()
                .name("smp.waker")
                .priority(8)
                .spawn(move || {
                    spin_seen(&SEEN_A);
                    xtask::sleep_ms(20); // 让 waiter 先阻塞;到期由主核 tick 经 IPI 投回
                    POST_TICK.store(xtask::tick() as usize, Ordering::Release);
                    s_post.post();
                    finish(2);
                });
            TaskBuilder::new()
                .name("smp.waiter")
                .priority(8)
                .spawn(move || {
                    spin_seen(&SEEN_B);
                    s_wait.wait();
                    WAKE_TICK.store(xtask::tick() as usize, Ordering::Release);
                    finish(2);
                });
            wait_phase();
            let latency = WAKE_TICK.load(Ordering::Acquire) - POST_TICK.load(Ordering::Acquire);
            check(
                latency < 30,
                "wake_latency",
                format!("唤醒延迟 {latency} tick(应 <30)"),
            );
            sprintln!("test wake_latency ... done (latency {latency} ticks)");

            // ---- 4. 锁压力:8 任务 × 1000 递增,跨核 Mutex/自旋锁 ----
            static COUNTER: Mutex<u64> = Mutex::new(0);
            *COUNTER.lock() = 0;
            begin_phase();
            for _ in 0..8 {
                TaskBuilder::new().name("smp.stress").priority(7).spawn(|| {
                    for _ in 0..1000 {
                        *COUNTER.lock() += 1;
                    }
                    finish(8);
                });
            }
            wait_phase();
            let total = *COUNTER.lock();
            check(
                total == 8000,
                "lock_stress",
                format!("总数 {total}(应 8000)"),
            );
            sprintln!("test lock_stress ... done (total {total})");

            // ---- 5. 堆压力:4 任务并发分配/释放 ----
            begin_phase();
            for _ in 0..4 {
                TaskBuilder::new().name("smp.heap").priority(7).spawn(|| {
                    for i in 0..200u32 {
                        let mut v: Vec<u8> = Vec::with_capacity(256);
                        for j in 0..256u32 {
                            v.push((i ^ j) as u8);
                        }
                        core::hint::black_box(&v);
                    }
                    finish(4);
                });
            }
            wait_phase();
            sprintln!("test heap_stress ... done");

            // ---- 6. 双核皆忙时节拍推进(主核独占 tick)----
            let t0 = xtask::tick();
            begin_phase();
            TaskBuilder::new()
                .name("smp.spin2")
                .priority(8)
                .spawn(|| spin_task(&SEEN_A, 1));
            xtask::sleep_ms(100);
            let dt = xtask::tick() - t0;
            wait_phase();
            check(
                (90..300).contains(&dt),
                "tick_alive",
                format!("100ms 实测 {dt} tick"),
            );
            sprintln!("test tick_alive ... done ({dt} ticks)");

            // ---- 汇总 ----
            let fails = FAILED.lock().clone();
            if fails.is_empty() {
                sprintln!("smp PASS: 6/6");
                qemu_exit_pass();
            } else {
                for m in fails.iter() {
                    sprintln!("FAILED: {m}");
                }
                sprintln!("smp FAIL: {}/6", 6 - fails.len());
                qemu_exit_fail();
            }
        });

    xtask::start()
}
