#![no_std]
#![no_main]

extern crate alloc;

// QEMU 内核全功能自测套件——在真实上下文切换 + 真实时钟节拍下逐项验证
// 全部内核机制(宿主回归跑在 HostPorting 上:无调度器、systick 恒 0,
// 所有阻塞路径从未执行过——这正是本套件的存在理由)。
//
// 每项:考官(本任务,prio 1 最高)spawn 子任务 → 子任务末尾 post 完成信号量
// → 考官 wait + 确定性断言 → `test <name> ... ok`。
// 全过:`N/N passed` → SiFive exit 0;任一失败:`FAILED:` → exit 非 0。
//
// 运行:qemu-system-riscv32 -M virt -nographic -bios none -kernel \
//       target/riscv32imac-unknown-none-elf/release/examples/qemu_kernel_tests

use alloc::format;
use alloc::string::String;
use alloc::sync::Arc as XArc;
use alloc::vec::Vec;

use xtask::arch::riscv::rt;
use xtask::chip::qemu_riscv::stdout::{qemu_exit_fail, qemu_exit_pass, write_str};
use xtask::prelude::*;
use xtask::sync::mutex::Mutex;
use xtask::sprintln;

// 覆盖 panic 处理,把 panic 位置打到串口——panic 可见而非静默复位,
// 这本身就是执行级验证的一部分(定位过程见书稿/提交信息)
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    write_str("\r\n!!! PANIC at ");
    if let Some(l) = info.location() {
        // file:line:col —— 用 format! 一次(轻量;死循环前可用堆)
        let full = format!("{}:{}:{}", l.file(), l.line(), l.column());
        for b in full.bytes() {
            xtask::chip::qemu_riscv::stdout::putc(b);
        }
    }
    write_str("\r\n");
    loop {
        core::hint::spin_loop();
    }
}

/// 事件序列设施:子任务 push (事件名, tick),考官校验
static EVENTS: Mutex<Option<Vec<(&'static str, u64)>>> = Mutex::new(None);

fn ev(name: &'static str) {
    EVENTS.lock().as_mut().unwrap().push((name, xtask::tick()));
}

fn events() -> Vec<(&'static str, u64)> {
    EVENTS.lock().as_mut().unwrap().clone()
}

fn clear_events() {
    EVENTS.lock().as_mut().unwrap().clear();
}

/// 单项测试结果收集
static FAILED: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn fail(msg: String) {
    FAILED.lock().push(msg);
}

fn check(cond: bool, name: &str, detail: String) -> bool {
    if cond {
        true
    } else {
        fail(format!("{name}: {detail}"));
        false
    }
}

// ============ 测试 1:优先级抢占 ============
fn test_prio_preempt() {
    let done = XArc::new(Semaphore::new());
    let done_w = done.clone();
    clear_events();
    TaskBuilder::new().name("t1.low").priority(10).spawn(move || {
        ev("L1");
        // 低任务运行中 spawn 高任务——调度器应立即抢占
        TaskBuilder::new().name("t1.high").priority(2).spawn(move || {
            ev("H");
        });
        ev("L2"); // 高任务结束后才能继续
        done_w.post();
    });
    done.wait();
    let seq: Vec<&str> = events().iter().map(|(n, _)| *n).collect();
    check(
        seq == ["L1", "H", "L2"],
        "prio_preempt",
        format!("序列 {seq:?}(应 L1→H→L2)"),
    );
}

// ============ 测试 2:时间片轮转 ============
fn test_time_slice() {
    static CNT: Mutex<(u64, u64)> = Mutex::new((0, 0));
    let done = XArc::new(Semaphore::new());
    let done_w = done.clone();
    TaskBuilder::new().name("t2.a").priority(8).stack_size(1024).spawn(move || {
        let t0 = xtask::tick();
        while xtask::tick() - t0 < 50 {
            CNT.lock().0 += 1;
        }
    });
    TaskBuilder::new().name("t2.b").priority(8).stack_size(1024).spawn(move || {
        let t0 = xtask::tick();
        while xtask::tick() - t0 < 50 {
            CNT.lock().1 += 1;
        }
        done_w.post();
    });
    done.wait();
    let (a, b) = *CNT.lock();
    check(a > 0 && b > 0, "time_slice", format!("a={a} b={b}(应都>0)"));
}

// ============ 测试 3:信号量阻塞/唤醒 ★ ============
fn test_sem_block() {
    let sem = Semaphore::new();
    let s2 = sem.clone();
    clear_events();
    TaskBuilder::new().name("t3.waiter").priority(5).spawn(move || {
        ev("W1");
        s2.wait(); // 应挂起
        ev("W2");
    });
    xtask::sleep_ms(20); // 让 waiter 先跑(考官 prio 1 最高,主动 sleep 让出)
    let seq: Vec<&str> = events().iter().map(|(n, _)| *n).collect();
    if !check(seq == ["W1"], "sem_block", format!("post 前 {seq:?}(应只有 W1)")) {
        return;
    }
    sem.post();
    xtask::sleep_ms(20);
    let seq2: Vec<&str> = events().iter().map(|(n, _)| *n).collect();
    check(seq2 == ["W1", "W2"], "sem_block", format!("post 后 {seq2:?}(应 W1→W2)"));
}

// ============ 测试 4:Mutex 阻塞互斥 ★ ============
fn test_mutex_blocking() {
    static LOCK: Mutex<()> = Mutex::new(());
    static WINDOWS: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());
    let done = XArc::new(Semaphore::new());
    let d2 = done.clone();
    TaskBuilder::new().name("t4.a").priority(6).spawn(move || {
        let _g = LOCK.lock();
        let t0 = xtask::tick();
        WINDOWS.lock().push((t0, t0 + 30));
        xtask::sleep_ms(30); // 持锁睡——B 必须挂起
        TaskBuilder::new().name("t4.b").priority(3).spawn(move || {
            let _g = LOCK.lock(); // 更高优先级但锁被占——挂起直到 A 释放
            let t = xtask::tick();
            WINDOWS.lock().push((t, t + 5));
            xtask::sleep_ms(5);
            d2.post();
        });
        xtask::sleep_ms(10);
    });
    done.wait();
    let w = WINDOWS.lock().clone();
    let ok = w.len() == 2 && w[0].1 <= w[1].0;
    check(ok, "mutex_blocking", format!("窗口 {w:?}(应不相交且 A 先)"));
}

