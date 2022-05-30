pub mod broadcast;
pub mod mutex;
pub mod notify;
pub mod queue;
pub mod semaphore;
pub use mutex::free;

#[derive(Debug)]
pub enum Error {
    /// 信号量满了
    SemaphoreFull,
    /// 队列满了
    QueueFull,
}
