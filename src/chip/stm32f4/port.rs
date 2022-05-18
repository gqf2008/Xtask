/// 系统节拍器中断
#[exception]
unsafe fn SysTick() {}

/// 软中断
#[exception]
unsafe fn PendSV() {}
