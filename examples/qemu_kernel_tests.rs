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
use core::sync::atomic::{AtomicBool, Ordering};

use xtask::arch::riscv::rt;
use xtask::chip::qemu_riscv::stdout::{qemu_exit_fail, qemu_exit_pass, write_str};
use xtask::prelude::*;
use xtask::sync::mutex::Mutex;
use xtask::sprintln;
use core::alloc::Layout;

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

// ============ 测试 14:优先级继承(火星探路者剧场)★ ============
// 经典反转三人组:L(低 5)持锁 → M(中 3)长跑占满 CPU → H(高 2)等锁。
// 无 PI:H 被 M 无限期压制——L 优先级低于 M,永远轮不到放锁,这是 1997
// 火星探路者事故的形态;有 PI:H 阻塞的瞬间把 L 抬到 2,L 立即抢占 M
// 跑完临界区放锁,H 在 M 的长跑还没结束时就已经进去。
// 断言双侧:事件序列精确 + tick 不等式(tL2/tH 必须早于 tMdone——
// M 的长跑被 PI"切开"了;阳性对照:去掉 PI,这个不等式在 100 个 tick
// 的余量下必然翻红)。
fn test_priority_inheritance() {
    static LOCK: Mutex<()> = Mutex::new(());
    static SEQ4: Mutex<Vec<(&'static str, u64)>> = Mutex::new(Vec::new());
    let done = XArc::new(Semaphore::new());
    let d2 = done.clone();
    SEQ4.lock().clear();
    // L:拿锁 → 记录 → 把舞台交给 M(L 被 M 抢占,持锁睡在就绪队列)
    TaskBuilder::new().name("t14.low").priority(5).spawn(move || {
        let g = LOCK.lock(); // 空闲,立即持有
        SEQ4.lock().push(("L1", xtask::tick()));
        // spawn M(3 < 5):立即抢占 L。L 在此处被顶下,持着锁停在就绪队列
        TaskBuilder::new().name("t14.med").priority(3).spawn(move || {
            let t0 = xtask::tick();
            SEQ4.lock().push(("M0", t0));
            // 长跑 100 tick:PI 若失效,H 要等它跑完才轮到 L
            TaskBuilder::new().name("t14.high").priority(2).spawn(move || {
                let _h = LOCK.lock(); // 被 L 持有 → 挂起,并触发 PI 抬 L
                SEQ4.lock().push(("H", xtask::tick()));
                // guard 在此析构:释放并唤醒下一个等待者(无人)——H 退场
            });
            while xtask::tick() - t0 < 100 {
                core::hint::spin_loop();
            }
            SEQ4.lock().push(("Mdone", xtask::tick()));
            d2.post();
        });
        // PI 抬升后 L 在此恢复(仍是持有者,优先级已是 2):
        // 记录"临界区内"的 L2,然后才 drop —— L2 时刻必然仍在 M 的长跑中
        SEQ4.lock().push(("L2", xtask::tick()));
        drop(g); // 释放:醒来 H(2),立即抢占
    });
    done.wait();
    let s = SEQ4.lock().clone();
    let names: Vec<&str> = s.iter().map(|(n, _)| *n).collect();
    let get = |name: &str| s.iter().find(|(n, _)| *n == name).map(|(_, t)| *t);
    // 序列:L1 → (L 被 M 抢占) → M0 → (M spawn H → H 阻塞 → PI 抬 L →
    // L 抢占 M) → L2 → (H 被唤醒,高优先级抢先) → H → (M 恢复跑完长跑) → Mdone
    let ok_seq = names == ["L1", "M0", "L2", "H", "Mdone"];
    let ok_pi = match (get("L2"), get("H"), get("Mdone"), get("M0")) {
        (Some(l2), Some(h), Some(md), Some(m0)) => l2 < md && h < md && h < m0 + 100,
        _ => false,
    };
    check(
        ok_seq && ok_pi,
        "priority_inheritance",
        format!("序列 {names:?}(应 L1 M0 L2 H Mdone) PI 判据: L2/H 须早于 Mdone 且 H 在 M 长跑内进入"),
    );
}

// ============ 测试 15:完整 PI——多锁持有下释放不丢继承 ★ ============
// 经典实现的已知局限:持有者 T 先拿 A 再拿 B,WA(4)/WB(3) 分别阻塞在
// A/B 上(T 被继承抬到 3);T 释放 A 时若"一把锁一把锁地回落到出生值",
// 会直接掉回 6——B 上的继承暂时丢失,M(4) 就能在 T 放 B 之前插进来,
// 反转从"释放缝"钻回。完整 PI(Task 持锁集合 + 全链重算)让 T 停在 3
// (B 的队首 WB),M 只能排在 WB 之后。判别:完整实现序列
// ...RA→RL→WB→WA→M(T 一气放完 B,M 排最后);经典实现是
// ...RA→WA→WB→RL→M(WA 在 T 放 B 之前就抢进)——WA 的位置即判别点。
fn test_priority_inheritance_multi_hold() {
    static LOCK_A: Mutex<()> = Mutex::new(());
    static LOCK_B: Mutex<()> = Mutex::new(());
    static L_DONE: AtomicBool = AtomicBool::new(false);
    static SEQ: Mutex<Vec<(&'static str, u64)>> = Mutex::new(Vec::new());
    let done = XArc::new(Semaphore::new());
    let d2 = done.clone();
    let m_not = Notifier::new();
    let m_wait = m_not.clone();
    SEQ.lock().clear();
    L_DONE.store(false, Ordering::SeqCst);
    // 持有者 T:先 A 后 B;两个高位等待者分别堵在两把锁上。
    TaskBuilder::new().name("t15.holder").priority(6).spawn(move || {
        let ga = LOCK_A.lock();
        SEQ.lock().push(("TA", xtask::tick()));
        // WA(4) 抢入:WA0 → 阻塞在 A 上 → T 被抬到 4
        TaskBuilder::new().name("t15.wa").priority(4).spawn(move || {
            SEQ.lock().push(("WA0", xtask::tick()));
            let _g = LOCK_A.lock(); // 阻塞;醒后(T 放 B 之后)才拿得到
            SEQ.lock().push(("WA", xtask::tick()));
        });
        let gb = LOCK_B.lock();
        SEQ.lock().push(("TB", xtask::tick()));
        // WB(3) 抢入:WB0 → 阻塞在 B 上(T 的继承变成 min(4,3)=3)
        TaskBuilder::new().name("t15.wb").priority(3).spawn(move || {
            SEQ.lock().push(("WB0", xtask::tick()));
            let _g = LOCK_B.lock();
            SEQ.lock().push(("WB", xtask::tick()));
        });
        // M(4):等第二阶段结束的观察者——完整 PI 下它只能排在 WB 后
        TaskBuilder::new().name("t15.m").priority(4).spawn(move || {
            SEQ.lock().push(("M0", xtask::tick()));
            m_wait.wait(); // 等 T 释放 A 的通知
            while !L_DONE.load(Ordering::SeqCst) {
                xtask::sleep_ms(5); // 观察窗口:慢速轮询(经典实现会在此插队)
            }
            SEQ.lock().push(("M", xtask::tick()));
            d2.post();
        });
        xtask::sleep_ms(30); // 等 WA/WB 阻塞、M 挂到通知上
        drop(ga); // 释放 A:还剩 B——继承必须停在 3(B 的队首 WB),不许掉 6
        SEQ.lock().push(("RA", xtask::tick()));
        m_not.notify(); // 放 M 出来观察 WA 身后有多少空档
        // RL 记在释放 B 之前:T 仍被 WB 的继承托在 3,记录瞬间不可被抢;
        // 若放完才记,T 已掉回出生值 6,落在 drop 与拿 SEQ 之间的 tick 会
        // 让 WB(3) 抢先记录——序列偶发翻红,测的就不是 PI 而是 tick 运气
        SEQ.lock().push(("RL", xtask::tick()));
        drop(gb); // 释放 B:再无持锁,T 才真正回落到出生值 6
        L_DONE.store(true, Ordering::SeqCst);
    });
    done.wait();
    let s = SEQ.lock().clone();
    let names: Vec<&str> = s.iter().map(|(n, _)| *n).collect();
    check(
        names == ["TA", "WA0", "TB", "WB0", "M0", "RA", "RL", "WB", "WA", "M"],
        "priority_inheritance_multi_hold",
        format!("序列 {names:?}(应 TA WA0 TB WB0 M0 RA RL WB WA M——释放 A 后 T 仍持 B 的继承 3,M 不能插进第二个临界区)"),
    );
}

// ============ 测试 16:PCP——规则 2 拦下"空闲锁",交叉持锁不死锁 ★ ============
// 理论(书稿第 27 章):X(2)/Y(5) 都要 A、B 两把天花板锁(天花板 2),
// 顺序相反。PI 时代 Y 会先拿到空闲的 B → X 等 B、Y 等 A → 死锁;
// PCP 的规则 2 在 Y 试图拿 B 时拦住("A 被 X 持,天花板 2 <= Y 的 5"),
// **哪怕 B 是空的**——X 睡醒后自己拿 B 完成,环合不拢。
// 判别:序列 [XA, YF, XB, YB, YA]——YB 落在 XB 之后,而 YF(XB 前,
// X 只持 A 的窗口)时刻 B 确实是空的——Y 是"空锁面前被拦",铁证。
// 看门狗(16)兜底:万一实现错了(死锁),套件不至于吊死,判红退出。
fn test_pcp_ceiling_blocked() {
    static LOCK_A: Mutex<()> = Mutex::with_ceiling((), 2);
    static LOCK_B: Mutex<()> = Mutex::with_ceiling((), 2);
    static SEQ: Mutex<Vec<(&'static str, u64)>> = Mutex::new(Vec::new());
    static X_DONE: AtomicBool = AtomicBool::new(false);
    static Y_DONE: AtomicBool = AtomicBool::new(false);
    static Y_TRIED: AtomicBool = AtomicBool::new(false);
    let done = XArc::new(Semaphore::new());
    let d2 = done.clone();
    SEQ.lock().clear();
    X_DONE.store(false, Ordering::SeqCst);
    Y_DONE.store(false, Ordering::SeqCst);
    Y_TRIED.store(false, Ordering::SeqCst);
    // X(2,高):A → 等 Y 尝试信号(规则 2 把 Y 拦住的时刻,保证"X 仍持 A")→
    // B(自己持 A,规则 2 不拦自己)→ 放干净 → 完成。
    // 不用固定睡眠窗口:那依赖"Y 必然在窗口内被调度",时序脆弱
    // (调度器/负载下 Y 可能整窗未跑——测试就空转了)。
    TaskBuilder::new().name("t16.x").priority(2).spawn(move || {
        let ga = LOCK_A.lock();
        SEQ.lock().push(("XA", xtask::tick()));
        // Y(5):X 睡出让出 CPU 时它才跑(5>2 抢不了 X)——试图拿**空闲的** B
        // → 规则 2 拦(他人持 A,天花板 2 ≤ 5)
        TaskBuilder::new().name("t16.y").priority(5).spawn(move || {
            SEQ.lock().push(("YF", xtask::tick()));
            Y_TRIED.store(true, Ordering::SeqCst);
            let _gb = LOCK_B.lock(); // 天花板阻塞:锁是空的,但没资格
            SEQ.lock().push(("YB", xtask::tick()));
            let _ga = LOCK_A.lock();
            SEQ.lock().push(("YA", xtask::tick()));
            Y_DONE.store(true, Ordering::SeqCst);
        });
        while !Y_TRIED.load(Ordering::SeqCst) {
            xtask::sleep_ms(1); // 等 Y 出手;它的 B.lock 若被拦,此刻 X 必仍持 A
        }
        // 再多睡一拍:tick 若恰好落在 Y 置标志与进 LOCK_B 临界区之间,X 会
        // 抢先把 B 拿走——规则 2 从未开火,序列照样绿(假绿洞)。让出这一拍,
        // Y 必定已走到 B.lock 并被拦下,"空锁面前被拦"才是必然事件
        xtask::sleep_ms(1);
        let _gb = LOCK_B.lock();
        SEQ.lock().push(("XB", xtask::tick()));
        drop(_gb); // 先 B 后 A(逆声明序;B 释放即唤醒天花板阻塞者 Y)
        drop(ga);
        X_DONE.store(true, Ordering::SeqCst);
    });
    // 看门狗(16):100ms 后拍快照——实现错了(死锁)时套件判红而不是吊死。
    // 快照在读完之后由考官直读全局标志(死锁后无人再改写,读是安全的)。
    TaskBuilder::new().name("t16.watch").priority(16).spawn(move || {
        xtask::sleep_ms(100);
        d2.post();
    });
    done.wait();
    let s = SEQ.lock().clone();
    let names: Vec<&str> = s.iter().map(|(n, _)| *n).collect();
    check(
        names == ["XA", "YF", "XB", "YB", "YA"]
            && X_DONE.load(Ordering::SeqCst)
            && Y_DONE.load(Ordering::SeqCst),
        "pcp_ceiling_blocked",
        format!(
            "序列 {names:?}(应 XA YF XB YB YA——YB 必须落在 XB 后:Y 在空锁 B 前被规则 2 拦) \
             双完成 {:?}", (X_DONE.load(Ordering::SeqCst), Y_DONE.load(Ordering::SeqCst))
        ),
    );
}

// ============ 测试 17:阳性对照——PI 交叉持锁死锁,看门狗确认 ★ ============
// 与测试 16 **完全相同**的场景,只是两把锁换成普通(无天花板/纯 PI)互斥锁:
// PI 只抬人不断环——Y 先拿空闲 B,再等 A;X 睡醒等 B——两条边互指,
// 永久死锁。这是第 27 章"缺口一"的执行级实证:PI 管"快慢",不管"相持"。
// 判别:序列停在 [XA, YF, YB](XB/YA 永不出现)、双 done 恒假 → 死锁确认。
// 看门狗(16)150ms 后确认——两个任务都已无进展,套件继续而不是吊死。
fn test_pi_cross_acquire_deadlock() {
    static LOCK_A: Mutex<()> = Mutex::new(());
    static LOCK_B: Mutex<()> = Mutex::new(());
    static SEQ: Mutex<Vec<(&'static str, u64)>> = Mutex::new(Vec::new());
    static X_DONE: AtomicBool = AtomicBool::new(false);
    static Y_DONE: AtomicBool = AtomicBool::new(false);
    static Y_GOT_B: AtomicBool = AtomicBool::new(false);
    let done = XArc::new(Semaphore::new());
    let d2 = done.clone();
    SEQ.lock().clear();
    X_DONE.store(false, Ordering::SeqCst);
    Y_DONE.store(false, Ordering::SeqCst);
    Y_GOT_B.store(false, Ordering::SeqCst);
    // X(2,高):A → 等"Y 已持 B"的握手 → 再等 B——两步都**只靠事件顺序**
    // 不靠睡眠窗口:Y 没拿到 B 之前 X 碰都不碰 B,死锁由事实保证。
    TaskBuilder::new().name("t17.x").priority(2).spawn(move || {
        let ga = LOCK_A.lock();
        SEQ.lock().push(("XA", xtask::tick()));
        TaskBuilder::new().name("t17.y").priority(5).spawn(move || {
            SEQ.lock().push(("YF", xtask::tick()));
            let _gb = LOCK_B.lock(); // 空闲(PI 无规则 2)——立刻拿到
            SEQ.lock().push(("YB", xtask::tick()));
            Y_GOT_B.store(true, Ordering::SeqCst);
            let _ga = LOCK_A.lock(); // X 持 → 挂起;PI 只把 X 抬到 2(它已是 2)
            SEQ.lock().push(("YA", xtask::tick())); // 永不可达
            Y_DONE.store(true, Ordering::SeqCst);
        });
        while !Y_GOT_B.load(Ordering::SeqCst) {
            xtask::sleep_ms(1);
        }
        let _gb = LOCK_B.lock(); // Y 持 → 挂起。X 等 B、Y 等 A:环闭合,死锁
        SEQ.lock().push(("XB", xtask::tick())); // 永不可达
        drop(_gb);
        drop(ga);
        X_DONE.store(true, Ordering::SeqCst);
    });
    // 看门狗(16):150ms 后两个任务都该再没有进展——死锁确认(死锁时
    // 只有 watcher 与 idle 可运行,watcher 必然走到这里;考官直读标志)
    TaskBuilder::new().name("t17.watch").priority(16).spawn(move || {
        xtask::sleep_ms(150);
        d2.post();
    });
    done.wait();
    let s = SEQ.lock().clone();
    let names: Vec<&str> = s.iter().map(|(n, _)| *n).collect();
    check(
        names == ["XA", "YF", "YB"]
            && !X_DONE.load(Ordering::SeqCst)
            && !Y_DONE.load(Ordering::SeqCst),
        "pi_cross_acquire_deadlock",
        format!(
            "序列 {names:?}(应 XA YF YB——XB/YA 永不出现,双 done 恒假:PI 交叉持锁死锁确认) \
             双完成 {:?}", (X_DONE.load(Ordering::SeqCst), Y_DONE.load(Ordering::SeqCst))
        ),
    );
}

// ============ 第 28 章:迷你 TLSF(两引擎直驱,与全局分配器后端选择无关) ============

/// RISC-V cycle CSR(M 模式直读;QEMU virt 实现)——分配耗时的执行级量具
#[inline]
fn rdcycle() -> usize {
    let v: usize;
    unsafe { core::arch::asm!("rdcycle {0}", out(reg) v) };
    v
}

/// 测试 18:棋盘格物理碎片——两引擎共限(第 28 章的诚实底线执行级复现)。
/// 棋盘 + 填隙后申请 4KB:first-fit 与迷你 TLSF 都必须失败;全放合并后
/// 双双成功。顺带钉死 TLSF 的精确记账(used/free 逐字节可算)。
fn test_tlsf_fragmentation() {
    use xtask::allocator::{tlsf::MiniTlsf, FirstFit};
    static mut BACK_FF: [u8; 16384] = [0; 16384];
    static mut BACK_TF: [u8; 16384] = [0; 16384];
    let lay = Layout::from_size_align(128, 8).unwrap();
    let lay4k = Layout::from_size_align(4096, 8).unwrap();

    let mut ff = FirstFit::empty();
    let mut tf = MiniTlsf::empty();
    let (bf, bt) = unsafe {
        (
            core::ptr::addr_of_mut!(BACK_FF) as *mut u8 as usize,
            core::ptr::addr_of_mut!(BACK_TF) as *mut u8 as usize,
        )
    };
    unsafe {
        ff.init(bf, BACK_FF.len());
        tf.init(bt, BACK_TF.len());
    }
    // TLSF 精确记账:每块 = 128 + 块头 16(RV32 为 8)——按平台算
    let blk = (128 + 2 * core::mem::size_of::<usize>()).max(4 * core::mem::size_of::<usize>());
    let mut pf = [core::ptr::null_mut::<u8>(); 32];
    let mut pt = [None; 32];
    let used0 = tf.used();
    for i in 0..32 {
        pf[i] = unsafe { ff.alloc(lay) }.expect("ff 32×128 应够").as_ptr();
        pt[i] = unsafe { tf.alloc(lay) };
    }
    check(
        tf.used() - used0 == 32 * blk,
        "tlsf_fragmentation",
        format!("TLSF 记账:32 块后 used 增量 {} 应恰为 {}", tf.used() - used0, 32 * blk),
    );
    // 填隙:占掉棋盘区外的余量,碎片才成立(余量在,4KB 会被它服务)
    let ff_fill = Layout::from_size_align(ff.free() - 64, 8).unwrap();
    let tf_fill = Layout::from_size_align(tf.free() - 64, 8).unwrap();
    let _gf = unsafe { ff.alloc(ff_fill) };
    let _gt = unsafe { tf.alloc(tf_fill) };
    // 交错释放:16 个互不相邻的洞
    for i in (1..32).step_by(2) {
        unsafe {
            ff.dealloc(core::ptr::NonNull::new(pf[i]).unwrap(), lay);
            tf.dealloc(pt[i].unwrap(), lay);
        }
    }
    // 4KB:两引擎都必须诚实失败(物理碎片是共限,TLSF 不是魔法)
    let ff_big = unsafe { ff.alloc(lay4k) };
    let tf_big = unsafe { tf.alloc(lay4k) };
    // 全放:合并回连续区,双双成功
    for i in (0..32).step_by(2) {
        unsafe {
            ff.dealloc(core::ptr::NonNull::new(pf[i]).unwrap(), lay);
            tf.dealloc(pt[i].unwrap(), lay);
        }
    }
    let ff_big2 = unsafe { ff.alloc(lay4k) };
    let tf_big2 = unsafe { tf.alloc(lay4k) };
    check(
        ff_big.is_none() && tf_big.is_none() && ff_big2.is_some() && tf_big2.is_some(),
        "tlsf_fragmentation",
        format!(
            "棋盘格 4KB:两引擎应同败(共限),合并后应同胜——实际 \
             ff 败={} tf 败={} ff 胜={} tf 胜={}",
            ff_big.is_none(), tf_big.is_none(), ff_big2.is_some(), tf_big2.is_some()
        ),
    );
}

/// 测试 19:分配耗时的结构性差异——"深链行走" vs "位图直达"。
/// 布局:K 个 32B 小洞(守卫块隔开,不合并)堵在 first-fit 链前,
/// 唯一的 2KB 洞在链尾;请求 2KB:first-fit 必须走过 K 个小洞(O(链)),
/// TLSF 从小洞所在的桶位图直接跳到大洞桶(O(1))。取 8 次最小周期数
/// (滤掉 tick 落入测量窗的干扰),断言 TLSF 显著更快且方向稳定。
fn test_tlsf_alloc_determinism() {
    use xtask::allocator::{tlsf::MiniTlsf, FirstFit};
    // K=512(初版 256):深链加长把 first-fit 的 O(K) 行走与大洞
    // 位图直达的差距放大——比例对 TCG 翻译块布局/整体 LTO 重排的
    // 灵敏度随之下降(2026-08-26:口侧新增 ISR 后 LTO 重排,256 时
    // 比例从 ~3× 掉到 1.70× 区间,512 后恢复稳定 ~2.5×+)
    const K: usize = 512;
    static mut BACK_FF2: [u8; 65536] = [0; 65536];
    static mut BACK_TF2: [u8; 65536] = [0; 65536];
    let small = Layout::from_size_align(32, 8).unwrap();
    let guard = Layout::from_size_align(8, 8).unwrap();
    let big = Layout::from_size_align(2048, 8).unwrap();

    // 同一布局搭两遍(每引擎一遍),测量"请求 2KB"的耗时
    let mut build = |back: *mut u8, len: usize, is_tlsf: bool, ff: &mut FirstFit, tf: &mut MiniTlsf| {
        unsafe {
            if is_tlsf { tf.init(back as usize, len) } else { ff.init(back as usize, len) }
        }
        let mut guards = [core::ptr::null_mut::<u8>(); K];
        let mut smalls_ff = [core::ptr::null_mut::<u8>(); K];
        let mut smalls_tf: [Option<core::ptr::NonNull<u8>>; K] = [None; K];
        for i in 0..K {
            unsafe {
                if is_tlsf {
                    smalls_tf[i] = tf.alloc(small);
                    guards[i] = tf.alloc(guard).unwrap().as_ptr();
                } else {
                    smalls_ff[i] = ff.alloc(small).unwrap().as_ptr();
                    guards[i] = ff.alloc(guard).unwrap().as_ptr();
                }
            }
        }
        // 2KB 块 + 填隙(挡住它与余量合并/被余量代劳)
        unsafe {
            if is_tlsf {
                let b = tf.alloc(big).unwrap();
                let fill = Layout::from_size_align(tf.free() - 64, 8).unwrap();
                let _f = tf.alloc(fill);
                for i in 0..K { tf.dealloc(smalls_tf[i].unwrap(), small); }
                tf.dealloc(b, big); // 2KB 洞在链尾/大桶,小洞 K 个在前
            } else {
                let b = ff.alloc(big).unwrap();
                let fill = Layout::from_size_align(ff.free() - 64, 8).unwrap();
                let _f = ff.alloc(fill);
                for i in 0..K { ff.dealloc(core::ptr::NonNull::new(smalls_ff[i]).unwrap(), small); }
                ff.dealloc(b, big);
            }
        }
        guards // 守卫块保活到测量结束(防合并)
    };

    let mut ff = FirstFit::empty();
    let mut tf = MiniTlsf::empty();
    let (bf, lf, bt, lt) = unsafe {
        (
            core::ptr::addr_of_mut!(BACK_FF2) as *mut u8,
            BACK_FF2.len(),
            core::ptr::addr_of_mut!(BACK_TF2) as *mut u8,
            BACK_TF2.len(),
        )
    };
    let _g1 = build(bf, lf, false, &mut ff, &mut tf);
    let _g2 = build(bt, lt, true, &mut ff, &mut tf);

    // 各测 8 轮(分配→释放→再分配,洞不变),取最小周期数
    let mut ff_min = usize::MAX;
    let mut tf_min = usize::MAX;
    for _ in 0..8 {
        let c0 = rdcycle();
        let b1 = unsafe { ff.alloc(big) };
        let c1 = rdcycle();
        unsafe { ff.dealloc(b1.unwrap(), big) };
        let c2 = rdcycle();
        let b2 = unsafe { tf.alloc(big) };
        let c3 = rdcycle();
        unsafe { tf.dealloc(b2.unwrap(), big) };
        ff_min = ff_min.min(c1 - c0);
        tf_min = tf_min.min(c3 - c2);
    }
    sprintln!("  [test19] K={K} first-fit={ff_min}cyc tlsf={tf_min}cyc");
    check(
        tf_min < ff_min && ff_min >= 2 * tf_min,
        "tlsf_alloc_determinism",
        format!("K={K} 深链:TLSF({tf_min}cyc)应显著快于 first-fit({ff_min}cyc,≥2×)"),
    );
}

// ============ 第 29 章:tickless 动态节拍 ============
//
// 本节各测的"投递及时性"谓词(零中断/exp 上界/拍账窗)是**环境承诺**:
// 本口 QEMU 的虚拟时钟随主机墙钟推进,主机某刻调度不到 vCPU 线程时,
// 到期中断会在 vCPU 恢复后才投递——惰性全部期进入被测的"迟到"。
// 空载本机实测迟到 <1 拍(0.375ms);门禁/忙碌窗口实测 +14~+55 拍
// (2026-08-27:门禁内 +14/+16.9,满载窗口 +55)。故本节上界统一取
// 40 拍环境容差常数——**判别力全部放在语义侧**:下限"不早于期限"、
// 中断计数差分(tl≤4 vs per≥100)、多集合完备。真失败 = 账烂(早醒/
// 丢窗口/计数同档),假红只可能是投递慢——账本自洽(el 跳账吸收惰性,
// 见测试 20/23 注释),重跑即可分辨。

/// 测试 20:tickless 错峰睡眠——不早醒 + 节拍中断计数差分 ★
/// 三个任务错峰睡 30/50/100ms;各自记录 (pre, post) tick——跳账到点
/// 账目不早于期限(post-pre ≥ 毫秒数,1000Hz 下即同数;上限为 40 拍
/// 环境容差,见 wins_ok 注)。判别器是节拍中断计数:恒定节拍每拍一次
/// (窗口 ≈ 100 次),tickless 只到点才中(错峰 3 次)——"到点之外
/// 零中断"就是动态节拍的定义。空载下投递及时,可观测到 30→50→100
/// 依次错峰唤醒;投递迟到时过期任务同帧唤醒(记录序不定,按 ms 配对)。
/// 同场景关掉 tickless 作为阳性对照:计数暴涨,拍账仍不早于期限
/// (绝对时刻账本与逐拍账在"拍数"上等价,差的是墙钟相位与中断次数)。
fn test_tickless_staggered_wakes() {
    use xtask::chip::qemu_riscv::debug_tick_isr_count;
    static WINS: Mutex<Vec<(usize, u64, u64)>> = Mutex::new(Vec::new());
    let done = XArc::new(Semaphore::new());
    let d2 = done.clone();
    let d3 = done.clone();
    let d4 = done.clone();
    WINS.lock().clear();

    fn spawn_sleeper(ms: usize, done_arc: XArc<Semaphore>) {
        TaskBuilder::new().name("t20.slp").priority(9).spawn(move || {
            let pre = xtask::tick();
            xtask::sleep_ms(ms);
            let post = xtask::tick();
            WINS.lock().push((ms, pre, post));
            done_arc.post();
        });
    }

    // —— tickless 档(默认开)——
    let isr0 = debug_tick_isr_count();
    spawn_sleeper(30, done.clone());
    spawn_sleeper(50, d2);
    spawn_sleeper(100, d3);
    for _ in 0..3 {
        done.wait();
    }
    let isr1 = debug_tick_isr_count();
    let w_tl = WINS.lock().clone();

    // —— 恒定节拍对照(同场景,阳性对照)——
    WINS.lock().clear();
    xtask::tickless::set_enabled(false);
    let isr2 = debug_tick_isr_count();
    spawn_sleeper(30, done.clone());
    spawn_sleeper(50, done.clone());
    spawn_sleeper(100, d4);
    for _ in 0..3 {
        done.wait();
    }
    let isr3 = debug_tick_isr_count();
    let w_per = WINS.lock().clone();
    xtask::tickless::set_enabled(true);

    let wins_ok = |w: &[(usize, u64, u64)]| {
        w.len() == 3
            && {
                // 按 ms 值配对(不按下标):投递迟到时多个过期任务会在同一次
                // 补账里一起唤醒,记录先后不定(踩坑 3 环境容差;空载下
                // 实测仍严格 30→50→100 依次错峰)
                let mut mss = [w[0].0, w[1].0, w[2].0];
                mss.sort_unstable();
                mss == [30, 50, 100]
            }
            && w.iter().all(|(ms, pre, post)| {
                let exp = xtask::time::ms2ticks(*ms) as u64;
                let delta = *post - *pre;
                // 下限"不早于期限"是 sleep 语义(严格);上限 40 拍是
                // **环境容差**——本口虚拟时钟随主机墙钟走,主机调度不到
                // vCPU 时到期中断攒着晚投,空载实测 <1 拍(0.375ms),
                // 忙碌窗口实测 +14~+55 拍(与测试 23 同一常数,踩坑 3)
                *post >= *pre && delta >= exp && delta <= exp + 40
            })
    };
    let ok_tl = wins_ok(&w_tl);
    let ok_per = wins_ok(&w_per);
    let fires_tl = isr1 - isr0;
    let fires_per = isr3 - isr2;
    check(
        ok_tl && ok_per && fires_tl <= 4 && fires_per >= 100 && fires_per > 10 * fires_tl,
        "tickless_staggered_wakes",
        format!(
            "拍账 tl={ok_tl} per={ok_per}(应精确 30/50/100);中次数 tl={fires_tl} per={fires_per}\
             (应 tl≤4 且 per≥100——到点之外零中断) tl数据={w_tl:?}",
        ),
    );
}

/// 测试 21:单个远期期限——150ms 只到点一次 ★
/// 睡眠窗口内只有一个期限:节拍中断次数 tickless ≈ 1(武装一次、到点一次),
/// 恒定节拍 ≈ 150(每毫秒一拍)。拍账两档都不早于期限(150;上限 40 拍
/// 环境容差)。让"中间零拍"成为可断言的定量事实:tl ≤ 2 且 per ≥ 100。
fn test_tickless_far_deadline() {
    use xtask::chip::qemu_riscv::debug_tick_isr_count;
    static WIN: Mutex<(u64, u64)> = Mutex::new((0, 0));
    let done = XArc::new(Semaphore::new());
    let d2 = done.clone();

    // —— tickless 档(默认开)——
    let isr0 = debug_tick_isr_count();
    TaskBuilder::new().name("t21.far").priority(9).spawn(move || {
        let pre = xtask::tick();
        xtask::sleep_ms(150);
        let post = xtask::tick();
        *WIN.lock() = (pre, post);
        d2.post();
    });
    done.wait();
    let isr1 = debug_tick_isr_count();
    let (pre_tl, post_tl) = *WIN.lock();

    // —— 恒定节拍对照(阳性对照)——
    xtask::tickless::set_enabled(false);
    let isr2 = debug_tick_isr_count();
    let d3 = done.clone();
    TaskBuilder::new().name("t21.far2").priority(9).spawn(move || {
        let pre = xtask::tick();
        xtask::sleep_ms(150);
        let post = xtask::tick();
        *WIN.lock() = (pre, post);
        d3.post();
    });
    done.wait();
    let isr3 = debug_tick_isr_count();
    let (pre_per, post_per) = *WIN.lock();
    xtask::tickless::set_enabled(true);

    let exp = xtask::time::ms2ticks(150) as u64;
    let d_tl = post_tl - pre_tl;
    let d_per = post_per - pre_per;
    let fires_tl = isr1 - isr0;
    let fires_per = isr3 - isr2;
    check(
        // 拍账下限不早于期限(语义,严格);上限 +40 拍 = 投递迟到环境
        // 容差(空载 <1 拍,门禁/忙碌实测 +14~+55 拍,与测试 20/23 同一
        // 常数,踩坑 3)。真正的判别器是 fires:远期期限只到点一次
        d_tl >= exp && d_tl <= exp + 40
            && d_per >= exp && d_per <= exp + 40
            && fires_tl <= 2
            && fires_per >= 100
            && fires_per > 10 * fires_tl,
        "tickless_far_deadline",
        format!(
            "拍账 tl={d_tl} per={d_per}(应 [{exp}, {}]);中次数 tl={fires_tl} per={fires_per}\
             (应 tl≤2:远期期限只到点一次)",
            exp + 40
        ),
    );
}

// ============ 第 29 章(练习 1 兑现):外部中断——冻眠与早醒 ============

/// 测试 22/23 的 ISR→任务 唤醒槽。ISR 回调是裸 fn 指针(无捕获),信号量
/// 经 `Arc::into_raw` 泄漏后把裸指针存进这个原子槽(借用即可,ISR 不碰
/// 引用计数;泄漏一个 Arc 是测试可接受的代价)。0 = 尚未登记。
/// 两次测试共用同一实例:信号量计数语义,先后消费互不干扰
static UART_WAKE_SEM: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

/// UART RX 中断回调(ISR 上下文):post_isr 唤醒槽里的考官
fn uart_rx_wake_cb() {
    let p = UART_WAKE_SEM.load(Ordering::Relaxed);
    if p != 0 {
        // SAFETY: 槽里只存过 Arc::into_raw 泄漏的 Semaphore,存活期 = 测试
        // 全程(泄漏),此处只借 &Semaphore 调用 post_isr
        let s: &Semaphore = unsafe { &*(p as *const Semaphore) };
        let _ = s.post_isr();
    }
}

/// 测试 22:冻眠(SleepForever)→ 外部中断唤醒 → 恢复调度 ★
/// 书稿 ch29 踩坑 4 的诚实边界由此关闭:考官挂起后全系统零就绪、零期限,
/// idle 走到 SleepForever 停表+wfi 深睡——**能叫醒它的只剩外部中断**。
/// 验证器从串口读到 T22-FROZEN 标记后向 qemu stdin 喂一个字节(握手,
/// 无任何时序假设),UART RX 中断(PLIC,mext)正是那个"外部"。
/// 断言三连:①冻眠期节拍中断 ≤1 次(时钟真停了,不是空转——这是与
/// "tickless 关掉后自旋等拍"的判别器;≤1 容纳停表入窗竞态,见下);
/// ②睡眠跨度 > 0(真睡过、唤醒者只能是外部中断;跨度长短取决于验证器
/// 反应,冻眠没有最短时长);③唤醒后 30ms 睡眠拍账不早于期限(节拍链
/// 完好,一轮完整"睡→外部打断→恢复"闭环;上界 40 拍环境容差)。
fn test_tickless_frozen_wake() {
    use xtask::chip::qemu_riscv::{
        debug_tick_isr_count, uart_disable_rx_irq, uart_enable_rx_irq, uart_set_rx_callback,
        TICK_PERIOD,
    };
    use xtask::port::{Portable, Porting};
    let wake = XArc::new(Semaphore::new());
    UART_WAKE_SEM.store(XArc::into_raw(wake.clone()) as usize, Ordering::Relaxed);
    uart_set_rx_callback(Some(uart_rx_wake_cb));
    uart_enable_rx_irq();
    let isr0 = debug_tick_isr_count();
    let w0 = Porting::systick();
    // 握手标记:验证器读到才喂字节——字节必然落在冻眠之后
    write_str("T22-FROZEN: zero-deadline frozen, feed byte\r\n");
    wake.wait(); // 考官挂起 → 唯一任务全阻塞 → SleepForever 停表 + wfi
    let w1 = Porting::systick();
    let isr1 = debug_tick_isr_count();
    uart_disable_rx_irq();
    uart_set_rx_callback(None);
    // ① 冻眠期零节拍(容入窗竞态 ≤1):isr0 读数到停表之间有一个
    // 指令级入窗——停表前已锁存的到期会在 wfi 处投递,计 1 次;这是
    // 边界竞态不是时钟未停(自旋档每毫秒 1 次,冻眠跨度内会是几十上百)
    check(
        isr1 - isr0 <= 1,
        "tickless_frozen_wake",
        format!("冻眠期节拍中断 {} 次(应 ≤1——时钟真停了;1=入窗竞态)", isr1 - isr0),
    );
    // ② 真睡过:唤醒者只可能是外部字节(零节拍中断期间,没有别的
    // 唤醒源);跨度长短 = 验证器反应速度 + 字节到达,冻眠没有最短时长
    let span = w1 - w0;
    check(
        span > 0,
        "tickless_frozen_wake",
        format!("冻眠跨度 {span} mtime 拍(应 >0——真睡过,唤醒者=外部中断)"),
    );
    // ③ 唤醒后节拍链完好:30ms 睡眠不早于期限,上界 40 拍环境容差
    // (与测试 20/21/23 同一常数,踩坑 3)
    let t0 = xtask::tick();
    xtask::sleep_ms(30);
    let dt = xtask::tick() - t0;
    check(
        (30..=70).contains(&dt),
        "tickless_frozen_wake",
        format!("唤醒后 30ms 睡眠拍账 {dt}(应 [30,70]——外部唤醒后节拍链仍精确)"),
    );
}

/// 测试 23:睡眠中被外部中断早醒——墙钟期限不得被拖后 ★
/// 书稿踩坑 5(早醒重武装漂移)的现行修复本无回归守卫,这里补上:
/// 任务睡 2000ms,中途被 UART RX 外部中断早醒(验证器读 T23-SLEEPING
/// 标记后喂字节)。判别用**墙钟**(systick)而非 TICKS——TICKS 账本
/// 自洽(跳账把早醒段一并记上),漂移只对墙钟可见:
///   有 leave_idle 补账:重新武装按"期限 - 已过拍"算 → 墙钟 2000ms 到点;
///   无(踩坑 5 初稿):新武装锚在冻结的 TICKS 上 → 墙钟拖后 ≈ el(+早醒
///   到补账点之间的全段),断言 [exp, exp+40ms] 必红。
///   阈值说明:上界是**环境**容差不是语义——TCG -smp 2 下到期 ISR 投递
///   有重尾(实测 2.1ms/16.9ms——多 vCPU 轮转推迟 trap),墙钟拍账把
///   投递延迟一并量进"迟到"。判别力:无补账时墙钟拖后 = 喂字节前置延迟
///   150ms(≈ 3.75× 上界);有补账时实测迟到 ≤16.9ms(≈ 0.42× 上界)。
/// 顺带:考官被早醒唤醒运行的那一刻正是 leave_idle 边界钩子的执行现场。
fn test_tickless_early_wake_drift() {
    use xtask::chip::qemu_riscv::{
        uart_disable_rx_irq, uart_enable_rx_irq, uart_set_rx_callback, TICK_PERIOD,
    };
    use xtask::port::{Portable, Porting};
    static WIN: Mutex<(u64, u64)> = Mutex::new((0, 0));
    const MS: usize = 2000;
    let wake = {
        // 槽里是测试 22 泄漏的 Arc 裸指针:补一次强引用计数再取回(ISR 的
        // 槽不消费计数,这里补的是"取回"这一份)
        let raw = UART_WAKE_SEM.load(Ordering::Relaxed) as *const Semaphore;
        unsafe { XArc::increment_strong_count(raw) };
        unsafe { XArc::from_raw(raw) }
    };
    let done = XArc::new(Semaphore::new());
    let d2 = done.clone();
    *WIN.lock() = (0, 0);
    uart_set_rx_callback(Some(uart_rx_wake_cb));
    uart_enable_rx_irq();
    TaskBuilder::new().name("t23.slp").priority(9).spawn(move || {
        let pre = Porting::systick();
        write_str("T23-SLEEPING: early IRQ will interrupt 2000ms sleep\r\n");
        xtask::sleep_ms(MS);
        let post = Porting::systick();
        *WIN.lock() = (pre, post);
        d2.post();
    });
    wake.wait(); // 睡眠中被外部中断早醒(考官运行 = leave_idle 补账现场)
    done.wait(); // 等任务真正到点(墙钟 2000ms)
    let (pre, post) = *WIN.lock();
    uart_disable_rx_irq();
    uart_set_rx_callback(None);
    let exp = MS as u64 * TICK_PERIOD;
    let d = post - pre;
    check(
        d >= exp && d <= exp + 40 * TICK_PERIOD,
        "tickless_early_wake_drift",
        format!(
            "墙钟拍账 {d}(应 [{}, {}]——早醒不早醒墙钟期限都得到点,\
             拖后意味着早醒段没人记账)", exp, exp + 40 * TICK_PERIOD
        ),
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
    // 输出与计数必乱(check.sh 的 23/23 与 -smp 2 门禁会抓到)
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
                ("priority_inheritance", test_priority_inheritance),
                ("priority_inheritance_multi_hold", test_priority_inheritance_multi_hold),
                ("pcp_ceiling_blocked", test_pcp_ceiling_blocked),
                ("pi_cross_acquire_deadlock", test_pi_cross_acquire_deadlock),
                ("tlsf_fragmentation", test_tlsf_fragmentation),
                ("tlsf_alloc_determinism", test_tlsf_alloc_determinism),
                ("tickless_staggered_wakes", test_tickless_staggered_wakes),
                ("tickless_far_deadline", test_tickless_far_deadline),
                ("tickless_frozen_wake", test_tickless_frozen_wake),
                ("tickless_early_wake_drift", test_tickless_early_wake_drift),
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
