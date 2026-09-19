/* CH583(QingKe V4A)内存布局 —— 对齐官方 EVT `Ld/Link.ld`(SRC/Ld/Link.ld):
   FLASH @0x00000000,LENGTH 448K(⚠️ 基址是 0,不是 0x08000000;
   尾部 64K(0x70000-0x7FFFF)留给 BootLoader/ISP/配置区,故只用 448K);
   RAM   @0x20000000,LENGTH 32K。
   真机核对点:0x00000000 是否可写、是否存在 0x08000000 别名。

   前 0x18 字节是**启动头**(`.bootvec`,见 port.S):WCH ROM 从 0x0 取指,
   并按"向量表第 5 个字 = boot option 0xF3F9BDA9"识别有效用户程序(官方
   startup_CH583.S 的 `.vector` 段,注释 "boot option, can't modify";
   官方 Link.ld 里该字落在 flash 0x14)。故 `.text` 顶到 0x18 起(见下面
   `_stext` 赋值),启动头用下面的 SECTIONS 显式摆到 flash 0x0。 */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 448K
  RAM   : ORIGIN = 0x20000000, LENGTH = 32K
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
ASSERT(_bootvec == 0x00000000, "ERROR(ch583): 启动头 .bootvec 必须落在 flash 0x0");
ASSERT(_boot_magic == 0x00000014, "ERROR(ch583): boot option 0xF3F9BDA9 必须落在 flash 0x14");
