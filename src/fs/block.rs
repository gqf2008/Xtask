//! 块设备 → FatFS 的字节流适配层
//!
//! 第 21 章的分层：**块设备只会两个动作**（读扇区、写扇区，`drv::BdDevice`），
//! FAT 文件系统想要的是"字节流 + 可移动游标"（`Read`/`Write`/`Seek`）——本模块的
//! [`FatAdapter`] 就是这 30 行翻译：把一个 `BdDevice` 变成 fatfs 可以直接开卷的存储。
//!
//! 三个正确的关键点（每一个都有对应宿主回归，见测试）：
//! 1. **扇区跨界数学**——一个字节流读写会跨两个扇区；首尾非对齐段必须走
//!    "读整扇区、再切片"（读）或"读-改-写"（写）。写绝不能直写 512 字节：
//!    FAT 的脏标志只有 1 字节（偏移 0x25），直写会毁掉引导扇区其余字段；
//! 2. **EOF 语义**——读/写到卷尾返回 `Ok(0)`（fatfs 的 `read_exact` 靠它
//!    判断结束），越界 seek 返回 `Err(InvalidInput)`；
//! 3. **写入契约**——fatfs 要求"返回 Err 时本缓冲区零写入"；跨扇区写入中途
//!    失败时降级为 `Ok(done)` 短写，把真正的失败留到下一次调用（那时坏扇区
//!    成了本次第一个扇区，`done == 0` 才能诚实报 Err）。

use crate::drv::{BdDevice, BdError, SECTOR_SIZE};
use fatfs::{IoBase, IoError, Read, Seek, SeekFrom, Write};

/// `BdDevice` → fatfs 存储的适配器：内部只有一个字节游标。
/// fatfs 对存储的访问全部是 `seek + read/write`，游标即全部状态（无缓冲，
/// 所以 [`Write::flush`] 是空实现——没有需要落盘的中间状态）。
pub struct FatAdapter<B: BdDevice> {
    /// 被包装的块设备（具体设备或 `&'static dyn BdDevice` 均可）
    dev: B,
    /// 字节游标（fatfs 的一切读写都落到它之上）
    pos: u64,
}

impl<B: BdDevice> FatAdapter<B> {
    /// 包装一个块设备。**开卷前游标必须为 0**——fatfs 的 `FileSystem::new`
    /// 有调试断言 `seek(Current(0)) == 0`（fs.rs:380），用完的适配器不要复用。
    pub fn new(dev: B) -> Self {
        Self { dev, pos: 0 }
    }

    /// 整卷字节数（`format_volume` 会以 `SeekFrom::End(0)` 探测它）
    pub fn total_bytes(&self) -> u64 {
        self.dev.sector_count() * SECTOR_SIZE
    }
}

// fatfs 的错误类型是泛型 `Error<E: IoError>`：给 `BdError` 实现 `IoError`，
// 适配器的 `type Error = BdError` 即可直接用 `?` 放入 `Error::Io`。
impl IoError for BdError {
    /// 无"被中断可重试"语义（不做信号量式重试），恒 false
    fn is_interrupted(&self) -> bool {
        false
    }
    /// `read_exact` 提前碰到卷尾时的构造器
    fn new_unexpected_eof_error() -> Self {
        BdError::UnexpectedEof
    }
    /// `write_all` 拿到 `Ok(0)`（写零字节）时的构造器
    fn new_write_zero_error() -> Self {
        BdError::WriteZero
    }
}

impl<B: BdDevice> IoBase for FatAdapter<B> {
    type Error = BdError;
}

impl<B: BdDevice> Read for FatAdapter<B> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, BdError> {
        let total = self.total_bytes();
        if self.pos >= total {
            return Ok(0); // EOF：read_exact 判结束的语义基础
        }
        let n = (buf.len() as u64).min(total - self.pos) as usize;
        let mut done = 0;
        // 栈上暂存 512B：非对齐段"读整扇区再切片"。注意调用方（文件任务）
        // 的栈要为此余量——示例里文件任务用 1024+ words 的栈。
        let mut tmp = [0u8; SECTOR_SIZE as usize];
        while done < n {
            let abs = self.pos + done as u64;
            let sec = abs / SECTOR_SIZE;
            let off = (abs % SECTOR_SIZE) as usize;
            let chunk = (SECTOR_SIZE as usize - off).min(n - done);
            let r = if off == 0 && chunk == SECTOR_SIZE as usize {
                // 整扇区对齐：直读进目标缓冲
                self.dev.read_sector(sec, &mut buf[done..done + chunk])
            } else {
                self.dev.read_sector(sec, &mut tmp).map(|_| {
                    buf[done..done + chunk].copy_from_slice(&tmp[off..off + chunk]);
                })
            };
            match r {
                Ok(()) => done += chunk,
                // 契约补偿：错误时"本次未写任何字节"——已完成的部分退回短读，
                // 未读到的留给下一次（坏扇区那时成为第一个扇区才诚实报错）。
                Err(_) if done > 0 => break,
                Err(e) => return Err(e),
            }
        }
        self.pos += done as u64;
        Ok(done)
    }
}

