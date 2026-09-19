/* CH572(QingKe 无 A 扩展)内存布局 —— 对齐官方 EVT `Ld/Link.ld`(SRC/Ld/Link.ld):
   FLASH @0x00000000,LENGTH 240K(芯片 CODE 区,另有 BOOT 8K + INFO 8K 在尾部);
   RAM   @0x20000000,LENGTH 12K(⚠️ 极窄,任务栈账见 examples/multitask_ch572.rs)。
   真机核对点:0x00000000 是否可写、是否存在别名。

   前 0x18 字节是**启动头**(`.bootvec`,见 port.S):WCH ROM 从 0x0 取指,并按
   "向量表第 5 个字 = boot option 0xF3F9BDA9"识别有效用户程序。CH572 官方
   `startup_CH572.S`/`Link.ld` 的实测结果与 CH583 一致:该字落在 flash 0x14
   (官方 `.highcode` 里的 `. = ALIGN(1024)` 因 RAM 基址本就 1024 对齐而无效果,
   已用官方 Link.ld 现场链接实测)。故 `.text` 顶到 0x18 起(见下面 `_stext` 赋值),
   启动头用 SECTIONS 显式摆到 flash 0x0。 */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 240K
  RAM   : ORIGIN = 0x20000000, LENGTH = 12K
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

SECTIONS
{
  /* 启动头:flash 0x00..0x17(ROM 认的启动头字节,内容见 port.S) */
  .bootvec 0x00000000 :
  {
    KEEP(*(.bootvec));
  } > FLASH
}

/* riscv-rt 的 link.x 用 `PROVIDE(_stext = ORIGIN(REGION_TEXT))` 兜底;本文件
   先于 link.x 处理,这里先定义 `_stext`,把 `.text` 顶到启动头之后(0x18),
   避免与 `.bootvec` 抢 0x0(PROVIDE 不会覆盖已定义符号)。 */
_stext = 0x00000018;

/* 链接期守卫:启动头与 boot option 必须落在 ROM 认的地址上(符号见 port.S) */
ASSERT(_bootvec == 0x00000000, "ERROR(ch572): 启动头 .bootvec 必须落在 flash 0x0");
ASSERT(_boot_magic == 0x00000014, "ERROR(ch572): boot option 0xF3F9BDA9 必须落在 flash 0x14");
