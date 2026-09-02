#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use gd32vf103xx_hal as hal;
use hal::{gpio::GpioExt, pac, prelude::*, rcu::RcuExt};

use xtask::arch::riscv::rt;
use xtask::bsp::longan_nano::drv_sd::SdCard;
use xtask::device::table::{self, DeviceSlot};
use xtask::fs::{
    format_volume, FatAdapter, FileSystem, FormatVolumeOptions, FsOptions, Read, Seek, SeekFrom,
    Write,
};
use xtask::prelude::*;

// 文件系统示例（第 21 章）：全部组件在一条链上
//   SdCard（SPI 驱动，bsp）→ BlockDevice（两个函数：读/写扇区，ch20 驱动层）
//   → FatAdapter（字节流翻译，宿主回归）→ fatfs → 文件任务
// - writer（prio 2）：每秒向 LOG.TXT 追加一行（"文件 I/O 与任务阻塞"——
//   writer/reader 抢同一把 FS 互斥锁，抢不到的任务进入 Blocked）
// - reader（prio 3）：每 3 秒全量读回 LOG.TXT 打印（内容被打断的直觉破除：
//   文件操作都在持锁闭包里，没有"读到一半"）
// - 首次上电卷未格式化：自动 FAT 格式化后重挂（警告：抹掉卡上全部内容！
//   真机上这就是"格式化确认框"的实现位置）
// - 日志走 USART0 终端（57600，8N1）；本例不占用 uart 设备类

// 编译期设备清单：SD 卡经 ch20 驱动层按名获取（清单与注册表同认 `&'static dyn Device`）
xtask::device_list! { BOARD_SD {
    "sd0" => &SD_SLOT,
} }

static SD_SLOT: DeviceSlot = DeviceSlot::new();

/// 文件系统类型：fatfs 操作的是"字节流 + 游标"（FatAdapter），
/// 存储来自驱动层按名取出的块设备（`&'static dyn BlockDevice`）
type SdFs = FileSystem<FatAdapter<&'static dyn BlockDevice>>;

/// 挂载好的文件系统：全局唯一（单卡单卷）。
/// 注意用 `Mutex<Option<_>>` 而不是 `OnceCell`：core 的 `OnceCell` 是
/// 刻意 `!Sync` 的（`set` 与 `get` 可跨线程竞争，once.rs 里 `impl !Sync`），
/// 静态全局放不进去；挂载 = 启动期加锁把 `None` 写成 `Some`，
/// 任务侧先 `lock()` 再 `as_ref()`——正好也演示"文件系统所有权在锁里"。
static FS: Mutex<Option<SdFs>> = Mutex::new(None);

/// 挂载（或格式化后挂载）文件系统。**必须在 `xtask::start()` 前调用**：
/// 首次上电的格式化与坏卷处理都发生在调度器启动前——出问题直接 panic
/// 可见；若推迟到任务里，会与 writer/reader 抢同一把 FS 锁，还可能在
/// 第一个文件操作时才被迫格式化（卡顿发生在任务上下文里，难定位）。
fn mount() {
    let bd = table::find_block("sd0").expect("SD 卡未就绪：清单未挂载或槽未填");
    let fs = match FileSystem::new(FatAdapter::new(bd), FsOptions::new()) {
        Ok(fs) => fs,
        // 全 0 卷（无 55AA 签名）= 未格式化（宿主回归：unformatted_volume_open_fails）
        Err(_) => {
            log::warn!(
                "fatfs: 卷未格式化，执行 FAT 格式化——警告：这会抹掉 TF 卡上的全部内容！"
            );
            format_volume(&mut FatAdapter::new(bd), FormatVolumeOptions::new())
                .expect("FAT 格式化失败");
            log::info!("fatfs: 格式化完成，重新挂载");
            FileSystem::new(FatAdapter::new(bd), FsOptions::new()).expect("格式化后挂载失败")
        }
    };
    let clusters = fs.stats().expect("统计失败").total_clusters();
    // 挂载 = 写 Some（启动前无人竞争；重复挂载直接 panic 暴露）
    *FS.lock() = Some(fs);
    log::info!(
        "fatfs: 挂载成功，容量 {} 扇区（{} KiB），簇总数 {}",
        bd.sector_count(),
        bd.sector_count() * bd.sector_size() / 1024,
        clusters
    );
}

