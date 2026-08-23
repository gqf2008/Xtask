/* QEMU RISC-V virt 机内存布局:
   DRAM @0x80000000(-bios none -kernel 加载全部 PT_LOAD 并直跳入口)。
   单一区:全 DRAM 可写——text/rodata/data/bss/heap/stack 同区,
   不做 FLASH/RAM 分离(分离会导致 .eh_frame 的 PC 相对重定位溢出 2GB)。 */
MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 16M
}

REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

/* 丢弃 .eh_frame:no_std + panic-halt 不展开栈;.L0 人格引用在
   --gc-sections 回收后落到 0 地址,与 0x80000000 基址差出 PC 相对域 */
SECTIONS {
  /DISCARD/ : { *(.eh_frame) *(.eh_frame_hdr) }
}