// ============ 测试 5:Queue MPMC 阻塞 ★ ============
fn test_queue_mpmc() {
    const N: usize = 40;
    let q: Queue<usize> = Queue::with_capacity(8); // 小容量逼出满阻塞
    static RECV: Mutex<Vec<usize>> = Mutex::new(Vec::new());
    for p in 0..2 {
        let q2 = q.clone();
        TaskBuilder::new()
            .name(if p == 0 { "t5.p0" } else { "t5.p1" })
            .priority(7)
            .spawn(move || {
                for i in 0..N {
                    q2.push_back(i * 2 + p); // 值域 0..2N 无重复
                }
            });
    }
    for _c in 0..3 {
        let q3 = q.clone();
        TaskBuilder::new().name("t5.c").priority(7).spawn(move || loop {
            match q3.pop_front() {
                Some(usize::MAX) => break, // 哨兵:退出
                Some(v) => RECV.lock().push(v),
                None => continue,
            }
        });
    }
    xtask::sleep_ms(800); // 等 2N 条全部推完、消费完
    for _ in 0..3 {
        q.push_back(usize::MAX);
    }
    xtask::sleep_ms(100);
    let mut r = RECV.lock();
    r.sort();
    let expect: Vec<usize> = (0..2 * N).collect();
    let ok = r.len() == 2 * N && *r == expect;
    check(ok, "queue_mpmc", format!("收 {} 条(应 {} 无丢失重复)", r.len(), 2 * N));
}

// ============ 测试 6:Notifier 唤醒 ★ ============
fn test_notify() {
    let n = Notifier::new();
    let w = n.clone();
    clear_events();
    TaskBuilder::new().name("t6.waiter").priority(5).spawn(move || {
        w.wait();
        ev("NW");
    });
    xtask::sleep_ms(20);
    n.notify();
    xtask::sleep_ms(20);
    let seq: Vec<&str> = events().iter().map(|(n, _)| *n).collect();
    check(seq.contains(&"NW"), "notify", format!("{seq:?}(应含 NW)"));
}

