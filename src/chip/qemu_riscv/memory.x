/* QEMU RISC-V virt 机内存布局:
   DRAM @0x80000000(-bios none -kernel 加载全部 PT_LOAD 并直跳入口)。
   单一 32M 区:全 DRAM 可写——text/rodata/data/bss/heap/stack 同区。
   【曾踩坑】FLASH(0x80000000,4M)+RAM(0x80400000,16M) 两区拆分时,
   _sheap 落在 text 区末(0x800ca008),4MB 堆既越过 RAM 末尾又压过
   _stack_start(0x81000000)——分配器高地址分配踩坏栈/写未映射区,
   现场错乱直到野写 0x100000(sifive_test)触发整机复位。堆 1MB 的
   pingpong 从不越界所以从未暴露。单一区内一切连续,无此族问题。 */
MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 32M
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