/// 向 LOG.TXT 追加一行。整个"打开→定位→写入→释放"在一把 FS 锁内。
/// 追加写 = `seek(End(0))`：fatfs 的 File 持有目录项快照，size 从快照取，
/// 打开时的 size 即真实长度（宿主回归：空文件 seek(End(0)) 返回 Ok(0)）。
fn append_log(line: &str) {
    let guard = FS.lock(); // writer/reader 串行化：抢不到的任务进入 Blocked
    let fs = guard.as_ref().expect("FS 未挂载（mount 未调用）");
    let mut f = match fs.root_dir().open_file("LOG.TXT") {
        Ok(f) => f,
        Err(e) if matches!(e, xtask::fs::Error::NotFound) => {
            fs.root_dir().create_file("LOG.TXT").expect("创建 LOG.TXT 失败")
        }
        Err(e) => panic!("打开 LOG.TXT 失败: {:?}", e),
    };
    f.seek(SeekFrom::End(0)).expect("定位文件尾失败");
    f.write_all(line.as_bytes()).expect("写入失败");
    // guard 在此释放：写文件的任务把 FS 锁交还系统
}

/// 全量读回 LOG.TXT 并打印（reader 专用：每次一个完整快照）。
fn read_log() {
    let guard = FS.lock();
    let fs = guard.as_ref().expect("FS 未挂载（mount 未调用）");
    let mut f = match fs.root_dir().open_file("LOG.TXT") {
        Ok(f) => f,
        Err(_) => {
            log::info!("reader: LOG.TXT 还不存在，等 writer 创建");
            return;
        }
    };
    let mut buf = Vec::new();
    let mut tmp = [0u8; 128];
    loop {
        let n = f.read(&mut tmp).expect("读取失败");
        if n == 0 {
            break; // EOF：fatfs 的 read 返回 0（适配器的 EOF 语义）
        }
        buf.extend_from_slice(&tmp[..n]);
    }
    let text = String::from_utf8(buf).unwrap_or_else(|_| "<非 UTF-8 内容>".into());
    let lines = text.lines().count();
    log::info!("reader: LOG.TXT 共 {lines} 行、见下——");
    for line in text.lines() {
        log::info!("  {line}");
    }
}

fn init() {
    extern "C" {
        /// 堆内存开始地址，在 riscv-rt link.x 文件里定义
        static _sheap: u8;
    }
    // 堆要够 FAT 的中间分配（目录项字符串、统计）：64 KB
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init_heap(start_addr, 64 * 1024);
    xtask::init_logger();

    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let mut afio = dp.AFIO.constrain(&mut rcu);

    // 日志走 USART0 终端（57600，8N1）；本例不占 uart 设备类
    xtask::bsp::longan_nano::stdout::configure(
        dp.USART0,
        gpioa.pa9,
        gpioa.pa10,
        57600.bps(), // 这块板子 PCB 设计有瑕疵，uart 速率只能到 57600
        &mut afio,
        &mut rcu,
    );

    // 板载 TF 卡：SPI1 原生引脚（PB13 SCK / PB14 MISO / PB15 MOSI），CS=PB12。
    // 失败即 panic（无卡/卡坏）：文件系统没有卡就没有意义，启动期直接暴露
    let sd = SdCard::new(dp.SPI1, gpiob.pb12, gpiob.pb13, gpiob.pb14, gpiob.pb15, &mut rcu)
        .unwrap_or_else(|e| panic!("SD 卡初始化失败: {:?}（检查 TF 卡是否插好）", e));

    // 挂进 ch20 驱动层：编译期清单里登记名字，消费者按名取
    SD_SLOT.fill(sd);
    table::attach(BOARD_SD);

    mount();
}

#[rt::entry]
fn main() -> ! {
    init();

    // writer：每秒追加一行。prio 2（数字小优先级高）——比 reader 高，
    // 但二者大部分时间在 sleep/拿锁上，互不忙等
    TaskBuilder::new()
        .name("writer")
        .priority(2)
        .stack_size(1024) // FAT 适配器在调用方栈上放 512B 暂存，任务栈要够
        .spawn(move || {
            let mut seq = 0u32;
            loop {
                seq += 1;
                append_log(&format!("#{seq}: xtask 文件系统心跳 {}\n", "=".repeat(1 + (seq % 16) as usize)));
                xtask::sleep_ms(1000);
            }
        });

    // reader：每 3 秒全量读回。prio 3（数字小优先级高，但比 writer 低）
    TaskBuilder::new()
        .name("reader")
        .priority(3)
        .stack_size(1024) // 同上：适配器暂存 + 128B 读缓冲
        .spawn(|| loop {
            read_log();
            xtask::sleep_ms(3000);
        });

    xtask::start()
}