// ============ 测试 7:Broadcast 一对多 ★ ============
fn test_broadcast() {
    static CNT: Mutex<u32> = Mutex::new(0);
    let b = Broadcast::new();
    for _ in 0..5 {
        let w = b.clone();
        TaskBuilder::new().name("t7.w").priority(6).spawn(move || {
            w.wait();
            *CNT.lock() += 1;
        });
    }
    xtask::sleep_ms(30); // 等 5 个都挂起
    b.notify();
    xtask::sleep_ms(30);
    let c = *CNT.lock();
    check(c == 5, "broadcast", format!("收到 {c}/5"));
}

// ============ 测试 8:软件定时器(真 tick 下)★ ============
fn test_timer() {
    static TICKS: Mutex<u32> = Mutex::new(0);
    static ONESHOT: Mutex<u32> = Mutex::new(0);
    {
        // 周期 20ms;句柄作用域结束 drop 即取消
        let _t = xtask::timer::Timer::period(20, || {
            *TICKS.lock() += 1;
        });
        xtask::timer::Timer::after(40, || {
            *ONESHOT.lock() += 1;
        });
        xtask::sleep_ms(100);
    }
    let ticks = *TICKS.lock();
    let once = *ONESHOT.lock();
    check(
        (3..=8).contains(&ticks) && once == 1,
        "timer_periodic",
        format!("period={ticks}(应 3..8) oneshot={once}(应 1)"),
    );
}

// ============ 测试 9:sleep 时基 ============
fn test_sleep_accuracy() {
    let t0 = xtask::tick();
    xtask::sleep_ms(100);
    let dt = xtask::tick() - t0;
    check(
        (100..200).contains(&dt),
        "sleep_accuracy",
        format!("sleep_ms(100) 实测 {dt} tick(应 [100,200))"),
    );
}

// ============ 测试 10:堆分配 ============
fn test_heap_alloc() {
    let before = xtask::used_memory();
    {
        let mut v: Vec<u8> = Vec::new();
        for i in 0..1000u32 {
            v.push((i & 0xff) as u8);
        }
        let mut keep: Vec<alloc::boxed::Box<[u8; 128]>> = Vec::new();
        for _ in 0..32 {
            keep.push(alloc::boxed::Box::new([0u8; 128]));
        }
        let _ = (keep, v);
    } // 全部 drop
    let after = xtask::used_memory();
    check(
        before <= after + 256 && after <= before + 256,
        "heap_alloc",
        format!("水位 before={before} after={after}(应回落同量级)"),
    );
}

// ============ 测试 11:消息总线 ============
fn test_bus() {
    static GOT: Mutex<u32> = Mutex::new(0);
    let bus: xtask::bus::Bus<&'static str> = xtask::bus::Bus::new();
    let tok = bus.subscribe("t", |_topic: &'static str, _e: &'static str| {
        *GOT.lock() += 1;
    });
    bus.publish("t", "e1");
    xtask::sleep_ms(10);
    bus.unsubscribe(tok);
    bus.publish("t", "e2");
    xtask::sleep_ms(10);
    let g1 = *GOT.lock();
    check(g1 == 1, "bus_pubsub", format!("收到 {g1}(应 1:订阅收 1,退订后不收)"));
}

// ============ 测试 12:任务退出回收 ★ ============
fn test_task_exit() {
    let before = xtask::used_memory();
    for i in 0..8 {
        TaskBuilder::new()
            .name("t12.churn")
            .priority(12)
            .stack_size(512)
            .spawn(move || {
                let _bag: Vec<u8> = alloc::vec![7u8; 256];
                let _ = i;
            });
    }
    xtask::sleep_ms(200); // 等全部退出
    let after = xtask::used_memory();
    check(
        after < before + 4096,
        "task_exit",
        format!("水位 before={before} after={after}(退出应回收,净增 <4KB)"),
    );
}