impl<B: BdDevice> Write for FatAdapter<B> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, BdError> {
        let total = self.total_bytes();
        if self.pos >= total {
            return Ok(0); // 卷尾之外的写：0 字节（write_all 会据此报 WriteZero）
        }
        let n = (buf.len() as u64).min(total - self.pos) as usize;
        let mut done = 0;
        let mut tmp = [0u8; SECTOR_SIZE as usize];
        while done < n {
            let abs = self.pos + done as u64;
            let sec = abs / SECTOR_SIZE;
            let off = (abs % SECTOR_SIZE) as usize;
            let chunk = (SECTOR_SIZE as usize - off).min(n - done);
            let r = if off == 0 && chunk == SECTOR_SIZE as usize {
                self.dev.write_sector(sec, &buf[done..done + chunk])
            } else {
                // 读-改-写：write_sector 的契约是"写完整 512B"，
                // 非对齐部分直写会毁掉同扇区其他字段（脏标志只有 1 字节！）。
                self.dev.read_sector(sec, &mut tmp).and_then(|_| {
                    tmp[off..off + chunk].copy_from_slice(&buf[done..done + chunk]);
                    self.dev.write_sector(sec, &tmp)
                })
            };
            match r {
                Ok(()) => done += chunk,
                Err(_) if done > 0 => break, // 同上：短写，失败延后
                Err(e) => return Err(e),
            }
        }
        self.pos += done as u64;
        Ok(done)
    }

    fn flush(&mut self) -> Result<(), BdError> {
        Ok(()) // 无缓冲，无中间状态可落盘
    }
}

