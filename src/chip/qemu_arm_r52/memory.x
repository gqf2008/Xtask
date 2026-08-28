/* QEMU xlnx-zcu102 Cortex-R5F 内存布局:
   R5 复位向量 = 0xFFFF0000(SCTLR.V=1, reset-hivecs)→ OCM bank3。
   向量表(32B,8 个绝对跳)在 OCM;其余全在 DDR @0x0010_0000
   (-device loader 按 ELF 段加载)。

   【A53 干扰】镜像起点必须避开 0x00000000:A53 从核复位后从 0x0
   读取执行(reset 向量),若 0x0 是我们的代码,A53 按 aarch64 乱解
   ARM 指令并乱写 RAM——实测破坏 bss 里的分配器元数据(首个任务栈
   分配到 bss 区 → save_context 写坏 free 链表 → capacity overflow)。
   基址移到 1M 处后 0x0 冷启动为全零(aarch64 UNALLOCATED 编码 →
   A53 异常停泊,不再乱写)。DDR 探针实测可执行,见 /tmp/r5probe。

   【曾踩坑】整镜像放 OCM(64KB)时,__stack_top = 0xFFFFFFFF+1 溢出
   32 位——栈顶必须 `- 8` 且 8 对齐(AAPCS);DDR 布局无此问题。 */
MEMORY
{
  OCM : ORIGIN = 0xFFFF0000, LENGTH = 0x10000
  RAM : ORIGIN = 0x00100000, LENGTH = 16M
}