// ============ 测试 13:可重入锁(嵌套 + 跨任务互斥)★ ============
// host 单上下文测不到的两条:①同一任务嵌套 lock 不睡死(普通 Mutex 会);
// ②别的高优先级任务必须等到"两层都放完"才能进——host 只有一个执行身份,
// 永远可重入,跨任务互斥只能在真核上验证。
fn test_reentrant_mutex() {
    static LOCK: ReentrantMutex<u32> = ReentrantMutex::new(0);
    static SEQ2: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
    let done = XArc::new(Semaphore::new());
    let d2 = done.clone();
    SEQ2.lock().clear();
    TaskBuilder::new().name("t13.a").priority(6).spawn(move || {
        {
            let mut g1 = LOCK.lock(); // 深度 1
            SEQ2.lock().push("A1");
            *g1 += 1;
            let mut g2 = LOCK.lock(); // 嵌套:深度 2——普通 Mutex 在此永久阻塞
            SEQ2.lock().push("A2");
            *g2 += 1;
            // 持两层锁期间 spawn 更高优先级的 B——B 必须挂起直到两层都放完
            let d3 = d2.clone();
            TaskBuilder::new().name("t13.b").priority(3).spawn(move || {
                let mut g = LOCK.lock(); // 挂起,直到 A 彻底释放
                SEQ2.lock().push("B");
                *g += 1;
                d3.post();
            });
            xtask::sleep_ms(30); // 持两层锁睡——B 不能进
            drop(g2); // 深度 2→1:仍持有,B 仍不能进
            SEQ2.lock().push("A3");
            xtask::sleep_ms(20);
        } // g1 析构:深度 1→0,真正释放 → 唤醒 B(高优先级,立即抢占)
    });
    done.wait();
    let s = SEQ2.lock().clone();
    // A1<A2<A3 由程序序保证;A3 必在 B 前(A3 时 A 仍持一层,B 进不来)
    let ok = s == ["A1", "A2", "A3", "B"];
    let val = *LOCK.lock(); // 此刻锁空闲:1(A)+1(A嵌套)+1(B)=3
    check(
        ok && val == 3,
        "reentrant_mutex",
        format!("序列 {s:?}(应 A1 A2 A3 B) val={val}(应 3)"),
    );
}

// ============ 考官 ============
#[rt::entry]
fn main() -> ! {
    extern "C" {
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    // 保守 1M 堆:曾用 4M —— memory.x 两区拆分时越过 RAM 末尾并压过栈,
    // 分配器高地址分配踩坏现场直到野写 0x100000(sifive_test)整机复位
    // (详见 memory.x 注)
    xtask::init_heap(start_addr, 1 * 1024 * 1024);
    xtask::init_logger();
    write_str("qemu_kernel_tests: suite starting\r\n");
    // 双核起跑契约(-smp 2):hart1 由 riscv-rt 默认 _mp_hook 停泊(wfi),
    // 只有 hart0 进 main。若停泊失效,hart1 会并发执行到这里——
    // 堆已初始化则下面断言可能双双通过,但随后双考官并发跑套件,
    // 输出与计数必乱(check.sh 的 12/12 与 -smp 2 门禁会抓到)
    use xtask::port::{Portable, Porting};
    sprintln!("boot hart: {}", Porting::hart_id());
    assert!(Porting::hart_id() == 0, "只有 hart0 应进入 main");

    TaskBuilder::new()
        .name("examiner")
        .priority(1)
        .stack_size(4096)
        .spawn(|| {
            *EVENTS.lock() = Some(Vec::new());

            let tests: Vec<(&'static str, fn())> = alloc::vec![
                ("prio_preempt", test_prio_preempt),
                ("time_slice", test_time_slice),
                ("sem_block", test_sem_block),
                ("mutex_blocking", test_mutex_blocking),
                ("queue_mpmc", test_queue_mpmc),
                ("notify", test_notify),
                ("broadcast", test_broadcast),
                ("timer_periodic", test_timer),
                ("sleep_accuracy", test_sleep_accuracy),
                ("heap_alloc", test_heap_alloc),
                ("bus_pubsub", test_bus),
                ("task_exit", test_task_exit),
                ("reentrant_mutex", test_reentrant_mutex),
            ];
            let total = tests.len();
            let mut passed = 0usize;
            for (name, f) in tests {
                clear_events();
                f();
                if FAILED.lock().is_empty() {
                    sprintln!("test {name} ... ok");
                    passed += 1;
                } else {
                    let msgs = FAILED.lock().join("; ");
                    sprintln!("test {name} ... FAILED: {msgs}");
                    sprintln!("{passed}/{total} passed, aborting");
                    qemu_exit_fail();
                }
            }
            sprintln!("{passed}/{total} passed");
            qemu_exit_pass();
        });

    xtask::start()
}