impl<B: BdDevice> Seek for FatAdapter<B> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, BdError> {
        let len = self.total_bytes();
        let new = match pos {
            SeekFrom::Start(x) => x,
            SeekFrom::End(e) => {
                let v = (len as i64).checked_add(e).ok_or(BdError::InvalidInput)?;
                if v < 0 || v > len as i64 {
                    return Err(BdError::InvalidInput);
                }
                v as u64
            }
            SeekFrom::Current(e) => {
                let v = (self.pos as i64).checked_add(e).ok_or(BdError::InvalidInput)?;
                if v < 0 || v > len as i64 {
                    return Err(BdError::InvalidInput);
                }
                v as u64
            }
        };
        if new > len {
            return Err(BdError::InvalidInput);
        }
        self.pos = new;
        Ok(new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drv::BdDevice;
    use core::cell::{Cell, RefCell};
    use fatfs::Error as FatError;
    use fatfs::{format_volume, FileSystem, FormatVolumeOptions, FsOptions};
    use fatfs::FatType;

    /// 内存盘：1MiB（2048 扇区 → FAT12、簇 512B，测试友好）。
    /// 坏扇区可注入：命中后读/写返回 Err(BdError::Io)——错误传播路径的失控测试。
    struct RamDisk {
        buf: RefCell<Vec<u8>>,
        bad_read: Cell<Option<u64>>,
        bad_write: Cell<Option<u64>>,
    }

    // SAFETY: `BdDevice: Sync` 超约束要求所有实现兼具 Sync；本 mock 仅在
    // 宿主测试内使用——所有访问发生在同一个测试线程里（借用的借出与归还
    // 成对出现于单个测试函数体内），不存在跨线程/跨上下文并发访问。
    unsafe impl Sync for RamDisk {}

    impl RamDisk {
        fn new() -> Self {
            Self {
                buf: RefCell::new(vec![0u8; 2048 * SECTOR_SIZE as usize]),
                bad_read: Cell::new(None),
                bad_write: Cell::new(None),
            }
        }
        fn burn_bad_sectors(&self, read: Option<u64>, write: Option<u64>) -> &Self {
            self.bad_read.set(read);
            self.bad_write.set(write);
            self
        }
        fn snapshot(&self, no: u64) -> [u8; SECTOR_SIZE as usize] {
            let buf = self.buf.borrow();
            let mut s = [0u8; SECTOR_SIZE as usize];
            s.copy_from_slice(&buf[no as usize * 512..(no as usize + 1) * 512]);
            s
        }
        fn read_u8(&self, no: u64, off: usize) -> u8 {
            self.buf.borrow()[no as usize * 512 + off]
        }
    }

    impl BdDevice for RamDisk {
        fn sector_count(&self) -> u64 {
            2048
        }
        fn read_sector(&self, no: u64, buf: &mut [u8]) -> Result<(), BdError> {
            if self.bad_read.get() == Some(no) {
                return Err(BdError::Io);
            }
            assert!(no < self.sector_count(), "扇区号越界");
            buf.copy_from_slice(&self.snapshot(no));
            Ok(())
        }
        fn write_sector(&self, no: u64, data: &[u8]) -> Result<(), BdError> {
            if self.bad_write.get() == Some(no) {
                return Err(BdError::Io);
            }
            assert!(no < self.sector_count(), "扇区号越界");
            let mut buf = self.buf.borrow_mut();
            buf[no as usize * 512..(no as usize + 1) * 512].copy_from_slice(data);
            Ok(())
        }
    }

    fn adapter(disk: &RamDisk) -> FatAdapter<&RamDisk> {
        FatAdapter::new(disk)
    }

    /// 阳性对照：未格式化（全 0）的卷打开必须失败——静默当有效卷会让
    /// 文件系统在垃圾数据上"工作"，是最隐蔽的坏卷形态。
    #[test]
    fn unformatted_volume_open_fails() {
        let disk = RamDisk::new();
        let err = match FileSystem::new(adapter(&disk), FsOptions::new()) {
            Ok(_) => panic!("未格式化卷不应打开成功"),
            Err(e) => e,
        };
        assert!(
            matches!(err, FatError::CorruptedFileSystem),
            "全 0 卷应识别为损坏文件系统，实际 {:?}", err
        );
    }

    /// 回归：格式化 → 挂载，FAT12/簇 512B（1MiB 盘的期望布局）。
    #[test]
    fn format_then_mount_fat12() {
        let disk = RamDisk::new();
        format_volume(&mut adapter(&disk), FormatVolumeOptions::new()).expect("格式化应成功");
        let fs = FileSystem::new(adapter(&disk), FsOptions::new()).expect("格式化后应能挂载");
        assert_eq!(fs.fat_type(), FatType::Fat12);
        assert_eq!(fs.cluster_size(), 512);
        assert!(fs.stats().unwrap().total_clusters() > 0);
    }

    /// 回归：文件创建→写入→重开→读回 全流程（fatfs 的完整目录/FAT 路径）。
    /// 写在前、drop 在后（File 借用 FS，作用域结束即释放），重开再读。
    #[test]
    fn file_create_write_read_roundtrip() {
        let disk = RamDisk::new();
        format_volume(&mut adapter(&disk), FormatVolumeOptions::new()).unwrap();
        let fs = FileSystem::new(adapter(&disk), FsOptions::new()).unwrap();
        {
            let mut f = fs.root_dir().create_file("A.TXT").expect("创建失败");
            f.write_all(b"hello xtask").expect("写入失败");
        }
        let mut f = fs.root_dir().open_file("A.TXT").expect("重开失败");
        let mut buf = [0u8; 32];
        let mut total = 0;
        loop {
            let n = f.read(&mut buf[total..]).expect("读取失败");
            if n == 0 {
                break; // EOF：read 返回 0
            }
            total += n;
        }
        assert_eq!(&buf[..total], b"hello xtask");
    }

    /// 回归（适配器层）：扇区跨界读写的数学——offset 300 写 700B（跨 2 扇区）→ 读回一致。
    /// 阳性对照：跨界数学错（如按"整扇区起点"对齐）读回即不一致。
    #[test]
    fn cross_sector_write_read() {
        let disk = RamDisk::new();
        let mut a = adapter(&disk);
        let pat: Vec<u8> = (0..700u32).map(|i| (i % 251) as u8).collect();
        a.seek(SeekFrom::Start(300)).unwrap();
        assert_eq!(a.write(&pat).unwrap(), 700);
        a.seek(SeekFrom::Start(300)).unwrap();
        let mut out = vec![0u8; 700];
        assert_eq!(a.read(&mut out).unwrap(), 700);
        assert_eq!(out, pat, "跨界读回必须与写入一致");
    }

    /// 回归（fs 层）：跨簇文件读写——FAT12 簇 512B，写 700B 跨 2 簇。
    #[test]
    fn cross_cluster_write_read() {
        let disk = RamDisk::new();
        format_volume(&mut adapter(&disk), FormatVolumeOptions::new()).unwrap();
        let fs = FileSystem::new(adapter(&disk), FsOptions::new()).unwrap();
        let pat: Vec<u8> = (0..700u32).map(|i| (i % 251) as u8).collect();
        {
            let mut f = fs.root_dir().create_file("B.TXT").unwrap();
            f.write_all(&pat).unwrap();
        }
        let mut f = fs.root_dir().open_file("B.TXT").unwrap();
        let mut out = vec![0u8; 700];
        f.read_exact(&mut out).unwrap();
        assert_eq!(out, pat);
    }

    /// 阳性对照：坏扇区写必须把错误穿透到适配器调用方——错误被吞（如
    /// 返回 Ok(0) 或 Ok(512)）即测试红；且对齐的整扇区写失败时不得有任何
    /// 半写残留（写的是第一个扇区，done==0 → 直接 Err）。
    #[test]
    fn bad_sector_write_propagates() {
        let disk = RamDisk::new();
        disk.burn_bad_sectors(None, Some(3));
        let mut a = adapter(&disk);
        let pat = [0x5A; SECTOR_SIZE as usize];
        a.seek(SeekFrom::Start(3 * SECTOR_SIZE)).unwrap();
        match a.write(&pat) {
            Err(BdError::Io) => {}
            other => panic!("坏扇区写应报 BdError::Io，实际 {:?}", other),
        }
        // 其他扇区不受影响（坏扇区本身不写，邻居也不该被动过）
        assert_eq!(disk.snapshot(2), [0u8; SECTOR_SIZE as usize]);
    }

    /// 回归：跨扇区写中途失败 → 短写（Ok(done)），失败扇区留到下次调用才报错
    /// （fatfs 契约：Err 时本次调用零写入）。
    #[test]
    fn cross_sector_write_mid_failure_short_writes() {
        let disk = RamDisk::new();
        disk.burn_bad_sectors(None, Some(1));
        let mut a = adapter(&disk);
        let pat = [0x77; 700];
        a.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(a.write(&pat).unwrap(), 512, "扇区 0 成功、扇区 1 坏 → 短写 512");
        // 下次调用：坏扇区是第一个扇区 → 诚实报错
        let more = [0x66; 512];
        match a.write(&more) {
            Err(BdError::Io) => {}
            other => panic!("坏扇区写应报错，实际 {:?}", other),
        }
    }

    /// 阳性对照：坏扇区读穿透 + 短读契约。
    #[test]
    fn bad_sector_read_propagates() {
        let disk = RamDisk::new();
        disk.burn_bad_sectors(Some(1), None);
        let mut a = adapter(&disk);
        let mut buf = [0u8; 700];
        a.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(a.read(&mut buf).unwrap(), 512, "扇区 0 读成、扇区 1 坏 → 短读 512");
        let mut buf2 = [0u8; 512];
        match a.read(&mut buf2) {
            Err(BdError::Io) => {}
            other => panic!("坏扇区读应报错，实际 {:?}", other),
        }
    }

    /// 回归：seek 越界报错；End(0) 返回整卷字节数（format_volume 的容量探测路径）。
    #[test]
    fn seek_bounds() {
        let disk = RamDisk::new();
        let mut a = adapter(&disk);
        let total = 2048 * SECTOR_SIZE;
        // pos = 0（游标起点）时向后 seek 必须报错
        assert!(a.seek(SeekFrom::Current(-1)).is_err(), "游标起点之前必须报错");
        assert_eq!(a.seek(SeekFrom::End(0)).unwrap(), total, "End(0) 返回整卷字节数");
        assert!(a.seek(SeekFrom::Start(total + 1)).is_err(), "越界必须报错");
        assert!(a.seek(SeekFrom::End(1)).is_err(), "卷尾之后必须报错");
    }

    /// 回归：读到卷尾下一次 read 返回 0（fatfs read_exact 依赖的 EOF 语义）。
    #[test]
    fn read_at_eof_returns_zero() {
        let disk = RamDisk::new();
        let mut a = adapter(&disk);
        let total = 2048 * SECTOR_SIZE;
        a.seek(SeekFrom::Start(total)).unwrap();
        let mut buf = [1u8; 16];
        assert_eq!(a.read(&mut buf).unwrap(), 0, "卷尾 read 必须返回 0");
    }

    /// 阳性对照：非对齐写必须走读-改-写——偏移 0x25 单字节写后，
    /// 同扇区其他字节必须原样保留（直写 512B 的实现会把它们清零 → 红）。
    /// FAT 脏标志就是 1 字节写在偏移 0x25（fs.rs:594-617），这一个字节的错误
    /// 会毁掉引导扇区其余字段，是适配器最容易犯的"整扇区直写"错误。
    #[test]
    fn partial_write_preserves_rest_of_sector() {
        let disk = RamDisk::new();
        let mut a = adapter(&disk);
        // 先铺满扇区 0 为一个已知模式
        let pat = [0xA5; SECTOR_SIZE as usize];
        a.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(a.write(&pat).unwrap(), 512);
        // 单字节写偏移 0x25
        let probe = [0xE7];
        a.seek(SeekFrom::Start(0x25)).unwrap();
        assert_eq!(a.write(&probe).unwrap(), 1);
        // 校验：0x25 变了，其余字节完好
        assert_eq!(disk.read_u8(0, 0x25), 0xE7);
        for i in 0..512u32 {
            let i = i as usize;
            if i != 0x25 {
                assert_eq!(disk.read_u8(0, i), 0xA5, "偏移 {} 被误改", i);
            }
        }
    }
}
