/* Cortex-R5F 链接脚本:无 runtime crate(R5 生态无 cortex-r-rt),自备。
   向量表在 OCM(.vectors),其余在 DDR。 */
ENTRY(_start)

SECTIONS
{
  .vectors : { KEEP(*(.vectors)) } > OCM
  .text : { *(.text .text.*) } > RAM
  .rodata : { *(.rodata .rodata.*) } > RAM
  .data : { *(.data .data.*) } > RAM
  .bss (NOLOAD) : { *(.bss .bss.*) } > RAM

  /* 堆起点 = bss 末(例程 init_heap(_sheap, size)) */
  _sheap = .;

  /* 栈:SVC 用满 DDR(8 对齐,0x01000000-8);IRQ 模式专用中断栈在其下
     (IRQ 入口压 64B 帧 + handler/调度器栈帧,4KB 富余)。
     【堆/栈分界】IRQ 栈区在 DDR 顶,堆绝不可越过 __heap_end 分配——
     否则任务栈/队列缓冲会分进中断栈区,与 IRQ 帧互相踩(曾实测:
     SP_irq 累积下掉 + 堆越过栈区,分配器元数据被压坏)。例程
     init_heap(start, __heap_end - start) 按此减堆 */
  __stack_top = ORIGIN(RAM) + LENGTH(RAM) - 8;
  __stack_irq = __stack_top - 0x1000;
  __heap_end = __stack_irq - 0x1000;   /* IRQ 栈下再留 4KB 余量 */

  /DISCARD/ : { *(.eh_frame) }
}
