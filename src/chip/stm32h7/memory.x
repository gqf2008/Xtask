MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* STM32F401CCU6 */
  /* FLASH : ORIGIN = 0x08000000, LENGTH = 256K */
  /* RAM : ORIGIN = 0x20000000, LENGTH = 64K */
  /* STM32F427VIT6 */
  /* FLASH : ORIGIN = 0x08000000, LENGTH = 2048K */
  /* RAM : ORIGIN = 0x20000000, LENGTH = 192K */
  /* STM32H7B0VBT6 */
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K  
  RAM : ORIGIN = 0x20000000, LENGTH = 1184K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
