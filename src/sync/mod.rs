pub mod arc;

/// no-CAS 目标(thumbv6m/M0+)的 AtomicCell 兼容层:crossbeam 的 AtomicCell
/// 在无原子 CAS 的目标被 cfg 掉;此处用临界区保护 Cell 提供同款 API
/// (load/store/fetch_update——notify 的 blocker 槽专用,临界区可重入
/// 于 PRIMASK,嵌套安全)。
#[cfg(not(target_has_atomic = "ptr"))]
pub(crate) mod atomic_cell {
    pub(crate) struct AtomicCell<T>(core::cell::Cell<T>);

    impl<T: Copy> AtomicCell<T> {
        pub(crate) fn new(v: T) -> Self {
            AtomicCell(core::cell::Cell::new(v))
        }
        pub(crate) fn load(&self) -> T {
            crate::sync::free(|_| self.0.get())
        }
        pub(crate) fn store(&self, v: T) {
            crate::sync::free(|_| self.0.set(v))
        }
        /// CAS 语义(返回 Ok(旧值) 表示已从 old 换成 new)——由闭包裁决
        pub(crate) fn fetch_update<F: FnOnce(T) -> Option<T>>(
            &self,
            f: F,
        ) -> Result<T, ()> {
            crate::sync::free(|_| {
                let cur = self.0.get();
                match f(cur) {
                    Some(new) => {
                        self.0.set(new);
                        Ok(cur)
                    }
                    None => Err(()),
                }
            })
        }
    }
}
pub mod broadcast;
pub mod critical;
// crossbeam epoch 依赖原子 CAS——no-CAS 目标(thumbv6m)不可用,门控掉
#[cfg(target_has_atomic = "ptr")]
pub mod free_queue;
pub mod mutex;
pub mod notify;
pub mod queue;
pub mod semaphore;

pub use critical::free;

#[derive(Debug)]
pub enum Error {
    /// 信号量满了
    SemaphoreFull,
    /// 队列满了
    QueueFull,
}
