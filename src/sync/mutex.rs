//! 互斥锁，互斥锁只能持有锁的任务自己释放，二值信号量可以由另外一个任务释放
//! TODO ,可以先用临界区函数sync::free(||{$同步代码});

use crate::port::{Portable, Porting};
use bare_metal::CriticalSection;

/// 临界区保护
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    Porting::free(f)
}
